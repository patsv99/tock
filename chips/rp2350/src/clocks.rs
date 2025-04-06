// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

use core::cell::Cell;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, register_structs, ReadOnly, ReadWrite};
use kernel::utilities::StaticRef;

/* Clocks base */
register_structs! {
    GpioClockRegisters {
        /// Clock control, can be changed on-the-fly (except for auxsrc)
        (0x000 => ctrl: ReadWrite<u32, CLK_GPOUTx_CTRL::Register>),
        /// Clock divisor, can be changed on-the-fly
        (0x004 => div: ReadWrite<u32, CLK_GPOUTx_DIV::Register>),
        /// Indicates which src is currently selected (one-hot)
        (0x008 => selected: ReadOnly<u32, CLK_GPOUTx_SELECTED::Register>),
        /// Clock control, can be changed on-the-fly (except for auxsrc)
        (0x00C => @END),
    },
    ClocksRegisters {
        (0x000 => clk_gpio: [GpioClockRegisters; 4]),
        /// Clock control, can be changed on-the-fly (except for auxsrc)
        (0x030 => clk_ref_ctrl: ReadWrite<u32, CLK_REF_CTRL::Register>),
        /// Clock divisor, can be changed on-the-fly
        (0x034 => clk_ref_div: ReadWrite<u32>),
        /// Indicates which src is currently selected (one-hot)
        (0x038 => clk_ref_selected: ReadOnly<u32, CLK_REF_SELECTED::Register>),
        /// Clock control, can be changed on-the-fly (except for auxsrc)
        (0x03C => clk_sys_ctrl: ReadWrite<u32, CLK_SYS_CTRL::Register>),
        /// Clock divisor, can be changed on-the-fly
        (0x040 => clk_sys_div: ReadWrite<u32, CLK_SYS_DIV::Register>),
        /// Indicates which src is currently selected (one-hot)
        (0x044 => clk_sys_selected: ReadOnly<u32, CLK_SYS_SELECTED::Register>),
        /// Clock control, can be changed on-the-fly (except for auxsrc)
        (0x048 => clk_peri_ctrl: ReadWrite<u32, CLK_PERI_CTRL::Register>),
        (0x04C => clk_peri_div: ReadWrite<u32,CLK_PERI_DIV::Register>),
        /// Indicates which src is currently selected (one-hot)
        (0x050 => clk_peri_selected: ReadOnly<u32, CLK_PERI_SELECTED::Register>),
        /// Clock control, can be changed on-the-fly (except for auxsrc)
        (0x054 => clk_hstx_ctrl: ReadWrite<u32, CLK_HSTX_CTRL::Register>),
        /// Clock divisor, can be changed on-the-fly
        (0x058 => clk_hstx_div: ReadWrite<u32,CLK_HSTX_DIV::Register>),
        /// Indicates which src is currently selected (one-hot)
        (0x05C => clk_hstx_selected: ReadOnly<u32, CLK_HSTX_SELECTED::Register>),

        /// Clock control, can be changed on-the-fly (except for auxsrc)
        (0x060 => clk_usb_ctrl: ReadWrite<u32, CLK_USB_CTRL::Register>),
        /// Clock divisor, can be changed on-the-fly
        (0x064 => clk_usb_div: ReadWrite<u32>),
        /// Indicates which src is currently selected (one-hot)
        (0x068 => clk_usb_selected: ReadOnly<u32, CLK_USB_SELECTED::Register>),
        /// Clock control, can be changed on-the-fly (except for auxsrc)
        (0x06c => clk_adc_ctrl: ReadWrite<u32, CLK_ADC_CTRL::Register>),
        /// Clock divisor, can be changed on-the-fly
        (0x070 => clk_adc_div: ReadWrite<u32>),
        /// Indicates which src is currently selected (one-hot)
        (0x074 => clk_adc_selected: ReadOnly<u32, CLK_ADC_SELECTED::Register>),
        /// Clock control, can be changed on-the-fly (except for auxsrc)
        (0x078 => dftclk_xosc_ctrl: ReadWrite<u32, DFTCLK_XOSC_CTRL::Register>),
        /// Clock divisor, can be changed on-the-fly
        (0x07c => dftclk_rosc_ctrl: ReadWrite<u32,DFTCLK_ROSC_CTRL::Register>),
        (0x080 => dftclk_lposc_ctrl: ReadWrite<u32,DFTCLK_LPOSC_CTRL::Register>),
        (0x084 => clk_sys_resus_ctrl: ReadWrite<u32, CLK_SYS_RESUS_CTRL::Register>),
        (0x088 => clk_sys_resus_status: ReadWrite<u32,CLK_SYS_RESUS_STATUS::Register>),
        /// Reference clock frequency in kHz
        (0x08c => fc0_ref_khz: ReadWrite<u32>),
        /// Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using t
        (0x090 => fc0_min_khz: ReadWrite<u32>),
        /// Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not
        (0x094 => fc0_max_khz: ReadWrite<u32>),
        /// Delays the start of frequency counting to allow the mux to settle
        /// Delay is measured in multiples of the reference clock period
        (0x098 => fc0_delay: ReadWrite<u32>),
        /// The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval
        /// The default gives a test interval of 250us
        (0x09c => fc0_interval: ReadWrite<u32>),
        /// Clock sent to frequency counter, set to 0 when not required
        /// Writing to this register initiates the frequency count
        (0x0a0 => fc0_src: ReadWrite<u32>),
        /// Frequency counter status
        (0x0a4 => fc0_status: ReadWrite<u32, FC0_STATUS::Register>),
        /// Result of frequency measurement, only valid when status_done=1
        (0x0a8 => fc0_result: ReadWrite<u32, FC0_RESULT::Register>),
        /// enable clock in wake mode
        (0x0Ac => wake_en0: ReadWrite<u32, WAKE_EN0::Register>),
        /// enable clock in wake mode
        (0x0b0 => wake_en1: ReadWrite<u32, WAKE_EN1::Register>),
        /// enable clock in sleep mode
        (0x0b4 => sleep_en0: ReadWrite<u32, SLEEP_EN0::Register>),
        /// enable clock in sleep mode
        (0x0b8 => sleep_en1: ReadWrite<u32, SLEEP_EN1::Register>),
        /// indicates the state of the clock enable
        (0x0Bc => enabled0: ReadWrite<u32, ENABLED0::Register>),
        /// indicates the state of the clock enable
        (0x0c0 => enabled1: ReadWrite<u32, ENABLED1::Register>),
        /// Raw Interrupts
        (0x0c4 => intr: ReadWrite<u32>),
        /// Interrupt Enable
        (0x0c8 => inte: ReadWrite<u32>),
        /// Interrupt Force
        (0x0CC => intf: ReadWrite<u32>),
        /// Interrupt status after masking & forcing
        (0x0d0 => ints: ReadWrite<u32>),
        (0x0d4 => @END),
    }
}

register_structs! {
    PllRegisters {
        /// Control and Status
        /// GENERAL CONSTRAINTS:
        /// Reference clock frequency min=5MHz, max=800MHz
        /// Feedback divider min=16, max=320
        /// VCO frequency min=400MHz, max=1600MHz
        (0x000 => cs: ReadWrite<u32, PLL_CS::Register>),
        /// Controls the PLL power modes.
        (0x004 => pwr: ReadWrite<u32, PLL_PWR::Register>),
        /// Feedback divisor
        /// (note: this PLL does not support fractional division)
        (0x008 => fbdiv_int: ReadWrite<u32, PLL_FBDIV_INT::Register>),
        /// Controls the PLL post dividers for the primary output
        /// (note: this PLL does not have a secondary output)
        /// the primary output is driven from VCO divided by postdiv1*postdiv2
        (0x00C => prim: ReadWrite<u32, PLL_PRIM::Register>),
        (0x010 => intr: ReadWrite<u32, PLL_INTR::Register>),
        (0x014 => inte: ReadWrite<u32, PLL_INTE::Register>),
        (0x018 => intf: ReadWrite<u32, PLL_INTF::Register>),
        (0x01c => ints: ReadWrite<u32, PLL_INTS::Register>),
        (0x020 => @END),
    }
}

