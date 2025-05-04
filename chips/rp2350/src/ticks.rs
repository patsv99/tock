// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.


// use kernel::utilities::registers::interfaces::{};
use kernel::utilities::registers::{
    register_bitfields, register_structs, ReadWrite
};
use kernel::utilities::StaticRef;



register_structs! {
    TICKSRegisters {
  (0x000 => proc0_ctrl: ReadWrite<u32, PROC0_CTRL::Register>),
(0x4 => proc0_cycles: ReadWrite<u32,PROC0_CYCLES::Register>),
(0x8 => proc0_count: ReadWrite<u32,PROC0_COUNT::Register>),
(0xc => proc1_ctrl: ReadWrite<u32, PROC1_CTRL::Register>),
(0x10 => proc1_cycles: ReadWrite<u32,PROC1_CYCLES::Register>),
(0x14 => proc1_count: ReadWrite<u32,PROC1_COUNT::Register>),
(0x18 => timer0_ctrl: ReadWrite<u32, TIMER0_CTRL::Register>),
(0x1c => timer0_cycles: ReadWrite<u32,TIMER0_CYCLES::Register>),
(0x20 => timer0_count: ReadWrite<u32,TIMER0_COUNT::Register>),
(0x24 => timer1_ctrl: ReadWrite<u32, TIMER1_CTRL::Register>),
(0x28 => timer1_cycles: ReadWrite<u32,TIMER1_CYCLES::Register>),
(0x2c => timer1_count: ReadWrite<u32,TIMER1_COUNT::Register>),
(0x30 => watchdog_ctrl: ReadWrite<u32, WATCHDOG_CTRL::Register>),
(0x34 => watchdog_cycles: ReadWrite<u32,WATCHDOG_CYCLES::Register>),
(0x38 => watchdog_count: ReadWrite<u32,WATCHDOG_COUNT::Register>),
(0x3c => riscv_ctrl: ReadWrite<u32, RISCV_CTRL::Register>),
(0x40 => riscv_cycles: ReadWrite<u32,RISCV_CYCLES::Register>),
(0x44 => riscv_count: ReadWrite<u32,RISCV_COUNT::Register>),
(0x48 => @END),
}
}
register_bitfields![u32,
    PROC0_CTRL [
        ENABLE OFFSET(0) NUMBITS(1) [],
        RUNNING OFFSET(1) NUMBITS(1) [],
    ],
    PROC0_CYCLES [
        CYCLES OFFSET(0) NUMBITS(9) [],
    ],
    PROC0_COUNT [
        COUNT OFFSET(0) NUMBITS(9) [],
    ],
    PROC1_CTRL [
        ENABLE OFFSET(0) NUMBITS(1) [],
        RUNNING OFFSET(1) NUMBITS(1) [],
    ],
    PROC1_CYCLES [
        CYCLES OFFSET(0) NUMBITS(9) [],
    ],
    PROC1_COUNT [
        COUNT OFFSET(0) NUMBITS(9) [],
    ],
    TIMER0_CTRL [
        ENABLE OFFSET(0) NUMBITS(1) [],
        RUNNING OFFSET(1) NUMBITS(1) [],
    ],
    TIMER0_CYCLES [
        CYCLES OFFSET(0) NUMBITS(9) [],
    ],
    TIMER0_COUNT [
        COUNT OFFSET(0) NUMBITS(9) [],
    ],
    TIMER1_CTRL [
        ENABLE OFFSET(0) NUMBITS(1) [],
        RUNNING OFFSET(1) NUMBITS(1) [],
    ],
    TIMER1_CYCLES [
        CYCLES OFFSET(0) NUMBITS(9) [],
    ],
    TIMER1_COUNT [
        COUNT OFFSET(0) NUMBITS(9) [],
    ],
    WATCHDOG_CTRL [
        ENABLE OFFSET(0) NUMBITS(1) [],
        RUNNING OFFSET(1) NUMBITS(1) [],
    ],
    WATCHDOG_CYCLES [
        CYCLES OFFSET(0) NUMBITS(9) [],
    ],
    WATCHDOG_COUNT [
        COUNT OFFSET(0) NUMBITS(9) [],
    ],
    RISCV_CTRL [
        ENABLE OFFSET(0) NUMBITS(1) [],
        RUNNING OFFSET(1) NUMBITS(1) [],
    ],
    RISCV_CYCLES [
        CYCLES OFFSET(0) NUMBITS(9) [],
    ],
    RISCV_COUNT [
        COUNT OFFSET(0) NUMBITS(9) [],
    ],

    ];

const TICKS_BASE: StaticRef<TICKSRegisters> =
    unsafe { StaticRef::new(0x40108000 as *const TICKSRegisters) };

pub struct TICKS {
    registers : StaticRef<TICKSRegisters>
}
impl TICKS {
    #[inline(never)]
    pub const fn new() -> TICKS {
        TICKS {
            registers: TICKS_BASE,
        }
    }
}
