// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

use core::cell::Cell;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, register_structs, ReadOnly, ReadWrite};
use kernel::utilities::StaticRef;

register_structs! {
    POWMANRegisters {
        (0x000 => badpasswd: ReadWrite<u32,POWMAN_BADPASSWD::Register>),
        (0x004 => vreg_ctrl: ReadWrite<u32,POWMAN_VREG_CTRL::Register>),
        (0x008 => vreg_sts: ReadWrite<u32,POWMAN_VREG_STS::Register>),
        (0x00c => vreg: ReadWrite<u32,POWMAN_VREG::Register>),
        (0x010 => vreg_lp_entry: ReadWrite<u32,POWMAN_VREG_LP_ENTRY::Register>),
        (0x014 => vreg_lp_exit: ReadWrite<u32,POWMAN_VREG_LP_EXIT::Register>),
        (0x018 => bod_ctrl: ReadWrite<u32,POWMAN_BOD_CTRL::Register>),
        (0x01c => bod: ReadWrite<u32,POWMAN_BOD::Register>),
        (0x020 => bod_lp_entry: ReadWrite<u32,POWMAN_BOD_LP_ENTRY::Register>),
        (0x024 => bod_lp_exit: ReadWrite<u32,POWMAN_BOD_LP_EXIT::Register>),
        (0x028 =>lposc: ReadWrite<u32,POWMAN_LPOSC::Register>),
        (0x02c => chip_reset: ReadWrite<u32,POWMAN_CHIPRESET::Register>),
        (0x030 => wdsel: ReadWrite<u32,POWMAN_WDSEL::Register>),
        (0x034 => seq_cfg: ReadWrite<u32,POWMAN_SEQ_CFG::Register>),
        (0x038 => state: ReadWrite<u32,POWMAN_STATE::Register>),
        (0x03c => pow_fastdiv: ReadWrite<u32,POWMAN_POW_FASTDIV::Register>),
        (0x040 => pow_delay: ReadWrite<u32,POWMAN_POW_DELAY::Register>),
        (0x044 => ext_ctrl0: ReadWrite<u32,POWMAN_EXT_CTRL0::Register>),
        (0x048 => ext_ctrl1: ReadWrite<u32,POWMAN_EXT_CTRL1::Register>),
        (0x04c => ext_time_ref: ReadWrite<u32,POWMAN_EXT_TIME_REF::Register>),
        (0x050 => lposc_freq_khz_int: ReadWrite<u32,POWMAN_LPOSC_FREQ_KHZ_INT::Register>),
        (0x054 => lposc_freq_khz_frac: ReadWrite<u32,POWMAN_LPOSC_FREQ_KHZ_FRAC::Register>),
        (0x058 => xosc_freq_khz_int: ReadWrite<u32,POWMAN_XOSC_FREQ_KHZ_INT::Register>),
        (0x05c => xosc_freq_khz_frac: ReadWrite<u32,POWMAN_XOSC_FREQ_KHZ_FRAC::Register>),
        (0x060 => set_time_63to48: ReadWrite<u32,POWMAN_SET_TIME_63TO48::Register>),
        (0x064 => set_time_47to32: ReadWrite<u32,POWMAN_SET_TIME_47TO32::Register>),
        (0x068 => set_time_31to16: ReadWrite<u32,POWMAN_SET_TIME_31TO16::Register>),
        (0x06c => set_time_15to0: ReadWrite<u32,POWMAN_SET_TIME_15TO0::Register>),
        (0x070 => read_time_upper: ReadWrite<u32,POWMAN_READ_TIME_UPPER::Register>),
        (0x074 => read_time_lower: ReadWrite<u32,POWMAN_READ_TIME_LOWER::Register>),
        (0x078 => alarm_time_63to48: ReadWrite<u32,POWMAN_ALARM_TIME_63TO48::Register>),
        (0x07c => alarm_time_47to32: ReadWrite<u32,POWMAN_ALARM_TIME_47TO32::Register>),
        (0x080 => alarm_time_31to16: ReadWrite<u32,POWMAN_ALARM_TIME_31TO16::Register>),
        (0x084 => alarm_time_15to0: ReadWrite<u32,POWMAN_ALARM_TIME_15TO0::Register>),
        (0x088 => timer: ReadWrite<u32,POWMAN_TIMER::Register>),
        (0x08c => pwrup0: ReadWrite<u32,POWMAN_PWRUP0::Register>),
        (0x090 => pwrup1: ReadWrite<u32,POWMAN_PWRUP1::Register>),
        (0x094 => pwrup2: ReadWrite<u32,POWMAN_PWRUP2::Register>),
        (0x098 => pwrup3: ReadWrite<u32,POWMAN_PWRUP3::Register>),
        (0x09c => current_pwrup_req: ReadWrite<u32,POWMAN_CURRENT_PWRUP_REQ::Register>),
        (0x0a0 => current_last_swcore_pwrup: ReadWrite<u32,POWMAN_LAST_SWCORE_PWRUP_REQ::Register>),
        (0x0a4 => dbg_pwrcfg: ReadWrite<u32,POWMAN_DBG_PWRCFG::Register>),
        (0x0a8 => bootdis: ReadWrite<u32,POWMAN_BOOTDIS::Register>),
        (0x0ac => dbgconfig: ReadWrite<u32,POWMAN_DBGCONFIG::Register>),
        (0x0b0 => scratch0: ReadWrite<u32,POWMAN_SCRATCH0::Register>),
        (0x0b4 => scratch1: ReadWrite<u32,POWMAN_SCRATCH1::Register>),
        (0x0b8 => scratch2: ReadWrite<u32,POWMAN_SCRATCH2::Register>),
        (0x0bc => scratch3: ReadWrite<u32,POWMAN_SCRATCH3::Register>),
        (0x0c0 => scratch4: ReadWrite<u32,POWMAN_SCRATCH4::Register>),
        (0x0c4 => scratch5: ReadWrite<u32,POWMAN_SCRATCH5::Register>),
        (0x0c8 => scratch6: ReadWrite<u32,POWMAN_SCRATCH6::Register>),
        (0x0cc => scratch7: ReadWrite<u32,POWMAN_SCRATCH7::Register>),
        (0x0d0 => boot0: ReadWrite<u32,POWMAN_BOOT0::Register>),
        (0x0d4 => boot1: ReadWrite<u32,POWMAN_BOOT1::Register>),
        (0x0d8 => boot2: ReadWrite<u32,POWMAN_BOOT2::Register>),
        (0x0dc => boot3: ReadWrite<u32,POWMAN_BOOT3::Register>),
        (0x0e0 => intr: ReadWrite<u32,POWMAN_INTR::Register>),
        (0x0e4 => inte: ReadWrite<u32,POWMAN_INTE::Register>),
        (0x0e8 => intf: ReadWrite<u32,POWMAN_INTF::Register>),
        (0x0ec => ints: ReadWrite<u32,POWMAN_INTS::Register>),
        (0x0f0 => @END),
    }
    }

    register_bitfields![u32,
POWMAN_BADPASSWD [
    VALUE OFFSET(0) NUMBITS(1) [],
],
POWMAN_VREG_CTRL [
    RST_N OFFSET(15) NUMBITS(1) [],
    UNLOCK OFFSET(13) NUMBITS(1) [],
    ISOLATE OFFSET(12) NUMBITS(1) [],
    DISABLE_VOLTAGE_LIMIT OFFSET(8) NUMBITS(1) [],
    HT_TH OFFSET(4) NUMBITS(3) [],
    RESERVED  OFFSET(0) NUMBITS(1) [],
],
POWMAN_VREG_STS [
VOUT_OK OFFSET(4) NUMBITS(1) [],
STARTUP OFFSET(0) NUMBITS(1) [],
],
POWMAN_VREG [
UPDATE_IN_PROGRESS OFFSET(15) NUMBITS(1) [],
VSEL OFFSET(4) NUMBITS(4) [],
RESERVED OFFSET(2) NUMBITS(1) [],
HIZ OFFSET(1) NUMBITS(1) [],
],
POWMAN_VREG_LP_ENTRY [
    VSEL OFFSET(4) NUMBITS(4) [],
    MODE OFFSET(2) NUMBITS(1) [],
    HIZ OFFSET(1) NUMBITS(1) [],
],

POWMAN_VREG_LP_EXIT [
    VSEL OFFSET(4) NUMBITS(4) [],
    MODE OFFSET(2) NUMBITS(1) [],
    HIZ OFFSET(1) NUMBITS(1) [],
],
POWMAN_BOD_CTRL [
    ISOLATE OFFSET(12) NUMBITS(1) [],
],
POWMAN_BOD [
    VSEL OFFSET(4) NUMBITS(4) [],
    MODE OFFSET(2) NUMBITS(1) [],
    HIZ OFFSET(1) NUMBITS(1) [],
],
POWMAN_BOD_LP_ENTRY [
    VSEL OFFSET(4) NUMBITS(4) [],
    MODE OFFSET(2) NUMBITS(1) [],
    HIZ OFFSET(1) NUMBITS(1) [],
],

POWMAN_BOD_LP_EXIT [
    VSEL OFFSET(4) NUMBITS(4) [],
    MODE OFFSET(2) NUMBITS(1) [],
    HIZ OFFSET(1) NUMBITS(1) [],
],

POWMAN_LPOSC [
    TRIM OFFSET(4) NUMBITS(6) [],
    MODE OFFSET(0) NUMBITS(2) [],
],
POWMAN_CHIPRESET [
    HAD_WATCHDOG_RESET_PSM OFFSET(28) NUMBITS(1) [],
    HAD_HZD_SYS_RESET_REQ OFFSET(27) NUMBITS(1) [],
    HAD_GLITCH_DETECT OFFSET(26) NUMBITS(1) [],
    HAD_SWCORE_PD OFFSET(25) NUMBITS(1) [],
    HAD_WATCHDOG_RESET_SWCORE OFFSET(24) NUMBITS(1) [],
    HAD_WATCHDOG_RESET_POWMAN OFFSET(23) NUMBITS(1) [],
    HAD_WATCHDOG_RESET_POWMAN_ASYNC  OFFSET(22) NUMBITS(1) [],
    HAD_RESCUE  OFFSET(21) NUMBITS(1) [],
    HAD_DP_RESET_REQ OFFSET(19) NUMBITS(1) [],
    HAD_RUN_LOW OFFSET(18) NUMBITS(1) [],
    HAD_BOR OFFSET(17) NUMBITS(1) [],
    HAD_POR OFFSET(16) NUMBITS(1) [],
    RESCUE_FLAG OFFSET(4) NUMBITS(1) [],
    DOUBLE_TAP OFFSET(0) NUMBITS(1) [],
],
POWMAN_WDSEL [
    RESET_PSM OFFSET(12) NUMBITS(1) [],
    RESET_SW_CORE OFFSET(8) NUMBITS(1) [],
    RESET_POWMAN_ASYNC OFFSET(0) NUMBITS(1) [],
],
POWMAN_SEQ_CFG [
    USING_FAST_POWCK OFFSET(20) NUMBITS(1) [],
    USING_BOD_LP OFFSET(17) NUMBITS(1) [],
    USING_VREG_LP OFFSET(16) NUMBITS(1) [],
    USE_FAST_POWCK OFFSET(12) NUMBITS(1) [],
    RUN_LPOSC_IN_LP OFFSET(8) NUMBITS(1) [],
    USE_BOD_HP OFFSET(7) NUMBITS(1) [],
    USE_BOD_LP OFFSET(6) NUMBITS(1) [],
    USE_VREG_HP OFFSET(5) NUMBITS(1) [],
    USE_VREG_LP OFFSET(4) NUMBITS(1) [],
    HW_PWRUP_SRAM0 OFFSET(1) NUMBITS(1) [],
    HW_PWRUP_SRAM1 OFFSET(0) NUMBITS(1) [],
],
POWMAN_STATE [
    CHANGING OFFSET(13) NUMBITS(1) [],
    WAITING OFFSET(12) NUMBITS(1) [],
    BAD_HW_REQ OFFSET(11) NUMBITS(1) [],
    BAD_SW_REQ OFFSET(10) NUMBITS(1) [],
    PWRUP_WHILE_WAITING OFFSET(9) NUMBITS(1) [],
    REQ_IGNORED OFFSET(8) NUMBITS(1) [],
    REQ OFFSET(4) NUMBITS(4) [],
    CURRENT OFFSET(0) NUMBITS(4) [],
],
POWMAN_POW_FASTDIV [
    VALUE OFFSET(0) NUMBITS(11) [],
],
POWMAN_POW_DELAY [
    SRAM_STEP OFFSET(8) NUMBITS(8) [],
    XIP_STEP OFFSET(4) NUMBITS(4) [],
    SWCORE OFFSET(0) NUMBITS(4) [],
],
POWMAN_EXT_CTRL0 [
    LP_EXIT_STATE OFFSET(14) NUMBITS(1) [],
    LP_ENTRY_STATE OFFSET(13) NUMBITS(1) [],
    INIT_STATE OFFSET(12) NUMBITS(1) [],
    INIT  OFFSET(8) NUMBITS(1) [],
    GPIO_SELECT OFFSET(0) NUMBITS(6) [],
],
POWMAN_EXT_CTRL1 [
        LP_EXIT_STATE OFFSET(14) NUMBITS(1) [],
        LP_ENTRY_STATE OFFSET(13) NUMBITS(1) [],
        INIT_STATE OFFSET(12) NUMBITS(1) [],
        INIT  OFFSET(8) NUMBITS(1) [],
        GPIO_SELECT OFFSET(0) NUMBITS(6) [],
],  
POWMAN_EXT_TIME_REF [
        DRIVE_LPCK OFFSET(4) NUMBITS(1) [],
        SOURCE_SEL OFFSET(0) NUMBITS(2) [GPIO12=0,GPIO20 = 1,GPIO14=2,GPIO22=3]
],
POWMAN_LPOSC_FREQ_KHZ_INT [
    VALUE OFFSET(0) NUMBITS(6) [],
],
POWMAN_LPOSC_FREQ_KHZ_FRAC [
    VALUE OFFSET(0) NUMBITS(16) [],
],
POWMAN_XOSC_FREQ_KHZ_INT [
    VALUE OFFSET(0) NUMBITS(16) [],
],
POWMAN_XOSC_FREQ_KHZ_FRAC [
    VALUE OFFSET(0) NUMBITS(16) [],
],
POWMAN_SET_TIME_63TO48 [
    VALUE OFFSET(0) NUMBITS(16) [],
],
POWMAN_SET_TIME_47TO32 [
    VALUE OFFSET(0) NUMBITS(16) [],
],
POWMAN_SET_TIME_31TO16 [
    VALUE OFFSET(0) NUMBITS(16) [],
],
POWMAN_SET_TIME_15TO0 [
    VALUE OFFSET(0) NUMBITS(16) [],
],
POWMAN_READ_TIME_UPPER [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_READ_TIME_LOWER [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_ALARM_TIME_63TO48 [
    VALUE OFFSET(0) NUMBITS(16) [],
],
POWMAN_ALARM_TIME_47TO32 [
    VALUE OFFSET(0) NUMBITS(16) [],
],
POWMAN_ALARM_TIME_31TO16 [
    VALUE OFFSET(0) NUMBITS(16) [],
],
POWMAN_ALARM_TIME_15TO0 [
    VALUE OFFSET(0) NUMBITS(16) [],
],
POWMAN_TIMER [
    USING_GPIO_1HZ OFFSET(19) NUMBITS(1) [],
    USING_GPIO_1KHZ OFFSET(18) NUMBITS(1) [],
    USING_LPOSC OFFSET(17) NUMBITS(1) [],
    USING_XOSC OFFSET(16) NUMBITS(1) [],
    USE_GPIO_1HZ OFFSET(13) NUMBITS(1) [],
    USE_GPIO_1KHZ OFFSET(10) NUMBITS(1) [],
    USE_XOSC OFFSET(9) NUMBITS(1) [],
    USE_LPOSC OFFSET(8) NUMBITS(1) [],
    ALARM OFFSET(6) NUMBITS(1) [],
    PWRUP_ON_ALARM OFFSET(5) NUMBITS(1) [],
    ALARM_ENAB OFFSET(4) NUMBITS(1) [],
    CLEAR OFFSET(2) NUMBITS(1) [],
    RUN OFFSET(1) NUMBITS(1) [],
    NONSEC_WRITE OFFSET(0) NUMBITS(1) [],
],
POWMAN_PWRUP0 [
    RAW_STATUS OFFSET(10) NUMBITS(1) [],
    STATUS OFFSET(9) NUMBITS(1) [],
    MODE OFFSET(8) NUMBITS(1) [LEVEL = 0x0,EDGE = 0x1],
    DIRECTION  OFFSET(7) NUMBITS(1) [LOW_FALLING = 0x0,HIGH_RISING = 0x1],
ENABLE OFFSET(6) NUMBITS(1) [],
SOURCE  OFFSET(0) NUMBITS(6) [],
],
POWMAN_PWRUP1 [
    RAW_STATUS OFFSET(10) NUMBITS(1) [],
    STATUS OFFSET(9) NUMBITS(1) [],
    MODE OFFSET(8) NUMBITS(1) [LEVEL = 0x0,EDGE = 0x1],
    DIRECTION  OFFSET(7) NUMBITS(1) [LOW_FALLING = 0x0,HIGH_RISING = 0x1],
ENABLE OFFSET(6) NUMBITS(1) [],
SOURCE  OFFSET(0) NUMBITS(6) [],
],
POWMAN_PWRUP2 [
    RAW_STATUS OFFSET(10) NUMBITS(1) [],
    STATUS OFFSET(9) NUMBITS(1) [],
    MODE OFFSET(8) NUMBITS(1) [LEVEL = 0x0,EDGE = 0x1],
    DIRECTION  OFFSET(7) NUMBITS(1) [LOW_FALLING = 0x0,HIGH_RISING = 0x1],
ENABLE OFFSET(6) NUMBITS(1) [],
SOURCE  OFFSET(0) NUMBITS(6) [],
],
POWMAN_PWRUP3 [
    RAW_STATUS OFFSET(10) NUMBITS(1) [],
    STATUS OFFSET(9) NUMBITS(1) [],
    MODE OFFSET(8) NUMBITS(1) [LEVEL = 0x0,EDGE = 0x1],
    DIRECTION  OFFSET(7) NUMBITS(1) [LOW_FALLING = 0x0,HIGH_RISING = 0x1],
ENABLE OFFSET(6) NUMBITS(1) [],
SOURCE  OFFSET(0) NUMBITS(6) [],
],
POWMAN_CURRENT_PWRUP_REQ [
    VALUE OFFSET(0) NUMBITS(7) [],
],
POWMAN_LAST_SWCORE_PWRUP_REQ [
    VALUE OFFSET(0) NUMBITS(7) [],
],
POWMAN_DBG_PWRCFG [
    VALUE OFFSET(0) NUMBITS(1) [],
],
POWMAN_BOOTDIS [
    NEXT OFFSET(1) NUMBITS(1) [],
    NOW OFFSET(0) NUMBITS(1) [],
],
POWMAN_DBGCONFIG [
    DP_INSTID OFFSET(0) NUMBITS(4) [],
],
POWMAN_SCRATCH0 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_SCRATCH1 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_SCRATCH2 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_SCRATCH3 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_SCRATCH4 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_SCRATCH5 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_SCRATCH6 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_SCRATCH7 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_BOOT0 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_BOOT1 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_BOOT2 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_BOOT3 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_BOOT4 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_BOOT5 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_BOOT6 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_BOOT7 [
    VALUE OFFSET(0) NUMBITS(32) [],
],
POWMAN_INTR [
    PWRUP_WHILE_WAITING OFFSET(3) NUMBITS(1) [],
    STATE_REQ_IGNORED OFFSET(2) NUMBITS(1) [],
    TIMER OFFSET(1) NUMBITS(1) [],
    VREG_OUTPUT_LOW OFFSET(0) NUMBITS(1) [],
],
POWMAN_INTE [
    PWRUP_WHILE_WAITING OFFSET(3) NUMBITS(1) [],
    STATE_REQ_IGNORED OFFSET(2) NUMBITS(1) [],
    TIMER OFFSET(1) NUMBITS(1) [],
    VREG_OUTPUT_LOW OFFSET(0) NUMBITS(1) [],
],

POWMAN_INTF [
    PWRUP_WHILE_WAITING OFFSET(3) NUMBITS(1) [],
    STATE_REQ_IGNORED OFFSET(2) NUMBITS(1) [],
    TIMER OFFSET(1) NUMBITS(1) [],
    VREG_OUTPUT_LOW OFFSET(0) NUMBITS(1) [],
],
POWMAN_INTS [
    PWRUP_WHILE_WAITING OFFSET(3) NUMBITS(1) [],
    STATE_REQ_IGNORED OFFSET(2) NUMBITS(1) [],
    TIMER OFFSET(1) NUMBITS(1) [],
    VREG_OUTPUT_LOW OFFSET(0) NUMBITS(1) [],
]
];

const POWMAN_BASE: StaticRef<POWMANRegisters> =
unsafe { StaticRef::new(0x40100000 as *const POWMANRegisters) };

pub struct POWMAN {
    registers: StaticRef<POWMANRegisters>,
}

impl POWMAN {
    #[inline(never)]
    pub const fn new() -> POWMAN {
        POWMAN {
            registers: POWMAN_BASE,
        }
    }
}
