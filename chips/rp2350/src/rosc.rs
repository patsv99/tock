// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

// use kernel::utilities::registers::interfaces::{};
use kernel::utilities::registers::{register_bitfields, register_structs, ReadWrite};
use kernel::utilities::StaticRef;

register_structs! {
    ROSCRegisters {
        (0x000 => ctrl: ReadWrite<u32,ROSC_CTRL::Register>),
        (0x004 => freqa: ReadWrite<u32,ROSC_FREQA::Register>),
        (0x008 => freqb: ReadWrite<u32,ROSC_FREQB::Register>),
        (0x00c => random: ReadWrite<u32,ROSC_RANDOM::Register>),
        (0x010 => dormant: ReadWrite<u32,ROSC_DORMANT::Register>),
        (0x014 => div: ReadWrite<u32,ROSC_DIV::Register>),
        (0x018 => phase: ReadWrite<u32,ROSC_PHASE::Register>),
        (0x01c => status: ReadWrite<u32,ROSC_STATUS::Register>),
        (0x020 => randombit: ReadWrite<u32,ROSC_RANDOMBIT::Register>),
        (0x024 => count: ReadWrite<u32,ROSC_COUNT::Register>),
        (0x28 => @END),
    }
    }
    

register_bitfields![u32,
ROSC_CTRL [
    ENABLE OFFSET(12) NUMBITS(12) [ DISABLE = 0xd1e,ENABLE = 0xfab],
    FREQ_RANGE OFFSET(0) NUMBITS(12) [
        LOW = 0xfa4,
        MEDIUM = 0xfa5,
        HIGH = 0xfa7,
        TOOHIGH = 0xfa6
    ]
],
ROSC_FREQA [
    PASSWD OFFSET(16) NUMBITS(16) [],
    DS3 OFFSET(12) NUMBITS(3) [],
    DS2 OFFSET(8) NUMBITS(3) [],
    DS1 OFFSET(4) NUMBITS(3) [],
    DS0_RANDOM OFFSET(3) NUMBITS(1) [],
    DS0 OFFSET(0) NUMBITS(3) [],
],
ROSC_FREQB [
    PASSWD OFFSET(16) NUMBITS(16) [],
    DS7 OFFSET(12) NUMBITS(3) [],
    DS6 OFFSET(8) NUMBITS(3) [],
    DS5 OFFSET(4) NUMBITS(3) [],
    DS4 OFFSET(0) NUMBITS(3) [],
],  
ROSC_RANDOM [
    SEED OFFSET(0) NUMBITS(32) [],
],
ROSC_DORMANT [
    VALUE OFFSET(0) NUMBITS(32) [DORMANT=0x636f6d61,WAKE=0x77616b65],
],
ROSC_DIV [
    VALUE OFFSET(0) NUMBITS(16) [],
],
ROSC_PHASE [
    PASSWD OFFSET(4) NUMBITS(8) [PASSWD_SET = 0xaaa,SHIFT_0 = 1],
    ENABLE OFFSET(3) NUMBITS(1) [],
    FLIP OFFSET(2) NUMBITS(1) [],
    SHIFT OFFSET(0) NUMBITS(1) [],
],
ROSC_STATUS [
    STABLE OFFSET(31) NUMBITS(1) [],
    BADWRITE OFFSET(24) NUMBITS(1) [],
    DIV_RUNNING OFFSET(16) NUMBITS(1) [],
    ENABLE OFFSET(12) NUMBITS(1) [],
],
ROSC_RANDOMBIT [
    VALUE OFFSET(0) NUMBITS(1) [],
],
ROSC_COUNT [
COUNTER  OFFSET(0) NUMBITS(16) [],
]
];



const ROSC_BASE: StaticRef<ROSCRegisters> =
unsafe { StaticRef::new(0x400e8000 as *const ROSCRegisters) };

pub struct Rosc {
    registers: StaticRef<ROSCRegisters>,
}

impl Rosc {
    #[inline(never)]
    pub const fn new() -> Rosc {
        Rosc {
            registers: ROSC_BASE,
        }
    }
}