register_structs! {
    XOSCRegisters {
(0x000 => ctrl: ReadWrite<u32,XOSC_CTRL::Register>),
(0x004 => status: ReadWrite<u32,XOSC_STATUS::Register>),
(0x008 => dormant: ReadWrite<u32,XOSC_DORMANT::Register>),
(0x00c => startup: ReadWrite<u32,XOSC_STARTUP::Register>),
(0x010 => count: ReadWrite<u32,XOSC_COUNT::Register>),
(0x014 => @END),
}
}

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
    (0x0d4 => boot0: ReadWrite<u32,POWMAN_BOOT1::Register>),
    (0x0d8 => boot0: ReadWrite<u32,POWMAN_BOOT2::Register>),
    (0x0dc => boot0: ReadWrite<u32,POWMAN_BOOT3::Register>),
    (0x0e0 => intr: ReadWrite<u32,POWMAN_INTR::Register>),
    (0x0e4 => inte: ReadWrite<u32,POWMAN_INTE::Register>),
    (0x0e8 => intf: ReadWrite<u32,POWMAN_INTF::Register>),
    (0x0ec => ints: ReadWrite<u32,POWMAN_INTS::Register>),
    (0x0f0 => @END),
}
}

register_bitfields![u32,
XOSC_CTRL [
    ENABLE OFFSET(12) NUMBITS(12) [ DISABLE = 0xd1e,ENABLE = 0xfab],
    FREQ_RANGE OFFSET(0) NUMBITS(12) []
],
XOSC_STATUS [
STABLE OFFSET(31) NUMBITS(1) [],
BADWRITE OFFSET(24) NUMBITS(1) [],
ENABLED OFFSET(12) NUMBITS(1) [],
FREQ_RANGE OFFSET(0) NUMBITS(2) [
F1_15MHZ = 0,
F10_30MHZ = 1,
F25_60MHZ = 2,
F40_100MHZ = 3,
]
],
XOSC_DORMANT [
    VALUE OFFSET(0) NUMBITS(32) [
        DORMANT = 0x636f6d61,
        WAKE = 0x77616b65
    ],
],
XOSC_STARTUP [
    X4 OFFSET(20) NUMBITS(1) [],
    DELAY OFFSET(0) NUMBITS(14) [],
],
XOSC_COUNT [
    COUNTER OFFSET(0) NUMBITS(16) [],
],
];

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
    RUN_LPOSC_IN_LP OFFSET(8) NUMBITS(1) [].
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



