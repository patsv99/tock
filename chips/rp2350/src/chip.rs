// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

//! Chip trait setup.

use core::fmt::Write;
use kernel::platform::chip::Chip;
use kernel::platform::chip::InterruptService;

use crate::adc;
use crate::clocks::Clocks;
use crate::gpio::{RPGpio, RPPins, SIO};
use crate::i2c;
use crate::interrupts;
use crate::pio::Pio;
use crate::pwm;
use crate::resets::Resets;
use crate::rtc;
use crate::spi;
use crate::sysinfo;
use crate::timer::RPTimer;
use crate::uart::Uart;
use crate::usb;
use crate::watchdog::Watchdog;
use crate::xosc::Xosc;
use cortexm33::{interrupt_mask, CortexM33, CortexMVariant};

#[repr(u8)]
pub enum Processor {
    Processor0 = 0,
    Processor1 = 1,
}

pub struct Rp2350<'a, I: InterruptService + 'a> {
    mpu: cortexm33::mpu::MPU,
    userspace_kernel_boundary: cortexm33::syscall::SysCall,
    interrupt_service: &'a I,
    sio: &'a SIO,
    processor0_interrupt_mask: (u128, u128),
    processor1_interrupt_mask: (u128, u128),
}

impl<'a, I: InterruptService> Rp2350<'a, I> {
    pub unsafe fn new(interrupt_service: &'a I, sio: &'a SIO) -> Self {
        Self {
            mpu: cortexm33::mpu::MPU::new(),
            userspace_kernel_boundary: cortexm33::syscall::SysCall::new(),
            interrupt_service,
            sio,
            processor0_interrupt_mask: interrupt_mask!(interrupts::SIO_IRQ_PROC1),
            processor1_interrupt_mask: interrupt_mask!(interrupts::SIO_IRQ_PROC0),
        }
    }
}

impl<I: InterruptService> Chip for Rp2350<'_, I> {
    type MPU = cortexm33::mpu::MPU;
    type UserspaceKernelBoundary = cortexm33::syscall::SysCall;

    fn service_pending_interrupts(&self) {
        unsafe {
            let mask = match self.sio.get_processor() {
                Processor::Processor0 => self.processor0_interrupt_mask,
                Processor::Processor1 => self.processor1_interrupt_mask,
            };
            loop {
                if let Some(interrupt) = cortexm33::nvic::next_pending_with_mask(mask) {
                    // ignore SIO_IRQ_PROC1 as it is intended for processor 1
                    // not able to unset its pending status
                    // probably only processor 1 can unset the pending by reading the fifo
                    if !self.interrupt_service.service_interrupt(interrupt) {
                        panic!("unhandled interrupt {}", interrupt);
                    }
                    let n = cortexm33::nvic::Nvic::new(interrupt);
                    n.clear_pending();
                    n.enable();
                } else {
                    break;
                }
            }
        }
    }

    fn has_pending_interrupts(&self) -> bool {
        // ignore SIO_IRQ_PROC1 as it is intended for processor 1
        // not able to unset its pending status
        // probably only processor 1 can unset the pending by reading the fifo
        let mask = match self.sio.get_processor() {
            Processor::Processor0 => self.processor0_interrupt_mask,
            Processor::Processor1 => self.processor1_interrupt_mask,
        };
        unsafe { cortexm33::nvic::has_pending_with_mask(mask) }
    }

    fn mpu(&self) -> &Self::MPU {
        &self.mpu
    }

    fn userspace_kernel_boundary(&self) -> &Self::UserspaceKernelBoundary {
        &self.userspace_kernel_boundary
    }

    fn sleep(&self) {
        unsafe {
            cortexm33::support::wfi();
        }
    }

    unsafe fn atomic<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        cortexm33::support::atomic(f)
    }

    unsafe fn print_state(&self, writer: &mut dyn Write) {
        CortexM33::print_cortexm_state(writer);
    }
}

