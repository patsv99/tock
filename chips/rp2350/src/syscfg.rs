// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

// use kernel::utilities::registers::interfaces::Readable;
use kernel::utilities::StaticRef;

use kernel::utilities::registers::{register_bitfields, register_structs, ReadWrite};

register_structs! {

    SysCfgRegisters {

        (0x000 => proc_config: ReadWrite<u32,PROC_CONFIG::Register>),

        (0x004 => proc_in_sync_bypass: ReadWrite<u32, PROC_IN_SYNC_BYPASS::Register>),

        (0x008 => _proc_in_sync_bypass_hi: ReadWrite<u32,PROC_IN_SYNC_BYPASS_HI::Register> ),

        (0x00c => dbgforce: ReadWrite<u32, DBGFORCE::Register>),
        (0x010 => mempowerdown: ReadWrite<u32, MEMPOWERDOWN::Register>),
        (0x014 => auxctrl: ReadWrite<u32, AUXCTRL::Register>),
        (0x018 => @END),
    }
}
register_bitfields![u32,
    PROC_CONFIG [

        PROC0_HALTED OFFSET(0) NUMBITS(1) [],
        PROC1_HALTED OFFSET(1) NUMBITS(1) [],

    ],
    PROC_IN_SYNC_BYPASS [
        GPIO OFFSET(0) NUMBITS(32) [],
    ],
    PROC_IN_SYNC_BYPASS_HI [
        GPIO OFFSET(0) NUMBITS(16) [],
        USB_DP OFFSET(24) NUMBITS(1) [],
        USB_DM OFFSET(25) NUMBITS(1) [],
        QSPI_SCK OFFSET(26) NUMBITS(1) [],
        QSPI_CSN OFFSET(27) NUMBITS(1) [],
        QSPI_SD OFFSET(28) NUMBITS(4) [],
    ],
    DBGFORCE [
        SWDO OFFSET(0) NUMBITS(1) [],
        SWDI OFFSET(1) NUMBITS(1) [],
        SWCLK OFFSET(2) NUMBITS(1) [],
        ATTACH OFFSET(3) NUMBITS(1) [],
    ],
    MEMPOWERDOWN [
        SRAM0 OFFSET(0) NUMBITS(1) [],
        SRAM1 OFFSET(1) NUMBITS(1) [],
        SRAM2 OFFSET(2) NUMBITS(1) [],
        SRAM3 OFFSET(3) NUMBITS(1) [],
        SRAM4 OFFSET(4) NUMBITS(1) [],
        SRAM5 OFFSET(5) NUMBITS(1) [],
        SRAM6 OFFSET(6) NUMBITS(1) [],
        SRAM7 OFFSET(7) NUMBITS(1) [],
        SRAM8 OFFSET(8) NUMBITS(1) [],
        SRAM9 OFFSET(9) NUMBITS(1) [],
USB OFFSET(10) NUMBITS(1) [],
ROM OFFSET(11) NUMBITS(1) [],
BOOTRAM OFFSET(12) NUMBITS(1) [],
    ],
    AUXCTRL [
        //  Force POWMAN clock to switch to LPOSC, by asserting its WDRESET
//input. This must be set before initiating a watchdog reset of the RSM from a
//stage that includes CLOCKS, if POWMAN is running from clk_ref at the point
//that the watchdog reset takes place. Otherwise, the short pulse generated on
//clk_ref by the reset of the CLOCKS block may affect POWMAN register state
        BIT0 OFFSET(0) NUMBITS(1) [],
        // When clear, the LPOSC output is XORed into the TRNG ROSC output as
//an additional, uncorrelated entropy source. When set, this behaviour is
// disabled.
        BIT1 OFFSET(0) NUMBITS(1) [],
//Set to mask OTP power analogue power supply detection from resetting OTP controller and PSM
        BIT2 OFFSET(0) NUMBITS(1) [],

    ]
];

const SYSCFG_BASE: StaticRef<SysCfgRegisters> =
    unsafe { StaticRef::new(0x40008000 as *const SysCfgRegisters) };

pub struct SysCfg {
    registers: StaticRef<SysCfgRegisters>,
}

impl SysCfg {
    #[inline(never)]
    pub const fn new() -> SysCfg {
        SysCfg {
            registers: SYSCFG_BASE,
        }
    }
}