register_bitfields![u32,
    CLK_GPOUTx_CTRL [
        /// An edge on this signal shifts the phase of the output by 1 cycle of the input cl
        /// This can be done at any time
        NUDGE OFFSET(20) NUMBITS(1) [],
        /// This delays the enable signal by up to 3 cycles of the input clock
        /// This must be set before the clock is enabled to have any effect
        PHASE OFFSET(16) NUMBITS(2) [],
        /// Enables duty cycle correction for odd divisors
        DC50 OFFSET(12) NUMBITS(1) [],
        /// Starts and stops the clock generator cleanly
        ENABLE OFFSET(11) NUMBITS(1) [],
        /// Asynchronously kills the clock generator
        KILL OFFSET(10) NUMBITS(1) [],
        /// Selects the auxiliary clock source, will glitch when switching
        AUXSRC OFFSET(5) NUMBITS(4) [
            CLKSRC_PLL_SYS = 0,
            CLKSRC_GPIN0 = 1,
            CLKSRC_GPIN1 = 2,
            CLKSRC_PLL_USB = 3,
            ROSC_CLKSRC = 4,
            XOSC_CLKSRC = 5,
            CLK_SYS = 6,
            CLK_USB = 7,
            CLK_ADC = 8,
            CLK_RTC = 9,
            CLK_REF = 0xa
        ],
    ],
    CLK_GPOUTx_DIV [
        /// Integer component of the divisor, 0 -> divide by 2^16
        INT OFFSET(8) NUMBITS(24) [],
        /// Fractional component of the divisor
        FRAC OFFSET(0) NUMBITS(8) []
    ],
    CLK_GPOUTx_SELECTED [
        VALUE OFFSET (0) NUMBITS (32) []
    ],
    CLK_REF_CTRL [
        /// Selects the auxiliary clock source, will glitch when switching
        AUXSRC OFFSET(5) NUMBITS(2) [
            CLKSRC_PLL_USB = 0x0,
            CLKSRC_GPIN0 = 0x1,
            CLKSRC_GPIN1 = 0x2,
            CLKSRC_PLL_USB_PRIMARY_REF_OPCG = 0x3,
        ],
        /// Selects the clock source glitchlessly, can be changed on-the-fly
        SRC OFFSET(0) NUMBITS(2) [
            ROSC_CLKSRC_PH = 0x0,
            CLKSRC_CLK_REF_AUX = 0x1,
            XOSC_CLKSRC = 0x2
        ]
    ],
    CLK_REF_DIV [
        /// Integer component of the divisor, 0 -> divide by 2^16
        INT OFFSET(8) NUMBITS(2) []
    ],
    CLK_REF_SELECTED [
        VALUE OFFSET (0) NUMBITS (32) []
    ],
    CLK_SYS_CTRL [
        /// Selects the auxiliary clock source, will glitch when switching
        AUXSRC OFFSET(5) NUMBITS(3) [

            CLKSRC_PLL_SYS = 0x0,
            CLKSRC_PLL_USB = 0x1,
            ROSC_CLKSRC = 0x2,
            XOSC_CLKSRC = 0x3,
            CLKSRC_GPIN0 = 0x4,
            CLKSRC_GPIN1 = 0x5
        ],
        /// Selects the clock source glitchlessly, can be changed on-the-fly
        SRC OFFSET(0) NUMBITS(1) [
            CLKSRC_CLK_SYS_AUX = 1,
            CLK_REF = 0
        ]
    ],
    CLK_SYS_DIV [
        /// Integer component of the divisor, 0 -> divide by 2^16
        INT OFFSET(8) NUMBITS(24) [],
        /// Fractional component of the divisor
        FRAC OFFSET(0) NUMBITS(8) []
    ],
    CLK_SYS_SELECTED [
        VALUE OFFSET (0) NUMBITS (32) []
    ],
    CLK_PERI_CTRL [
        /// Starts and stops the clock generator cleanly
        ENABLE OFFSET(11) NUMBITS(1) [],
        /// Asynchronously kills the clock generator
        KILL OFFSET(10) NUMBITS(1) [],
        /// Selects the auxiliary clock source, will glitch when switching
        AUXSRC OFFSET(5) NUMBITS(3) [
            CLK_SYS = 0,
            CLKSRC_PLL_SYS = 1,
            CLKSRC_PLL_USB = 2,
            ROSC_CLKSRC_PH = 3,
            XOSC_CLKSRC = 4,
            CLKSRC_GPIN0 = 5,
            CLKSRC_GPIN1 = 6
        ]
    ],
    CLK_PERI_DIV [
        ///  Integer part of clock divisor, 0 → max+1, can be changed on-the-fly
        INT OFFSET(16) NUMBITS(2) [],
    ],

    CLK_PERI_SELECTED [
        VALUE OFFSET (0) NUMBITS (32) []
    ],
    CLK_HSTX_CTRL [
        ENABLED OFFSET(28) NUMBITS(1) [],
        NUDGE OFFSET(20) NUMBITS(1) [],
        PHASE OFFSET(16) NUMBITS(2) [],
        ENABLE OFFSET(11) NUMBITS(1) [],
        KILL OFFSET(10) NUMBITS(1) [],
        AUXSRC OFFSET(5) NUMBITS(3) [],
    ],
    CLK_HSTX_DIV [
        INT OFFSET(16) NUMBITS(2) []
    ],
    CLK_HSTX_SELECTED [
        VALUE OFFSET (0) NUMBITS (32) []
    ],
    CLK_USB_CTRL [
        /// An edge on this signal shifts the phase of the output by 1 cycle of the input cl
        /// This can be done at any time
        NUDGE OFFSET(20) NUMBITS(1) [],
        /// This delays the enable signal by up to 3 cycles of the input clock
        /// This must be set before the clock is enabled to have any effect
        PHASE OFFSET(16) NUMBITS(2) [],
        /// Starts and stops the clock generator cleanly
        ENABLE OFFSET(11) NUMBITS(1) [],
        /// Asynchronously kills the clock generator
        KILL OFFSET(10) NUMBITS(1) [],
        /// Selects the auxiliary clock source, will glitch when switching
        AUXSRC OFFSET(5) NUMBITS(3) [

            CLKSRC_PLL_USB = 0,
            CLKSRC_PLL_SYS = 1,
            ROSC_CLKSRC_PH = 2,
            XOSC_CLKSRC = 3,
            CLKSRC_GPIN0 = 4,
            CLKSRC_GPIN1 = 5
        ]
    ],
    CLK_USB_DIV [
        /// Integer component of the divisor, 0 -> divide by 2^16
        INT OFFSET(8) NUMBITS(2) []
    ],
    CLK_USB_SELECTED [
        VALUE OFFSET (0) NUMBITS (32) []
    ],
    CLK_XOSC_CTRL [
        /// An edge on this signal shifts the phase of the output by 1 cycle of the input cl
        /// This can be done at any time
        NUDGE OFFSET(20) NUMBITS(1) [],
        /// This delays the enable signal by up to 3 cycles of the input clock
        /// This must be set before the clock is enabled to have any effect
        PHASE OFFSET(16) NUMBITS(2) [],
        /// Starts and stops the clock generator cleanly
        ENABLE OFFSET(11) NUMBITS(1) [],
        /// Asynchronously kills the clock generator
        KILL OFFSET(10) NUMBITS(1) [],
        /// Selects the auxiliary clock source, will glitch when switching
        AUXSRC OFFSET(5) NUMBITS(3) [

            CLKSRC_PLL_USB = 0,
            CLKSRC_PLL_SYS = 1,
            ROSC_CLKSRC_PH = 2,
            XOSC_CLKSRC = 3,
            CLKSRC_GPIN0 = 4,
            CLKSRC_GPIN1 = 5
        ]
    ],
    CLK_ROSC_CTRL [
        /// An edge on this signal shifts the phase of the output by 1 cycle of the input cl
        /// This can be done at any time
        NUDGE OFFSET(20) NUMBITS(1) [],
        /// This delays the enable signal by up to 3 cycles of the input clock
        /// This must be set before the clock is enabled to have any effect
        PHASE OFFSET(16) NUMBITS(2) [],
        /// Starts and stops the clock generator cleanly
        ENABLE OFFSET(11) NUMBITS(1) [],
        /// Asynchronously kills the clock generator
        KILL OFFSET(10) NUMBITS(1) [],
        /// Selects the auxiliary clock source, will glitch when switching
        AUXSRC OFFSET(5) NUMBITS(3) [

            CLKSRC_PLL_USB = 0,
            CLKSRC_PLL_SYS = 1,
            ROSC_CLKSRC_PH = 2,
            XOSC_CLKSRC = 3,
            CLKSRC_GPIN0 = 4,
            CLKSRC_GPIN1 = 5
        ]
    ],
    CLK_LPOSC_CTRL [
        /// An edge on this signal shifts the phase of the output by 1 cycle of the input cl
        /// This can be done at any time
        NUDGE OFFSET(20) NUMBITS(1) [],
        /// This delays the enable signal by up to 3 cycles of the input clock
        /// This must be set before the clock is enabled to have any effect
        PHASE OFFSET(16) NUMBITS(2) [],
        /// Starts and stops the clock generator cleanly
        ENABLE OFFSET(11) NUMBITS(1) [],
        /// Asynchronously kills the clock generator
        KILL OFFSET(10) NUMBITS(1) [],
        /// Selects the auxiliary clock source, will glitch when switching
        AUXSRC OFFSET(5) NUMBITS(3) [

            CLKSRC_PLL_USB = 0,
            CLKSRC_PLL_SYS = 1,
            ROSC_CLKSRC_PH = 2,
            XOSC_CLKSRC = 3,
            CLKSRC_GPIN0 = 4,
            CLKSRC_GPIN1 = 5
        ]
    ],
    CLK_ADC_CTRL [
        /// An edge on this signal shifts the phase of the output by 1 cycle of the input cl
        /// This can be done at any time
        NUDGE OFFSET(20) NUMBITS(1) [],
        /// This delays the enable signal by up to 3 cycles of the input clock
        /// This must be set before the clock is enabled to have any effect
        PHASE OFFSET(16) NUMBITS(2) [],
        /// Starts and stops the clock generator cleanly
        ENABLE OFFSET(11) NUMBITS(1) [],
        /// Asynchronously kills the clock generator
        KILL OFFSET(10) NUMBITS(1) [],
        /// Selects the auxiliary clock source, will glitch when switching
        AUXSRC OFFSET(5) NUMBITS(3) [

            CLKSRC_PLL_USB = 0,
            CLKSRC_PLL_SYS = 1,
            ROSC_CLKSRC_PH = 2,
            XOSC_CLKSRC = 3,
            CLKSRC_GPIN0 = 4,
            CLKSRC_GPIN1 = 5
        ]
    ],
    CLK_ADC_DIV [
        /// Integer component of the divisor, 0 -> divide by 2^16
        INT OFFSET(8) NUMBITS(2) []
    ],
    CLK_ADC_SELECTED [
        VALUE OFFSET (0) NUMBITS (32) []
    ],
    DFTCLK_XOSC_CTRL [
        SRC OFFSET(0) NUMBITS(2) [
            NULL = 0,
            CLKSRC_PLL_USB_PRIMARY = 1,
            CLKSRC_GPIN0 = 2
        ]
    ],
    /// Clock divisor, can be changed on-the-fly
    DFTCLK_ROSC_CTRL [
        SRC OFFSET(0) NUMBITS(2) [
            NULL = 0,
            CLKSRC_PLL_SYS_PRIMARY_ROSC = 1,
            CLKSRC_GPIN1 = 2
        ]
    ],
    DFTCLK_LPOSC_CTRL [
        SRC OFFSET(0) NUMBITS(2) [
            NULL = 0,
            CLKSRC_PLL_USB_PRIMARY_LPOSC = 1,
            CLKSRC_GPIN1 = 2
        ]
    ],

    CLK_SYS_RESUS_CTRL [
        /// For clearing the resus after the fault that triggered it has been corrected
        CLEAR OFFSET(16) NUMBITS(1) [],
        /// Force a resus, for test purposes only
        FRCE OFFSET(12) NUMBITS(1) [],
        /// Enable resus
        ENABLE OFFSET(8) NUMBITS(1) [],
        /// This is expressed as a number of clk_ref cycles
        /// and must be >= 2x clk_ref_freq/min_clk_tst_freq
        TIMEOUT OFFSET(0) NUMBITS(8) []
    ],
    CLK_SYS_RESUS_STATUS [
        /// Clock has been resuscitated, correct the error then send ctrl_clear=1
        RESUSSED OFFSET(0) NUMBITS(1) []
    ],
    FC0_REF_KHZ [

        FC0_REF_KHZ OFFSET(0) NUMBITS(20) []
    ],
    FC0_MIN_KHZ [

        FC0_MIN_KHZ OFFSET(0) NUMBITS(25) []
    ],
    FC0_MAX_KHZ [

        FC0_MAX_KHZ OFFSET(0) NUMBITS(25) []
    ],
    FC0_DELAY [

        FC0_DELAY OFFSET(0) NUMBITS(3) []
    ],
    FC0_INTERVAL [

        FC0_INTERVAL OFFSET(0) NUMBITS(4) []
    ],
    FC0_SRC [

        FC0_SRC OFFSET(0) NUMBITS(8) [

            NULL = 0,
        ]
    ],
    FC0_STATUS [
        /// Test clock stopped during test
        DIED OFFSET(28) NUMBITS(1) [],
        /// Test clock faster than expected, only valid when status_done=1
        FAST OFFSET(24) NUMBITS(1) [],
        /// Test clock slower than expected, only valid when status_done=1
        SLOW OFFSET(20) NUMBITS(1) [],
        /// Test failed
        FAIL OFFSET(16) NUMBITS(1) [],
        /// Waiting for test clock to start
        WAITING OFFSET(12) NUMBITS(1) [],
        /// Test running
        RUNNING OFFSET(8) NUMBITS(1) [],
        /// Test complete
        DONE OFFSET(4) NUMBITS(1) [],
        /// Test passed
        PASS OFFSET(0) NUMBITS(1) []
    ],
    FC0_RESULT [

        KHZ OFFSET(5) NUMBITS(25) [],

        FRAC OFFSET(0) NUMBITS(5) []
    ],
    WAKE_EN0 [

        clk_sys_sio OFFSET(31) NUMBITS(1) [],

        clk_sys_sha256 OFFSET(30) NUMBITS(1) [],

        clk_sys_psm OFFSET(29) NUMBITS(1) [],

        clk_sys_rosc OFFSET(28) NUMBITS(1) [],

        clk_sys_rom OFFSET(27) NUMBITS(1) [],

        clk_sys_resets OFFSET(26) NUMBITS(1) [],

        clk_sys_pwm OFFSET(25) NUMBITS(1) [],

        clk_sys_powman OFFSET(24) NUMBITS(1) [],

        clk_ref_powman OFFSET(23) NUMBITS(1) [],

        clk_sys_pll_sys_usb OFFSET(22) NUMBITS(1) [],

        clk_sys_pll_sys OFFSET(21) NUMBITS(1) [],

        clk_sys_pio2 OFFSET(20) NUMBITS(1) [],

        clk_sys_pio1 OFFSET(19) NUMBITS(1) [],

        clk_sys_pio0 OFFSET(18) NUMBITS(1) [],

        clk_sys_pads OFFSET(17) NUMBITS(1) [],

        clk_sys_otp OFFSET(16) NUMBITS(1) [],

        clk_sys_ref_otp OFFSET(15) NUMBITS(1) [],

        clk_sys_jtag OFFSET(14) NUMBITS(1) [],

        clk_sys_io OFFSET(13) NUMBITS(1) [],

        clk_sys_i2c1 OFFSET(12) NUMBITS(1) [],

        clk_sys_i2c0 OFFSET(11) NUMBITS(1) [],

        clk_sys_hstx OFFSET(10) NUMBITS(1) [],

        clk_hstx OFFSET(9) NUMBITS(1) [],

        clk_sys_glitch_detector OFFSET(8) NUMBITS(1) [],

        clk_sys_dma OFFSET(7) NUMBITS(1) [],

        clk_sys_busfabric OFFSET(6) NUMBITS(1) [],

        clk_sys_busctrl OFFSET(5) NUMBITS(1) [],

        clk_sys_bootram OFFSET(4) NUMBITS(1) [],

        clk_sys_adc OFFSET(3) NUMBITS(1) [],

        clk_adc_adc OFFSET(2) NUMBITS(1) [],

        clk_sys_accessctrl OFFSET(1) NUMBITS(1) [],

        clk_sys_clocks OFFSET(0) NUMBITS(1) []
    ],
    WAKE_EN1 [
        clk_sys_xosc OFFSET(30) NUMBITS(1) [],

        clk_sys_xip OFFSET(29) NUMBITS(1) [],

        clk_sys_watchdog OFFSET(28) NUMBITS(1) [],

        clk_usb OFFSET(27) NUMBITS(1) [],

        clk_sys_usbctrl OFFSET(26) NUMBITS(1) [],

        clk_sys_uart1 OFFSET(25) NUMBITS(1) [],

        clk_peri_uart1 OFFSET(24) NUMBITS(1) [],

        clk_sys_uart0 OFFSET(23) NUMBITS(1) [],

        clk_peri_uart0 OFFSET(22) NUMBITS(1) [],

        clk_sys_trng OFFSET(21) NUMBITS(1) [],

        clk_sys_timer1 OFFSET(20) NUMBITS(1) [],

        clk_sys_timer0 OFFSET(19) NUMBITS(1) [],

        clk_sys_ticks OFFSET(18) NUMBITS(1) [],
        clk_ref_ticks OFFSET(17) NUMBITS(1) [],

        clk_sys_tbman OFFSET(16) NUMBITS(1) [],

        clk_sys_sysinfo OFFSET(15) NUMBITS(1) [],

        clk_sys_syscfg OFFSET(14) NUMBITS(1) [],
        clk_sys_sram9 OFFSET(13) NUMBITS(1) [],
        clk_sys_sram8 OFFSET(12) NUMBITS(1) [],
        clk_sys_sram7 OFFSET(11) NUMBITS(1) [],
        clk_sys_sram6 OFFSET(10) NUMBITS(1) [],
        clk_sys_sram5 OFFSET(9) NUMBITS(1) [],
        clk_sys_sram4 OFFSET(8) NUMBITS(1) [],
        clk_sys_sram3 OFFSET(7) NUMBITS(1) [],
        clk_sys_sram2 OFFSET(6) NUMBITS(1) [],
        clk_sys_sram1 OFFSET(5) NUMBITS(1) [],
        clk_sys_sram0 OFFSET(4) NUMBITS(1) [],
        clk_sys_spi1 OFFSET(3) NUMBITS(1) [],
        clk_peri_spi1 OFFSET(2) NUMBITS(1) [],
        clk_sys_spi0 OFFSET(1) NUMBITS(1) [],
        clk_peri_spi0 OFFSET(0) NUMBITS(1) []
    ],
    SLEEP_EN0 [
    clk_sys_sio OFFSET(31) NUMBITS(1) [],
    clk_sys_sha256 OFFSET(30) NUMBITS(1) [],
    clk_sys_psm OFFSET(29) NUMBITS(1) [],

        clk_sys_rosc OFFSET(28) NUMBITS(1) [],

        clk_sys_rom OFFSET(27) NUMBITS(1) [],

        clk_sys_resets OFFSET(26) NUMBITS(1) [],
        clk_sys_pwm OFFSET(25) NUMBITS(1) [],
        clk_sys_powman OFFSET(24) NUMBITS(1) [],
        clk_ref_powman OFFSET(23) NUMBITS(1) [],
        clk_sys_pll_usb OFFSET(22) NUMBITS(1) [],
        clk_sys_pll_sys OFFSET(21) NUMBITS(1) [],
        clk_sys_pio2 OFFSET(20) NUMBITS(1) [],
        clk_sys_pio1 OFFSET(19) NUMBITS(1) [],
        clk_sys_pio0 OFFSET(18) NUMBITS(1) [],

        clk_sys_pads OFFSET(17) NUMBITS(1) [],

        clk_sys_otp OFFSET(16) NUMBITS(1) [],
        clk_ref_otp OFFSET(15) NUMBITS(1) [],
        clk_sys_jtag OFFSET(14) NUMBITS(1) [],
        clk_sys_io OFFSET(13) NUMBITS(1) [],
        clk_sys_i2c1 OFFSET(12) NUMBITS(1) [],
        clk_sys_i2c0 OFFSET(11) NUMBITS(1) [],
        clk_sys_hstx OFFSET(10) NUMBITS(1) [],
        clk_hstx OFFSET(9) NUMBITS(1) [],
        clk_sys_glitch_detector OFFSET(8) NUMBITS(1) [],
        clk_sys_dma OFFSET(7) NUMBITS(1) [],
        clk_sys_busfabric OFFSET(6) NUMBITS(1) [],
        clk_sys_busctrl OFFSET(5) NUMBITS(1) [],
        clk_sys_bootram OFFSET(4) NUMBITS(1) [],
        clk_sys_adc OFFSET(3) NUMBITS(1) [],
        clk_adc_adc OFFSET(2) NUMBITS(1) [],
        clk_sys_accessctrl OFFSET(1) NUMBITS(1) [],
        clk_sys_clocks OFFSET(0) NUMBITS(1) [],
    ],
    SLEEP_EN1 [
    clk_sys_xosc OFFSET(30) NUMBITS(1) [],

        clk_sys_xip OFFSET(29) NUMBITS(1) [],

        clk_sys_watchdog OFFSET(28) NUMBITS(1) [],
        clk_usb OFFSET(27) NUMBITS(1) [],
        clk_sys_usbctrl OFFSET(26) NUMBITS(1) [],

        clk_sys_uart1 OFFSET(25) NUMBITS(1) [],

        clk_peri_uart1 OFFSET(24) NUMBITS(1) [],

        clk_sys_uart0 OFFSET(23) NUMBITS(1) [],

        clk_peri_uart0 OFFSET(22) NUMBITS(1) [],
        clk_sys_trng OFFSET(21) NUMBITS(1) [],
        clk_sys_timer1 OFFSET(20) NUMBITS(1) [],
        clk_sys_timer0 OFFSET(19) NUMBITS(1) [],
        clk_sys_ticks OFFSET(18) NUMBITS(1) [],
        clk_ref_ticks OFFSET(17) NUMBITS(1) [],

        clk_sys_tbman OFFSET(16) NUMBITS(1) [],

        clk_sys_sysinfo OFFSET(15) NUMBITS(1) [],

        clk_sys_syscfg OFFSET(14) NUMBITS(1) [],
        clk_sys_sram9 OFFSET(13) NUMBITS(1) [],
        clk_sys_sram8 OFFSET(12) NUMBITS(1) [],
        clk_sys_sram7 OFFSET(11) NUMBITS(1) [],
        clk_sys_sram6 OFFSET(10) NUMBITS(1) [],
        clk_sys_sram5 OFFSET(9) NUMBITS(1) [],
        clk_sys_sram4 OFFSET(8) NUMBITS(1) []
        clk_sys_sram3 OFFSET(7) NUMBITS(1) [],

        clk_sys_sram2 OFFSET(6) NUMBITS(1) [],

        clk_sys_sram1 OFFSET(5) NUMBITS(1) [],

        clk_sys_sram0 OFFSET(4) NUMBITS(1) [],
        clk_sys_spi1 OFFSET(3) NUMBITS(1) [],

        clk_peri_spi1 OFFSET(2) NUMBITS(1) [],

        clk_sys_spi0 OFFSET(1) NUMBITS(1) [],

        clk_peri_spi0 OFFSET(0) NUMBITS(1) [],
    ],
    ENABLED0 [
        clk_sys_sio OFFSET(31) NUMBITS(1) [],

        clk_sys_sha256 OFFSET(30) NUMBITS(1) [],

        clk_sys_PSM OFFSET(29) NUMBITS(1) [],

        clk_sys_rosc OFFSET(28) NUMBITS(1) [],

        clk_sys_rom OFFSET(27) NUMBITS(1) [],

        clk_sys_resets OFFSET(26) NUMBITS(1) [],

        clk_sys_pwm OFFSET(25) NUMBITS(1) [],

        clk_sys_powman OFFSET(24) NUMBITS(1) [],

        clk_ref_powman OFFSET(23) NUMBITS(1) [],

        clk_sys_pll_usb OFFSET(22) NUMBITS(1) [],

        clk_sys_pll_sys OFFSET(21) NUMBITS(1) [],
        clk_sys_pio2 OFFSET(20) NUMBITS(1) [],
        clk_sys_pio1 OFFSET(19) NUMBITS(1) [],

        clk_sys_pio0 OFFSET(18) NUMBITS(1) [],

        clk_sys_pads OFFSET(17) NUMBITS(1) [],

        clk_sys_otp OFFSET(16) NUMBITS(1) [],

        clk_ref_otp OFFSET(15) NUMBITS(1) [],

        clk_sys_jtag OFFSET(14) NUMBITS(1) [],
        clk_sys_io OFFSET(13) NUMBITS(1) [],

        clk_sys_i2c1 OFFSET(12) NUMBITS(1) [],

        clk_sys_i2c0 OFFSET(11) NUMBITS(1) [],
        clk_sys_hstx OFFSET(10) NUMBITS(1) [],
        clk_hstx OFFSET(9) NUMBITS(1) [],

        clk_sys_glitch_detector OFFSET(8) NUMBITS(1) [],

        clk_sys_dma OFFSET(7) NUMBITS(1) [],

        clk_sys_busfabric OFFSET(6) NUMBITS(1) [],

        clk_sys_busctrl OFFSET(5) NUMBITS(1) [],
        clk_sys_bootram OFFSET(4) NUMBITS(1) [],

        clk_sys_adc OFFSET(3) NUMBITS(1) [],

        clk_adc_adc OFFSET(2) NUMBITS(1) [],
        clk_sys_accessctrl OFFSET(1) NUMBITS(1) [],
        clk_sys_clocks OFFSET(0) NUMBITS(1) []
    ],
    ENABLED1 [
        clk_sys_xosc OFFSET(30) NUMBITS(1) [],

        clk_sys_xip OFFSET(29) NUMBITS(1) [],

        clk_sys_watchdog OFFSET(28) NUMBITS(1) [],
        clk_usb OFFSET(27) NUMBITS(1) [],
        clk_sys_usbctrl OFFSET(26) NUMBITS(1) [],

        clk_sys_uart1 OFFSET(25) NUMBITS(1) [],

        clk_peri_uart1 OFFSET(24) NUMBITS(1) [],

        clk_sys_uart0 OFFSET(23) NUMBITS(1) [],

        clk_peri_uart0 OFFSET(22) NUMBITS(1) [],
        clk_sys_trng OFFSET(21) NUMBITS(1) [],
        clk_sys_timer1 OFFSET(20) NUMBITS(1) [],
        clk_sys_timer0 OFFSET(19) NUMBITS(1) [],
        clk_sys_ticks OFFSET(18) NUMBITS(1) [],
        clk_ref_ticks OFFSET(17) NUMBITS(1) [],

        clk_sys_tbman OFFSET(16) NUMBITS(1) [],

        clk_sys_sysinfo OFFSET(15) NUMBITS(1) [],

        clk_sys_syscfg OFFSET(14) NUMBITS(1) [],
        clk_sys_sram9 OFFSET(13) NUMBITS(1) [],
        clk_sys_sram8 OFFSET(12) NUMBITS(1) [],
        clk_sys_sram7 OFFSET(11) NUMBITS(1) [],
        clk_sys_sram6 OFFSET(10) NUMBITS(1) [],
        clk_sys_sram5 OFFSET(9) NUMBITS(1) [],
        clk_sys_sram4 OFFSET(8) NUMBITS(1) []
        clk_sys_sram3 OFFSET(7) NUMBITS(1) [],

        clk_sys_sram2 OFFSET(6) NUMBITS(1) [],

        clk_sys_sram1 OFFSET(5) NUMBITS(1) [],

        clk_sys_sram0 OFFSET(4) NUMBITS(1) [],
        clk_sys_spi1 OFFSET(3) NUMBITS(1) [],

        clk_peri_spi1 OFFSET(2) NUMBITS(1) [],

        clk_sys_spi0 OFFSET(1) NUMBITS(1) [],

        clk_peri_spi0 OFFSET(0) NUMBITS(1) [],
    ],
    INTR [
        CLK_SYS_RESUS OFFSET(0) NUMBITS(1) []
    ],
    INTE [
        CLK_SYS_RESUS OFFSET(0) NUMBITS(1) []
    ],
    INTF [
        CLK_SYS_RESUS OFFSET(0) NUMBITS(1) []
    ],
    INTS [
        CLK_SYS_RESUS OFFSET(0) NUMBITS(1) []
    ]
];

