// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.
#![recursion_limit = "256"]
#![no_std]

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
#[cfg_attr(all(target_arch = "arm", target_os = "none"), link_section = ".irqs")]
// used Ensures that the symbol is kept until the final binary
#[cfg_attr(all(target_arch = "arm", target_os = "none"), used)]
pub static IRQS: [unsafe extern "C" fn(); 52] = [
    CortexM33::GENERIC_ISR, // TIMER0 (0)  // TODO These are wrong sources look in interrupts.rs for correct
    CortexM33::GENERIC_ISR, // TIMER1 (1)
    CortexM33::GENERIC_ISR, // TIMER2 (2)
    CortexM33::GENERIC_ISR, // TIMER3 (3)
    CortexM33::GENERIC_ISR, // PWM WRAP (4)
    CortexM33::GENERIC_ISR, // USB (5)
    CortexM33::GENERIC_ISR, // XIP (6)
    CortexM33::GENERIC_ISR, // PIO0 INT0  (7)
    CortexM33::GENERIC_ISR, // PIO0 INT1 (8)
    CortexM33::GENERIC_ISR, // PIO1 INT0 (9)
    CortexM33::GENERIC_ISR, // PIO1 INT1 (10)
    CortexM33::GENERIC_ISR, // DMA0 (11)
    CortexM33::GENERIC_ISR, // DMA1 (12)
    CortexM33::GENERIC_ISR, // IO BANK 0 (13)
    CortexM33::GENERIC_ISR, // IO QSPI (14)
    CortexM33::GENERIC_ISR, // SIO PROC 0 (15)
    CortexM33::GENERIC_ISR, // SIO PROC 1 (16)
    CortexM33::GENERIC_ISR, // CLOCKS (17)
    CortexM33::GENERIC_ISR, // SPI 0 (18)
    CortexM33::GENERIC_ISR, // SPI 1 (19)
    CortexM33::GENERIC_ISR, // UART 0 (20)
    CortexM33::GENERIC_ISR, // UART 1 (21)
    CortexM33::GENERIC_ISR, // ADC FIFO (22)
    CortexM33::GENERIC_ISR, // I2C 0 (23)
    CortexM33::GENERIC_ISR, // I2C 1 (24)
    CortexM33::GENERIC_ISR, // ISR (25)
    CortexM33::GENERIC_ISR, // ISR (26)
    CortexM33::GENERIC_ISR, // ISR (27)
    CortexM33::GENERIC_ISR, // ISR (28)
    CortexM33::GENERIC_ISR, // ISR (29)
    CortexM33::GENERIC_ISR, // ISR (30)
    CortexM33::GENERIC_ISR, // ISR (31)
    CortexM33::GENERIC_ISR, // ISR (32)
    CortexM33::GENERIC_ISR, // ISR (33)
    CortexM33::GENERIC_ISR, // ISR (34)
    CortexM33::GENERIC_ISR, // ISR (35)
    CortexM33::GENERIC_ISR, // ISR (36)
    CortexM33::GENERIC_ISR, // ISR (37)
    CortexM33::GENERIC_ISR, // ISR (38)
    CortexM33::GENERIC_ISR, // ISR (39)
    CortexM33::GENERIC_ISR, // ISR (40)
    CortexM33::GENERIC_ISR, // ISR (41)
    CortexM33::GENERIC_ISR, // ISR (42)
    CortexM33::GENERIC_ISR, // ISR (43)
    CortexM33::GENERIC_ISR, // ISR (44)
    CortexM33::GENERIC_ISR, // ISR (45)
    unhandled_interrupt,    // ISR (46)
    unhandled_interrupt,    // ISR (47)
    unhandled_interrupt,    // ISR (48)
    unhandled_interrupt,    // ISR (49)
    unhandled_interrupt,    // ISR (50)
    unhandled_interrupt,    // ISR (51)
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