pub struct Rp2350DefaultPeripherals<'a> {
    pub adc: adc::Adc<'a>,
    pub clocks: Clocks,
    pub i2c0: i2c::I2c<'a, 'a>,
    pub pins: RPPins<'a>,
    pub pio0: Pio,
    pub pio1: Pio,
    pub pwm: pwm::Pwm<'a>,
    pub resets: Resets,
    pub sio: SIO,
    pub spi0: spi::Spi<'a>,
    pub sysinfo: sysinfo::SysInfo,
    pub timer: RPTimer<'a>,
    pub uart0: Uart<'a>,
    pub uart1: Uart<'a>,
    pub usb: usb::UsbCtrl<'a>,
    pub watchdog: Watchdog<'a>,
    pub xosc: Xosc,
    pub rtc: rtc::Rtc<'a>,
}

impl Rp2350DefaultPeripherals<'_> {
    pub fn new() -> Self {
        Self {
            adc: adc::Adc::new(),
            clocks: Clocks::new(),
            i2c0: i2c::I2c::new_i2c0(),
            pins: RPPins::new(),
            pio0: Pio::new_pio0(),
            pio1: Pio::new_pio1(),
            pwm: pwm::Pwm::new(),
            resets: Resets::new(),
            sio: SIO::new(),
            spi0: spi::Spi::new_spi0(),
            sysinfo: sysinfo::SysInfo::new(),
            timer: RPTimer::new(),
            uart0: Uart::new_uart0(),
            uart1: Uart::new_uart1(),
            usb: usb::UsbCtrl::new(),
            watchdog: Watchdog::new(),
            xosc: Xosc::new(),
            rtc: rtc::Rtc::new(),
        }
    }

    pub fn resolve_dependencies(&'static self) {
        self.pwm.set_clocks(&self.clocks);
        self.watchdog.resolve_dependencies(&self.resets);
        self.spi0.set_clocks(&self.clocks);
        self.uart0.set_clocks(&self.clocks);
        kernel::deferred_call::DeferredCallClient::register(&self.uart0);
        kernel::deferred_call::DeferredCallClient::register(&self.uart1);
        kernel::deferred_call::DeferredCallClient::register(&self.rtc);
        self.i2c0.resolve_dependencies(&self.clocks, &self.resets);
        self.usb.set_gpio(self.pins.get_pin(RPGpio::GPIO15));
        self.rtc.set_clocks(&self.clocks);
    }
}

impl InterruptService for Rp2350DefaultPeripherals<'_> {
    unsafe fn service_interrupt(&self, interrupt: u32) -> bool {
        match interrupt {
            interrupts::PIO0_IRQ_0 => {
                self.pio0.handle_interrupt();
                true
            }
            interrupts::TIMER_IRQ_0 => {
                self.timer.handle_interrupt();
                true
            }
            interrupts::SIO_IRQ_PROC0 => {
                self.sio.handle_proc_interrupt(Processor::Processor0);
                true
            }
            interrupts::SIO_IRQ_PROC1 => {
                self.sio.handle_proc_interrupt(Processor::Processor1);
                true
            }
            interrupts::SPI0_IRQ => {
                self.spi0.handle_interrupt();
                true
            }
            interrupts::UART0_IRQ => {
                self.uart0.handle_interrupt();
                true
            }
            interrupts::ADC_IRQ_FIFO => {
                self.adc.handle_interrupt();
                true
            }
            interrupts::USBCTRL_IRQ => {
                self.usb.handle_interrupt();
                true
            }
            interrupts::IO_IRQ_BANK0 => {
                self.pins.handle_interrupt();
                true
            }

            interrupts::I2C0_IRQ => {
                self.i2c0.handle_interrupt();
                true
            }
            interrupts::PWM_IRQ_WRAP => {
                // As the PWM HIL doesn't provide any support for interrupts, they are
                // simply ignored.
                //
                // Note that PWM interrupts are raised only during unit tests.
                true
            }
            _ => false,
        }
    }
}