register_bitfields![u32,
    CS [
        /// PLL is locked
        LOCK OFFSET(31) NUMBITS(1) [],
        //  PLL is not locked
        LOCK_N OFFSET(30) NUMBITS(1) [],

        /// Passes the reference clock to the output instead of the divided VCO. The VCO con
        BYPASS OFFSET(8) NUMBITS(1) [],
        /// Divides the PLL input reference clock.
        /// Behaviour is undefined for div=0.
        /// PLL output will be unpredictable during refdiv changes, wait for
        REFDIV OFFSET(0) NUMBITS(6) []
    ],
    PWR [
        /// PLL VCO powerdown
        /// To save power set high when PLL output not required or bypass=1.
        VCOPD OFFSET(5) NUMBITS(1) [],
        /// PLL post divider powerdown
        /// To save power set high when PLL output not required or bypass=1.
        POSTDIVPD OFFSET(3) NUMBITS(1) [],
        /// PLL DSM powerdown
        /// Nothing is achieved by setting this low.
        DSMPD OFFSET(2) NUMBITS(1) [],
        /// PLL powerdown
        /// To save power set high when PLL output not required.
        PD OFFSET(0) NUMBITS(1) []
    ],
    FBDIV_INT [
        /// see ctrl reg description for constraints
        FBDIV_INT OFFSET(0) NUMBITS(12) []
    ],
    PRIM [
        /// divide by 1-7
        POSTDIV1 OFFSET(16) NUMBITS(3) [],
        /// divide by 1-7
        POSTDIV2 OFFSET(12) NUMBITS(3) []
    ],
    INTR [
       LOCK_N_STICKY OFFSET(0) NUMBITS(1) []
    ],
    INTE [
       LOCK_N_STICKY OFFSET(0) NUMBITS(1) []
    ],
    INTF [
       LOCK_N_STICKY OFFSET(0) NUMBITS(1) []
    ],
    INTS [
       LOCK_N_STICKY OFFSET(0) NUMBITS(1) []
    ],


];

