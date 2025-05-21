// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.
#![no_std]
// GPIO has many register definitions in `register_structs()!`
// and requires a deeper recursion limit than the default to fully expand.
#![recursion_limit = "256"]

pub mod adc;
pub mod chip;
pub mod clocks;
mod deferred_calls;
pub mod gpio;
pub mod i2c;
pub mod interrupts;
pub mod pio;
pub mod pio_pwm;
pub mod pio_spi;
pub mod psm;
pub mod pwm;
pub mod resets;
pub mod rosc;

pub mod spi;
pub mod syscfg;
pub mod sysinfo;
pub mod test;
pub mod ticks;
pub mod timer;
pub mod uart;
pub mod usb;
pub mod watchdog;
pub mod xosc;

use cortexm33::{initialize_ram_jump_to_main, unhandled_interrupt, CortexM33, CortexMVariant};

extern "C" {
    // _estack is not really a function, but it makes the types work
    // You should never actually invoke it!!
    fn _estack();
}

#[cfg_attr(
    all(target_arch = "arm", target_os = "none"),
    link_section = ".vectors"
)]
// used Ensures that the symbol is kept until the final binary
#[cfg_attr(all(target_arch = "arm", target_os = "none"), used)]
pub static BASE_VECTORS: [unsafe extern "C" fn(); 16] = [
    _estack,                       // Stackptr    Exception no
    initialize_ram_jump_to_main,   // Reset       1
    unhandled_interrupt,           // NMI         2
    CortexM33::HARD_FAULT_HANDLER, // Hard Fault  3
    unhandled_interrupt,           // MemManage   4
    unhandled_interrupt,           // BusFault    5
    unhandled_interrupt,           // UsageFault  6
    unhandled_interrupt,           //             7
    unhandled_interrupt,           //             8
    unhandled_interrupt,           //             9
    unhandled_interrupt,           //            10
    CortexM33::SVC_HANDLER,        // SVC        11
    unhandled_interrupt,           // DebugMon   12
    unhandled_interrupt,           //            13
    unhandled_interrupt,           // PendSV     14
    CortexM33::SYSTICK_HANDLER,    // SysTick    15
];

// RP2350 has total of xx interrupts, but the SDK declares 32 as 26 - 32 might be manually handled
// used Ensures that the symbol is kept until the final binary
#[cfg_attr(
    all(target_arch = "arm", target_os = "none"),
    link_section = ".vectors"
)]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), used)]
pub static IRQS: [unsafe extern "C" fn(); 52] = [
CortexM33::GENERIC_ISR,  // TIMER0_IRQ_0
CortexM33::GENERIC_ISR,  // TIMER0_IRQ_1
CortexM33::GENERIC_ISR,  // TIMER0_IRQ_2
CortexM33::GENERIC_ISR,  // TIMER0_IRQ_3
CortexM33::GENERIC_ISR,  // TIMER1_IRQ_0
CortexM33::GENERIC_ISR,  // TIMER1_IRQ_1
CortexM33::GENERIC_ISR,  // TIMER1_IRQ_2
CortexM33::GENERIC_ISR,  // TIMER1_IRQ_3
    
CortexM33::GENERIC_ISR,  // PWM_IRQ_WRAP_0
CortexM33::GENERIC_ISR,  // PWM_IRQ_WRAP_1
    
CortexM33::GENERIC_ISR,  // DMA_IRQ_0
CortexM33::GENERIC_ISR,  // DMA_IRQ_1
CortexM33::GENERIC_ISR,  // DMA_IRQ_2
CortexM33::GENERIC_ISR,  // DMA_IRQ_3
    
CortexM33::GENERIC_ISR,  // USBCTRL_IRQ
    
CortexM33::GENERIC_ISR,  // PIO0_IRQ_0
CortexM33::GENERIC_ISR,  // PIO0_IRQ_1
CortexM33::GENERIC_ISR,  // PIO1_IRQ_0
CortexM33::GENERIC_ISR,  // PIO1_IRQ_1
CortexM33::GENERIC_ISR,  // PIO2_IRQ_0
CortexM33::GENERIC_ISR,  // PIO2_IRQ_1
    
CortexM33::GENERIC_ISR,  // IO_IRQ_BANK0
CortexM33::GENERIC_ISR,  // IO_IRQ_BANK0_NS
    
CortexM33::GENERIC_ISR,  // SIO_IRQ_QSPI
CortexM33::GENERIC_ISR,  // SIO_IRQ_QSPI_NS
    
CortexM33::GENERIC_ISR,  // SIO_IRQ_FIFO
CortexM33::GENERIC_ISR,  // SIO_IRQ_BELL
CortexM33::GENERIC_ISR,  // SIO_IRQ_FIFO_NS
CortexM33::GENERIC_ISR,  // SIO_IRQ_BELL_NS
CortexM33::GENERIC_ISR,  // SIO_IRQ_MTIMECMP
    
CortexM33::GENERIC_ISR,  // CLOCKS_IRQ
    
CortexM33::GENERIC_ISR,  // SPI0_IRQ
CortexM33::GENERIC_ISR,  // SPI1_IRQ
    
CortexM33::GENERIC_ISR,  // UART0_IRQ
CortexM33::GENERIC_ISR,  // UART1_IRQ
    
CortexM33::GENERIC_ISR,  // ADC_IRQ_FIFO
    
CortexM33::GENERIC_ISR,  // I2C0_IRQ
CortexM33::GENERIC_ISR,  // I2C1_IRQ
    
CortexM33::GENERIC_ISR,  // OTP_IRQ
    
CortexM33::GENERIC_ISR,  // TRNG_IRQ
    
CortexM33::GENERIC_ISR,  // PROC0_IRQ_CTI
CortexM33::GENERIC_ISR,  // PROC1_IRQ_CTI
    
CortexM33::GENERIC_ISR,  // PLL_SYS_IRQ
CortexM33::GENERIC_ISR,  // PLL_SYS_USB
CortexM33::GENERIC_ISR,  // POWMAN_IRQ_POW
CortexM33::GENERIC_ISR,  // POWMAN_IRQ_TIMER
CortexM33::GENERIC_ISR,  // 46
CortexM33::GENERIC_ISR,  // 47
CortexM33::GENERIC_ISR,  // 48
CortexM33::GENERIC_ISR,  // 49
CortexM33::GENERIC_ISR,  // 50
CortexM33::GENERIC_ISR,  // 51
];
    

extern "C" {
    static mut _szero: usize;
    static mut _ezero: usize;
    static mut _etext: usize;
    static mut _srelocate: usize;
    static mut _erelocate: usize;
}
#[inline(never)]
pub unsafe fn init() {
    cortexm33::nvic::disable_all();
    cortexm33::nvic::clear_all_pending();
    cortexm33::scb::set_vector_table_offset(core::ptr::addr_of!(BASE_VECTORS) as *const ());

    let sio = gpio::SIO::new();
    let processor = sio.get_processor();
    match processor {
        chip::Processor::Processor0 => {}
        _ => panic!(
            "Kernel should run only using processor 0 (now processor {})",
            processor as u8
        ),
    }
}
