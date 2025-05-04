// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.


// use kernel::debug;
// use kernel::hil;
// use kernel::utilities::cells::OptionalCell;
// use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{
    register_bitfields, register_structs, ReadWrite,
};
use kernel::utilities::StaticRef;
// use kernel::ErrorCode;


register_structs! {
    PSMRegisters {
 // Force block out of reset (i.e. power it on)
  (0x000 => frce_on: ReadWrite<u32, FRCE_ON::Register>),
 // Force into reset (i.e. power it off)
(0x4 => frce_off: ReadWrite<u32,FRCE_OFF::Register>),
// Set to 1 if the watchdog should reset this
(0x8 => wdsel: ReadWrite<u32,WDSEL::Register>),
// Is the subsystem ready
(0xc => done: ReadWrite<u32,DONE::Register>),
(0x010 => @END),
    }
}
register_bitfields![u32,
    FRCE_ON [
        proc1 OFFSET(24) NUMBITS(1) [],
        proc0 OFFSET(23) NUMBITS(1) [],
        accessctrl OFFSET(22) NUMBITS(1) [],
        sio OFFSET(21) NUMBITS(1) [],
        xip OFFSET(20) NUMBITS(1) [],
        sram9 OFFSET(19) NUMBITS(1) [],
        sram8 OFFSET(18) NUMBITS(1) [],
        sram7 OFFSET(17) NUMBITS(1) [],
        sram6 OFFSET(16) NUMBITS(1) [],
        sram5 OFFSET(15) NUMBITS(1) [],
        sram4 OFFSET(14) NUMBITS(1) [],
        sram3 OFFSET(13) NUMBITS(1) [],
        sram2 OFFSET(12) NUMBITS(1) [],
        sram1 OFFSET(11) NUMBITS(1) [],
        sram0 OFFSET(10) NUMBITS(1) [],
        bootram OFFSET(9) NUMBITS(1) [],
        rom OFFSET(8) NUMBITS(1) [],
        busfabric OFFSET(7) NUMBITS(1) [],
        psm_ready OFFSET(6) NUMBITS(1) [],
        clocks OFFSET(5) NUMBITS(1) [],
        resets OFFSET(4) NUMBITS(1) [],
        xosc OFFSET(3) NUMBITS(1) [],
        rosc OFFSET(2) NUMBITS(1) [],
        otp OFFSET(1) NUMBITS(1) [],
        proc_cold OFFSET(0) NUMBITS(1) [],
    ],
    FRCE_OFF [
        proc1 OFFSET(24) NUMBITS(1) [],
        proc0 OFFSET(23) NUMBITS(1) [],
        accessctrl OFFSET(22) NUMBITS(1) [],
        sio OFFSET(21) NUMBITS(1) [],
        xip OFFSET(20) NUMBITS(1) [],
        sram9 OFFSET(19) NUMBITS(1) [],
        sram8 OFFSET(18) NUMBITS(1) [],
        sram7 OFFSET(17) NUMBITS(1) [],
        sram6 OFFSET(16) NUMBITS(1) [],
        sram5 OFFSET(15) NUMBITS(1) [],
        sram4 OFFSET(14) NUMBITS(1) [],
        sram3 OFFSET(13) NUMBITS(1) [],
        sram2 OFFSET(12) NUMBITS(1) [],
        sram1 OFFSET(11) NUMBITS(1) [],
        sram0 OFFSET(10) NUMBITS(1) [],
        bootram OFFSET(9) NUMBITS(1) [],
        rom OFFSET(8) NUMBITS(1) [],
        busfabric OFFSET(7) NUMBITS(1) [],
        psm_ready OFFSET(6) NUMBITS(1) [],
        clocks OFFSET(5) NUMBITS(1) [],
        resets OFFSET(4) NUMBITS(1) [],
        xosc OFFSET(3) NUMBITS(1) [],
        rosc OFFSET(2) NUMBITS(1) [],
        otp OFFSET(1) NUMBITS(1) [],
        proc_cold OFFSET(0) NUMBITS(1) [],
    ],
    WDSEL [
        proc1 OFFSET(24) NUMBITS(1) [],
        proc0 OFFSET(23) NUMBITS(1) [],
        accessctrl OFFSET(22) NUMBITS(1) [],
        sio OFFSET(21) NUMBITS(1) [],
        xip OFFSET(20) NUMBITS(1) [],
        sram9 OFFSET(19) NUMBITS(1) [],
        sram8 OFFSET(18) NUMBITS(1) [],
        sram7 OFFSET(17) NUMBITS(1) [],
        sram6 OFFSET(16) NUMBITS(1) [],
        sram5 OFFSET(15) NUMBITS(1) [],
        sram4 OFFSET(14) NUMBITS(1) [],
        sram3 OFFSET(13) NUMBITS(1) [],
        sram2 OFFSET(12) NUMBITS(1) [],
        sram1 OFFSET(11) NUMBITS(1) [],
        sram0 OFFSET(10) NUMBITS(1) [],
        bootram OFFSET(9) NUMBITS(1) [],
        rom OFFSET(8) NUMBITS(1) [],
        busfabric OFFSET(7) NUMBITS(1) [],
        psm_ready OFFSET(6) NUMBITS(1) [],
        clocks OFFSET(5) NUMBITS(1) [],
        resets OFFSET(4) NUMBITS(1) [],
        xosc OFFSET(3) NUMBITS(1) [],
        rosc OFFSET(2) NUMBITS(1) [],
        otp OFFSET(1) NUMBITS(1) [],
        proc_cold OFFSET(0) NUMBITS(1) [],
    ],
    DONE [
        proc1 OFFSET(24) NUMBITS(1) [],
        proc0 OFFSET(23) NUMBITS(1) [],
        accessctrl OFFSET(22) NUMBITS(1) [],
        sio OFFSET(21) NUMBITS(1) [],
        xip OFFSET(20) NUMBITS(1) [],
        sram9 OFFSET(19) NUMBITS(1) [],
        sram8 OFFSET(18) NUMBITS(1) [],
        sram7 OFFSET(17) NUMBITS(1) [],
        sram6 OFFSET(16) NUMBITS(1) [],
        sram5 OFFSET(15) NUMBITS(1) [],
        sram4 OFFSET(14) NUMBITS(1) [],
        sram3 OFFSET(13) NUMBITS(1) [],
        sram2 OFFSET(12) NUMBITS(1) [],
        sram1 OFFSET(11) NUMBITS(1) [],
        sram0 OFFSET(10) NUMBITS(1) [],
        bootram OFFSET(9) NUMBITS(1) [],
        rom OFFSET(8) NUMBITS(1) [],
        busfabric OFFSET(7) NUMBITS(1) [],
        psm_ready OFFSET(6) NUMBITS(1) [],
        clocks OFFSET(5) NUMBITS(1) [],
        resets OFFSET(4) NUMBITS(1) [],
        xosc OFFSET(3) NUMBITS(1) [],
        rosc OFFSET(2) NUMBITS(1) [],
        otp OFFSET(1) NUMBITS(1) [],
        proc_cold OFFSET(0) NUMBITS(1) [],
    ]
];

const PSM_BASE: StaticRef<PSMRegisters> =
    unsafe { StaticRef::new(0x40018000 as *const PSMRegisters) };

pub struct PSM {
    registers : StaticRef<PSMRegisters>
}
impl PSM {
    #[inline(never)]
    pub const fn new() -> PSM {
        PSM {
            registers: PSM_BASE,
        }
    }
}