const PLL_SYS_BASE: StaticRef<PllRegisters> =
    unsafe { StaticRef::new(0x40050000 as *const PllRegisters) };

const PLL_USB_BASE: StaticRef<PllRegisters> =
    unsafe { StaticRef::new(0x40058000 as *const PllRegisters) };

const CLOCKS_BASE: StaticRef<ClocksRegisters> =
    unsafe { StaticRef::new(0x40010000 as *const ClocksRegisters) };
const XOSC_BASE: StaticRef<XOSCRegisters> =
    unsafe { StaticRef::new(0x40048000 as *const XOSCRegisters) };

    const ROSC_BASE: StaticRef<ROSCRegisters> =
    unsafe { StaticRef::new(0x400e8000 as *const ROSCRegisters) };

    const POWMAN_BASE: StaticRef<POWMANRegisters> =
    unsafe { StaticRef::new(0x40100000 as *const POWMANRegisters) };

//    const TICKS_BASE: StaticRef<TICKRegisters> =
//    unsafe { StaticRef::new(0x40108000 as *const TICKRegisters) };

const NUM_CLOCKS: usize = 10;

pub struct Clocks {
    registers: StaticRef<ClocksRegisters>,
    pll_registers: &'static [StaticRef<PllRegisters>],
    frequencies: [Cell<u32>; NUM_CLOCKS],
}

pub enum PllClock {
    Sys = 0,
    Usb = 1,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(usize)]
pub enum Clock {
    GpioOut0 = 0,
    GpioOut1 = 1,
    GpioOut2 = 2,
    GpioOut3 = 3,
    Reference = 4,
    System = 5,
    Peripheral = 6,
    Usb = 7,
    Adc = 8,
    Dftclkxosc = 9,
    Dftclkrosc = 10,
    Dftclklosc = 11,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum GpioAuxiliaryClockSource {
    PllSys = 0,
    Gpio0 = 1,
    Gpio1 = 2,
    PllUsb = 3,
    Rsoc = 4,
    Xosc = 5,
    Sys = 6,
    Usb = 7,
    Adc = 8,
    Rtc = 9,
    Ref = 10,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum ReferenceClockSource {
    Rsoc = 0,
    Auxiliary = 1,
    Xosc = 2,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum ReferenceAuxiliaryClockSource {
    PllUsb = 0,
    Gpio0 = 1,
    Gpio1 = 2,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum SystemClockSource {
    Reference = 0,
    Auxiliary = 1,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum SystemAuxiliaryClockSource {
    PllSys = 0,
    PllUsb = 1,
    Rsoc = 2,
    Xsoc = 3,
    Gpio0 = 4,
    Gpio1 = 5,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum PeripheralAuxiliaryClockSource {
    System = 0,
    PllSys = 1,
    PllUsb = 2,
    Rsoc = 3,
    Xsoc = 4,
    Gpio0 = 5,
    Gpio1 = 6,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum UsbAuxiliaryClockSource {
    PllSys = 0,
    PllUsb = 1,
    Rsoc = 2,
    Xsoc = 3,
    Gpio0 = 4,
    Gpio1 = 5,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum AdcAuxiliaryClockSource {
    PllSys = 0,
    PllUsb = 1,
    Rsoc = 2,
    Xsoc = 3,
    Gpio0 = 4,
    Gpio1 = 5,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum RtcAuxiliaryClockSource {
    PllSys = 0,
    PllUsb = 1,
    Rsoc = 2,
    Xsoc = 3,
    Gpio0 = 4,
    Gpio1 = 5,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ClockSource {
    GpioOut,
    Reference(ReferenceClockSource),
    System(SystemClockSource),
    Peripheral,
    Usb,
    Adc,
    Rtc,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ClockAuxiliarySource {
    GpioOut(GpioAuxiliaryClockSource),
    Reference(ReferenceAuxiliaryClockSource),
    System(SystemAuxiliaryClockSource),
    Peripheral(PeripheralAuxiliaryClockSource),
    Usb(UsbAuxiliaryClockSource),
    Adc(AdcAuxiliaryClockSource),
    Rtc(RtcAuxiliaryClockSource),
}

impl Clocks {
    pub const fn new() -> Self {
        Self {
            registers: CLOCKS_BASE,
            pll_registers: &[PLL_SYS_BASE, PLL_USB_BASE],
            frequencies: [
                Cell::new(0),
                Cell::new(0),
                Cell::new(0),
                Cell::new(0),
                Cell::new(0),
                Cell::new(0),
                Cell::new(0),
                Cell::new(0),
                Cell::new(0),
                Cell::new(0),
            ],
        }
    }

    pub fn enable_resus(&self) {
        self.registers
            .clk_sys_resus_ctrl
            .modify(CLK_SYS_RESUS_CTRL::ENABLE::SET);
    }

    pub fn disable_resus(&self) {
        self.registers
            .clk_sys_resus_ctrl
            .modify(CLK_SYS_RESUS_CTRL::ENABLE::CLEAR);
    }

    pub fn disable_sys_aux(&self) {
        self.registers
            .clk_sys_ctrl
            .modify(CLK_SYS_CTRL::SRC::CLK_REF);
        while self
            .registers
            .clk_sys_selected
            .read(CLK_SYS_SELECTED::VALUE)
            != 0x1
        {}
    }

    pub fn disable_ref_aux(&self) {
        self.registers
            .clk_ref_ctrl
            .modify(CLK_REF_CTRL::SRC::ROSC_CLKSRC_PH);
        while self
            .registers
            .clk_ref_selected
            .read(CLK_REF_SELECTED::VALUE)
            != 0x1
        {}
    }

    pub fn pll_init(
        &self,
        clock: PllClock,
        xosc_freq: u32,
        refdiv: u32,
        vco_freq: u32,
        post_div1: u32,
        post_div2: u32,
    ) {
        let registers = self.pll_registers[clock as usize];

        // Turn off PLL
        registers
            .pwr
            .modify(PWR::PD::SET + PWR::DSMPD::SET + PWR::POSTDIVPD::SET + PWR::VCOPD::SET);
        registers.fbdiv_int.modify(FBDIV_INT::FBDIV_INT.val(0));

        let ref_mhz = xosc_freq / refdiv;
        registers.cs.modify(CS::REFDIV.val(refdiv));

        // Calculate feedback divider
        let fbdiv = vco_freq / (ref_mhz * 1000000);

        // Should we use assert instead of if and panic! ?
        if fbdiv < 16 || fbdiv > 320 {
            panic!("Invalid feedback divider number {} not in [16, 320]", fbdiv)
        }

        if post_div1 < 1 || post_div1 > 7 || post_div2 < 1 || post_div2 > 7 {
            panic!(
                "Invalid post_div number {} or {} not in [1, 7]",
                post_div1, post_div2
            );
        }

        if post_div2 > post_div1 {
            panic!(
                "post_div2 must be less than post_div1 ({} >= {})",
                post_div1, post_div2
            );
        }

        if ref_mhz > vco_freq / 16 {
            panic!(
                "ref_mhz must be less than vco_freq / 16 ({} <= {})",
                ref_mhz,
                vco_freq / 16
            );
        }

        // Set feedback divider
        registers.fbdiv_int.modify(FBDIV_INT::FBDIV_INT.val(fbdiv));

        // Turn on PLL
        registers.pwr.modify(PWR::PD::CLEAR + PWR::VCOPD::CLEAR);

        // Wait for PLL to lock
        while !registers.cs.is_set(CS::LOCK) {}

        // Set up post divider
        registers
            .prim
            .modify(PRIM::POSTDIV1.val(post_div1) + PRIM::POSTDIV2.val(post_div2));

        // Turn on post divider
        registers.pwr.modify(PWR::POSTDIVPD::CLEAR);
    }

    pub fn pll_deinit(&self, clock: PllClock) {
        self.pll_registers[clock as usize]
            .pwr
            .modify(PWR::PD::SET + PWR::DSMPD::SET + PWR::POSTDIVPD::SET + PWR::VCOPD::SET);
    }

    pub fn set_frequency(&self, clock: Clock, freq: u32) {
        self.frequencies[clock as usize].set(freq);
    }

    pub fn get_frequency(&self, clock: Clock) -> u32 {
        self.frequencies[clock as usize].get()
    }

    fn set_divider(&self, clock: Clock, div: u32) {
        match clock {
            Clock::GpioOut0 | Clock::GpioOut1 | Clock::GpioOut2 | Clock::GpioOut3 => {
                self.registers.clk_gpio[clock as usize].div.set(div)
            }
            Clock::System => self.registers.clk_sys_div.set(div),
            Clock::Reference => self.registers.clk_ref_div.set(div),
            Clock::Usb => self.registers.clk_usb_div.set(div),
            Clock::Adc => self.registers.clk_adc_div.set(div),

            // Clock::Rtc => self.registers.clk_rtc_div.set(div),
            // Clock::Reference
            _ => panic!("failed to set div"),
        }
    }

    fn get_divider(&self, source_freq: u32, freq: u32) -> u32 {
        // pico-sdk: Div register is 24.8 int.frac divider so multiply by 2^8 (left shift by 8)
        (((source_freq as u64) << 8) / freq as u64) as u32
    }

    #[cfg(any(doc, all(target_arch = "arm", target_os = "none")))]
    #[inline]
    fn loop_3_cycles(&self, clock: Clock) {
        if self.get_frequency(clock) > 0 {
            let _delay_cyc: u32 = self.get_frequency(Clock::System) / self.get_frequency(clock) + 1;
            unsafe {
                use core::arch::asm;
                asm! (
                    "1:",
                    "subs {0}, #1",
                    "bne 1b",
                    in (reg) _delay_cyc
                );
            }
        }
    }

    #[cfg(not(any(doc, all(target_arch = "arm", target_os = "none"))))]
    fn loop_3_cycles(&self, _clock: Clock) {
        unimplemented!()
    }

    pub fn configure_gpio_out(
        &self,
        clock: Clock,
        auxiliary_source: GpioAuxiliaryClockSource,
        source_freq: u32,
        freq: u32,
    ) {
        match clock {
            Clock::GpioOut0 | Clock::GpioOut1 | Clock::GpioOut2 | Clock::GpioOut3 => {
                if freq > source_freq {
                    panic!(
                        "freq is greater than source freq ({} > {})",
                        freq, source_freq
                    );
                }

                let div = self.get_divider(source_freq, freq);

                // pico-sdk:
                // If increasing divisor, set divisor before source. Otherwise set source
                // before divisor. This avoids a momentary overspeed when e.g. switching
                // to a faster source and increasing divisor to compensate.
                if div > self.registers.clk_gpio[clock as usize].div.get() {
                    self.set_divider(clock, div);
                }

                self.registers.clk_gpio[clock as usize]
                    .ctrl
                    .modify(CLK_GPOUTx_CTRL::ENABLE::CLEAR);
                // pico-sdk:
                // Delay for 3 cycles of the target clock, for ENABLE propagation.
                // Note XOSC_COUNT is not helpful here because XOSC is not
                // necessarily running, nor is timer... so, 3 cycles per loop:
                self.loop_3_cycles(clock);

                self.registers.clk_gpio[clock as usize]
                    .ctrl
                    .modify(CLK_GPOUTx_CTRL::AUXSRC.val(auxiliary_source as u32));

                self.registers.clk_gpio[clock as usize]
                    .ctrl
                    .modify(CLK_GPOUTx_CTRL::ENABLE::SET);

                // pico-sdk:
                // Now that the source is configured, we can trust that the user-supplied
                // divisor is a safe value.
                self.set_divider(clock, div);

                self.set_frequency(clock, freq);
            }
            _ => panic!("trying to set a non gpio clock"),
        }
    }

    pub fn configure_system(
        &self,
        source: SystemClockSource,
        auxiliary_source: SystemAuxiliaryClockSource,
        source_freq: u32,
        freq: u32,
    ) {
        if freq > source_freq {
            panic!(
                "freq is greater than source freq ({} > {})",
                freq, source_freq
            );
        }
        let div = self.get_divider(source_freq, freq);

        // pico-sdk:
        // If increasing divisor, set divisor before source. Otherwise set source
        // before divisor. This avoids a momentary overspeed when e.g. switching
        // to a faster source and increasing divisor to compensate.
        if div > self.registers.clk_sys_div.get() {
            self.set_divider(Clock::System, div);
        }

        // pico-sdk:
        // If switching a glitchless slice (ref or sys) to an aux source, switch
        // away from aux *first* to avoid passing glitches when changing aux mux.
        // Assume (!!!) glitchless source 0 is no faster than the aux source.
        if source == SystemClockSource::Auxiliary {
            self.registers
                .clk_sys_ctrl
                .modify(CLK_SYS_CTRL::SRC::CLK_REF);
            while self
                .registers
                .clk_sys_selected
                .read(CLK_SYS_SELECTED::VALUE)
                != 0x1
            {}
        }

        self.registers
            .clk_sys_ctrl
            .modify(CLK_SYS_CTRL::AUXSRC.val(auxiliary_source as u32));
        self.registers
            .clk_sys_ctrl
            .modify(CLK_SYS_CTRL::SRC.val(source as u32));
        while self
            .registers
            .clk_sys_selected
            .read(CLK_SYS_SELECTED::VALUE)
            & (1 << (source as u32))
            == 0x0
        {}

        // pico-sdk:
        // Now that the source is configured, we can trust that the user-supplied
        // divisor is a safe value.
        self.set_divider(Clock::System, div);

        self.set_frequency(Clock::System, freq);
    }

    pub fn configure_reference(
        &self,
        source: ReferenceClockSource,
        auxiliary_source: ReferenceAuxiliaryClockSource,
        source_freq: u32,
        freq: u32,
    ) {
        if freq > source_freq {
            panic!(
                "freq is greater than source freq ({} > {})",
                freq, source_freq
            );
        }
        let div = self.get_divider(source_freq, freq);

        // pico-sdk:
        // If increasing divisor, set divisor before source. Otherwise set source
        // before divisor. This avoids a momentary overspeed when e.g. switching
        // to a faster source and increasing divisor to compensate.
        if div > self.registers.clk_ref_div.get() {
            self.set_divider(Clock::Reference, div);
        }

        // pico-sdk:
        // If switching a glitchless slice (ref or sys) to an aux source, switch
        // away from aux *first* to avoid passing glitches when changing aux mux.
        // Assume (!!!) glitchless source 0 is no faster than the aux source.
        if source == ReferenceClockSource::Auxiliary {
            self.registers
                .clk_ref_ctrl
                .modify(CLK_REF_CTRL::SRC::ROSC_CLKSRC_PH);
            while self
                .registers
                .clk_ref_selected
                .read(CLK_REF_SELECTED::VALUE)
                != 0x1
            {}
        }

        self.registers
            .clk_ref_ctrl
            .modify(CLK_REF_CTRL::AUXSRC.val(auxiliary_source as u32));
        self.registers
            .clk_ref_ctrl
            .modify(CLK_REF_CTRL::SRC.val(source as u32));
        while self
            .registers
            .clk_ref_selected
            .read(CLK_REF_SELECTED::VALUE)
            & (1 << (source as u32))
            == 0x0
        {}

        // pico-sdk:
        // Now that the source is configured, we can trust that the user-supplied
        // divisor is a safe value.
        self.set_divider(Clock::Reference, div);

        self.set_frequency(Clock::Reference, freq);
    }

    pub fn configure_peripheral(
        &self,
        auxiliary_source: PeripheralAuxiliaryClockSource,
        freq: u32,
    ) {
        self.registers
            .clk_peri_ctrl
            .modify(CLK_PERI_CTRL::ENABLE::CLEAR);

        // pico-sdk:
        // Delay for 3 cycles of the target clock, for ENABLE propagation.
        // Note XOSC_COUNT is not helpful here because XOSC is not
        // necessarily running, nor is timer... so, 3 cycles per loop:
        self.loop_3_cycles(Clock::Peripheral);

        self.registers
            .clk_peri_ctrl
            .modify(CLK_PERI_CTRL::AUXSRC.val(auxiliary_source as u32));

        self.registers
            .clk_peri_ctrl
            .modify(CLK_PERI_CTRL::ENABLE::SET);

        self.set_frequency(Clock::Peripheral, freq);
    }

    pub fn configure_usb(
        &self,
        auxiliary_source: UsbAuxiliaryClockSource,
        source_freq: u32,
        freq: u32,
    ) {
        if freq > source_freq {
            panic!(
                "freq is greater than source freq ({} > {})",
                freq, source_freq
            );
        }
        let div = self.get_divider(source_freq, freq);

        // pico-sdk:
        // If increasing divisor, set divisor before source. Otherwise set source
        // before divisor. This avoids a momentary overspeed when e.g. switching
        // to a faster source and increasing divisor to compensate.
        if div > self.registers.clk_usb_div.get() {
            self.set_divider(Clock::Usb, div);
        }

        self.registers
            .clk_usb_ctrl
            .modify(CLK_USB_CTRL::ENABLE::CLEAR);
        // pico-sdk:
        // Delay for 3 cycles of the target clock, for ENABLE propagation.
        // Note XOSC_COUNT is not helpful here because XOSC is not
        // necessarily running, nor is timer... so, 3 cycles per loop:
        self.loop_3_cycles(Clock::Usb);

        self.registers
            .clk_usb_ctrl
            .modify(CLK_USB_CTRL::AUXSRC.val(auxiliary_source as u32));

        self.registers
            .clk_usb_ctrl
            .modify(CLK_USB_CTRL::ENABLE::SET);

        // pico-sdk:
        // Now that the source is configured, we can trust that the user-supplied
        // divisor is a safe value.
        self.set_divider(Clock::Usb, div);

        self.set_frequency(Clock::Usb, freq);
    }

    pub fn configure_adc(
        &self,
        auxiliary_source: AdcAuxiliaryClockSource,
        source_freq: u32,
        freq: u32,
    ) {
        if freq > source_freq {
            panic!(
                "freq is greater than source freq ({} > {})",
                freq, source_freq
            );
        }
        let div = self.get_divider(source_freq, freq);

        // pico-sdk:
        // If increasing divisor, set divisor before source. Otherwise set source
        // before divisor. This avoids a momentary overspeed when e.g. switching
        // to a faster source and increasing divisor to compensate.
        if div > self.registers.clk_adc_div.get() {
            self.set_divider(Clock::Adc, div);
        }

        self.registers
            .clk_adc_ctrl
            .modify(CLK_ADC_CTRL::ENABLE::CLEAR);
        // pico-sdk:
        // Delay for 3 cycles of the target clock, for ENABLE propagation.
        // Note XOSC_COUNT is not helpful here because XOSC is not
        // necessarily running, nor is timer... so, 3 cycles per loop:
        self.loop_3_cycles(Clock::Adc);

        self.registers
            .clk_adc_ctrl
            .modify(CLK_ADC_CTRL::AUXSRC.val(auxiliary_source as u32));

        self.registers
            .clk_adc_ctrl
            .modify(CLK_ADC_CTRL::ENABLE::SET);

        // pico-sdk:
        // Now that the source is configured, we can trust that the user-supplied
        // divisor is a safe value.
        self.set_divider(Clock::Adc, div);

        self.set_frequency(Clock::Adc, freq);
    }
}
