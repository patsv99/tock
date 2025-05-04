// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

//! GPIO, RP2350
//!
//! ### Author
//! * Ioana Culic <ioana.culic@wyliodrin.com>

use enum_primitive::cast::FromPrimitive;
use enum_primitive::enum_from_primitive;
use kernel::hil;
// use kernel::hil::gpio;
use kernel::utilities::cells::OptionalCell;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, register_structs, ReadOnly, ReadWrite};
use kernel::utilities::StaticRef;

use crate::chip::Processor;
#[repr(C)]
struct GpioPin {
    status: ReadOnly<u32, GPIOx_STATUS::Register>,
    ctrl: ReadWrite<u32, GPIOx_CTRL::Register>,
}
#[repr(C)]
struct GpioProc {
    enable: [ReadWrite<u32, GPIO_INTxx::Register>; 6],
    force: [ReadWrite<u32, GPIO_INTxx::Register>; 6],
    status: [ReadWrite<u32, GPIO_INTxx::Register>; 6],
}

register_structs! {
    // GPIO Registers.
    GpioRegisters {
        (0x000 => pin: [GpioPin; 48]),
        (0x180 => fill: [ReadWrite<u32>; 32]),

        /// Raw interrupts
        (0x200 => irqsummary_proc0_secure0 : ReadWrite<u32,IRQSUMMARY_PROC0_SECURE0::Register>),
        (0x204 => irqsummary_proc0_secure1 : ReadWrite<u32,IRQSUMMARY_PROC0_SECURE1::Register>),
        (0x208 => irqsummary_proc0_nsecure0 : ReadWrite<u32,IRQSUMMARY_PROC0_NSECURE0::Register>),
        (0x20c => irqsummary_proc0_nsecure1 : ReadWrite<u32,IRQSUMMARY_PROC0_SECURE1::Register>),
        (0x210 => irqsummary_proc1_secure0 : ReadWrite<u32,IRQSUMMARY_PROC1_SECURE0::Register>),
        (0x214 => irqsummary_proc1_secure1 : ReadWrite<u32,IRQSUMMARY_PROC1_SECURE1::Register>),
        (0x218 => irqsummary_proc1_nsecure0 : ReadWrite<u32,IRQSUMMARY_PROC1_NSECURE0::Register>),
        (0x21c => irqsummary_proc1_nsecure1 : ReadWrite<u32,IRQSUMMARY_PROC1_NSECURE1::Register>),
        (0x220 => irqsummary_coma_wake_secure0 : ReadWrite<u32,IRQSUMMARY_COMA_WAKE_SECURE0::Register>),
        (0x224 => irqsummary_coma_wake_secure1 : ReadWrite<u32,IRQSUMMARY_COMA_WAKE_SECURE1::Register>),
        (0x228 => irqsummary_coma_wake_nsecure0 : ReadWrite<u32,IRQSUMMARY_COMA_WAKE_NSECURE0::Register>),
        (0x22c => irqsummary_coma_wake_nsecure1 : ReadWrite<u32,IRQSUMMARY_COMA_WAKE_NSECURE1::Register>),
        (0x230 => intr : [ReadWrite<u32,INTR_0::Register>; 6]),
        /*
        // More detailed version
        (0x230 => intr0 : ReadWrite<u32,INTR_0::Register>),
        (0x234 => intr1 : ReadWrite<u32,INTR_1::Register>),
        (0x238 => intr2 : ReadWrite<u32,INTR_2::Register>),
        (0x23c => intr3 : ReadWrite<u32,INTR_3::Register>),
        (0x240 => intr4 : ReadWrite<u32,INTR_4::Register>),
        (0x244 => intr5 : ReadWrite<u32,INTR_5::Register>),
*/
        /// Interrupts for procs
        (0x248 => interrupt_proc: [GpioProc; 2]),

/*
        (0x24c => proc0_inte1: ReadWrite<u32,INTR_1::Register>),
        (0x250 => proc0_inte2: ReadWrite<u32,INTR_2::Register>),
        (0x254 => proc0_inte3: ReadWrite<u32,INTR_3::Register>),
        (0x258 => proc0_inte4: ReadWrite<u32,INTR_4::Register>),
        (0x25c => proc0_inte5: ReadWrite<u32,INTR_5::Register>),

        (0x260 => proc0_intf0: ReadWrite<u32,INTR_1::Register>),
        (0x264 => proc0_intf1: ReadWrite<u32,INTR_1::Register>),
        (0x268 => proc0_intf2: ReadWrite<u32,INTR_2::Register>),
        (0x26c => proc0_intf3: ReadWrite<u32,INTR_3::Register>),
        (0x270 => proc0_intf4: ReadWrite<u32,INTR_4::Register>),
        (0x274 => proc0_intf5: ReadWrite<u32,INTR_5::Register>),

        (0x278 => proc0_ints0: ReadWrite<u32,INTR_1::Register>),
        (0x27c => proc0_ints1: ReadWrite<u32,INTR_1::Register>),
        (0x280 => proc0_ints2: ReadWrite<u32,INTR_2::Register>),
        (0x284 => proc0_ints3: ReadWrite<u32,INTR_3::Register>),
        (0x288 => proc0_ints4: ReadWrite<u32,INTR_4::Register>),
        (0x28c => proc0_ints5: ReadWrite<u32,INTR_5::Register>),

        (0x290 => proc1_inte0: ReadWrite<u32,INTR_0::Register>),
        (0x294 => proc1_inte1: ReadWrite<u32,INTR_1::Register>),
        (0x298 => proc1_inte2: ReadWrite<u32,INTR_2::Register>),
        (0x29c => proc1_inte3: ReadWrite<u32,INTR_3::Register>),
        (0x2a0 => proc1_inte4: ReadWrite<u32,INTR_4::Register>),
        (0x2a4 => proc1_inte5: ReadWrite<u32,INTR_5::Register>),

        (0x2a8 => proc1_intf0: ReadWrite<u32,INTR_1::Register>),
        (0x2ac => proc1_intf1: ReadWrite<u32,INTR_1::Register>),
        (0x2b0 => proc1_intf2: ReadWrite<u32,INTR_2::Register>),
        (0x2b4 => proc1_intf3: ReadWrite<u32,INTR_3::Register>),
        (0x2b8 => proc1_intf4: ReadWrite<u32,INTR_4::Register>),
        (0x2bc => proc1_intf5: ReadWrite<u32,INTR_5::Register>),

        (0x2c0 => proc1_ints0: ReadWrite<u32,INTR_1::Register>),
        (0x2c4 => proc1_ints1: ReadWrite<u32,INTR_1::Register>),
        (0x2c8 => proc1_ints2: ReadWrite<u32,INTR_2::Register>),
        (0x2cc => proc1_ints3: ReadWrite<u32,INTR_3::Register>),
        (0x2d0 => proc1_ints4: ReadWrite<u32,INTR_4::Register>),
        (0x2d4 => proc1_ints5: ReadWrite<u32,INTR_5::Register>),
*/
        /// Wake
        (0x2d8 => dormant_inte0: ReadWrite<u32,INTR_0::Register>),
        (0x2dc => dormant_inte1: ReadWrite<u32,INTR_1::Register>),
        (0x2e0 => dormant_inte2: ReadWrite<u32,INTR_2::Register>),
        (0x2e4 => dormant_inte3: ReadWrite<u32,INTR_3::Register>),
        (0x2e8 => dormant_inte4: ReadWrite<u32,INTR_4::Register>),
        (0x2ec => dormant_inte5: ReadWrite<u32,INTR_5::Register>),

        (0x2f0 => dormant_intf0: ReadWrite<u32,INTR_0::Register>),
        (0x2f4 => dormant_intf1: ReadWrite<u32,INTR_1::Register>),
        (0x2f8 => dormant_intf2: ReadWrite<u32,INTR_2::Register>),
        (0x2fc => dormant_intf3: ReadWrite<u32,INTR_3::Register>),
        (0x300 => dormant_intf4: ReadWrite<u32,INTR_4::Register>),
        (0x304 => dormant_intf5: ReadWrite<u32,INTR_5::Register>),

        (0x308 => dormant_ints0: ReadWrite<u32,INTR_0::Register>),
        (0x30c => dormant_ints1: ReadWrite<u32,INTR_1::Register>),
        (0x310 => dormant_ints2: ReadWrite<u32,INTR_2::Register>),
        (0x314 => dormant_ints3: ReadWrite<u32,INTR_3::Register>),
        (0x318 => dormant_ints4: ReadWrite<u32,INTR_4::Register>),
        (0x31c => dormant_ints5: ReadWrite<u32,INTR_5::Register>),

        /// End
        (0x320 => @END),
    },
    // User Bank Pad Control Registers
    GpioPadRegisters {
        /// Voltage select
        (0x00 => voltage: ReadWrite<u32, VOLTAGE_SELECT::Register>),

        /// Pads control
        (0x04 => gpio_pad: [ReadWrite<u32, GPIO_PAD::Register>; 48]),
        (0xc4 => swclk : ReadWrite<u32,SWCLK::Register>),
        (0xc8 => swd : ReadWrite<u32,SWD::Register>),
        /// End
        (0xcc => @END),
    },
    /// SIO Control Registers
    SIORegisters {
        /// Not used
        (0x000 => cpuid: ReadOnly<u32, CPUID::Register>),

        /// Input value for GPIO pins
        (0x004 => gpio_in: ReadOnly<u32, GPIO_IN::Register>),


        (0x008 => gpio_hi_in: ReadOnly<u32,GPIO_HI_IN::Register>),
        (0x00c => fill: ReadOnly<u32>),

        /// GPIO output value
        (0x010 => gpio_out: ReadWrite<u32, GPIO_OUT::Register>),
        /// GPIO hi output value
        (0x014 => gpio_hi_out: ReadWrite<u32, GPIO_HI_OUT::Register>),


        /// GPIO output value set
        (0x018 => gpio_out_set: ReadWrite<u32, GPIO_OUT_SET::Register>),

        (0x01c => gpio_hi_out_set: ReadWrite<u32, GPIO_HI_OUT_SET::Register>),


        /// GPIO output value clear
        (0x020 => gpio_out_clr: ReadWrite<u32, GPIO_OUT_CLR::Register>),

        (0x024 => gpio_hi_out_clr: ReadWrite<u32, GPIO_HI_OUT_CLR::Register>),

        /// GPIO output value XOR
        (0x028 => gpio_out_xor: ReadWrite<u32, GPIO_OUT_XOR::Register>),
        (0x02c => gpio_hi_out_xor: ReadWrite<u32, GPIO_HI_OUT_XOR::Register>),

        /// GPIO output enable
        (0x030 => gpio_oe: ReadWrite<u32, GPIO_OE::Register>),
        (0x034 => gpio_hi_oe: ReadWrite<u32, GPIO_HI_OE::Register>),

        /// GPIO output enable set
        (0x038 => gpio_oe_set: ReadWrite<u32, GPIO_OE_SET::Register>),
        (0x03c => gpio_hi_oe_set: ReadWrite<u32, GPIO_HI_OE_SET::Register>),

        /// GPIO output enable clear
        (0x040 => gpio_oe_clr: ReadWrite<u32, GPIO_OE_CLR::Register>),
        (0x044 => gpio_hi_oe_clr: ReadWrite<u32, GPIO_HI_OE_CLR::Register>),

        (0x048 => gpio_oe_xor: ReadWrite<u32, GPIO_OE_XOR::Register>),
        (0x04c => gpio_hi_oe_xor: ReadWrite<u32, GPIO_HI_OE_XOR::Register>),


        /// FIFO status
        (0x050 => fifo_st: ReadWrite<u32, FIFO_ST::Register>),

        /// FIFO write
        (0x054 => fifo_wr: ReadWrite<u32, FIFO_WR::Register>),

        /// FIFO read
        (0x058 => fifo_rd: ReadOnly<u32, FIFO_RD::Register>),

        /// End
        (0x05c => @END),
        // TODO many registers left...
    }
}

register_bitfields![u32,
    GPIOx_STATUS [
        /// interrupt to processors, after override is applied
        IRQTOPROC OFFSET(26) NUMBITS(1) [],
        /// interrupt from pad before override is applied
        IRQFROMPAD OFFSET(24) NUMBITS(1) [],
        /// input signal to peripheral, after override is applied
        INTOPERI OFFSET(19) NUMBITS(1) [],
        /// input signal from pad, before override is applied
        INFROMPAD OFFSET(17) NUMBITS(1) [],
        /// output enable to pad after register override is applied
        OETOPAD OFFSET(13) NUMBITS(1) [],
        /// output enable from selected peripheral, before registeroverride is applied
        OEFROMPERI OFFSET(12) NUMBITS(1) [],
        /// output signal to pad after register override is applied
        OUTTOPAD OFFSET(9) NUMBITS(1) [],
        /// output signal from selected peripheral, before registeroverride is applied
        OUTFROMPERI OFFSET(8) NUMBITS(1) []
    ],
    GPIOx_CTRL [
        /// interrupt override?
        IRQOVER OFFSET(28) NUMBITS(2) [
            NoInvert = 0,
            Invert = 1,
            DriveLow = 2,
            DriveHigh = 3
        ],
        /// input override
        INOVER OFFSET(16) NUMBITS(2) [
            NoInvert = 0,
            Invert = 1,
            DriveLow = 2,
            DriveHigh = 3
        ],
        /// output enable override
        OEOVER OFFSET(12) NUMBITS(2) [
            EnableSignal = 0,
            EnableInverseSignal = 1,
            Disable = 2,
            Enable = 3
        ],
        /// output override
        OUTOVER OFFSET(8) NUMBITS(2) [
            Signal = 0,
            InverseSignal = 1,
            Low = 2,
            High = 3
        ],
        /// Function select
        FUNCSEL OFFSET(0) NUMBITS(5) [
            GPIO_FUNC_XIP = 0,
            GPIO_FUNC_SPI = 1,
            GPIO_FUNC_UART = 2,
            GPIO_FUNC_I2C = 3,
            GPIO_FUNC_PWM = 4,
            GPIO_FUNC_SIO = 5,
            GPIO_FUNC_PIO0 = 6,
            GPIO_FUNC_PIO1 = 7,
            GPIO_FUNC_GPCK = 8,
            GPIO_FUNC_USB = 9,
            GPIO_FUNC_NULL = 0x1f
        ]
    ],
    IRQSUMMARY_PROC0_SECURE0 [
    GPIO31 OFFSET(31) NUMBITS(1) [],
    GPIO30 OFFSET(30) NUMBITS(1) [],
    GPIO29 OFFSET(29) NUMBITS(1) [],
    GPIO28 OFFSET(28) NUMBITS(1) [],
    GPIO27 OFFSET(27) NUMBITS(1) [],
    GPIO26 OFFSET(26) NUMBITS(1) [],
    GPIO25 OFFSET(25) NUMBITS(1) [],
    GPIO24 OFFSET(24) NUMBITS(1) [],
    GPIO23 OFFSET(23) NUMBITS(1) [],
    GPIO22 OFFSET(22) NUMBITS(1) [],
    GPIO21 OFFSET(21) NUMBITS(1) [],
    GPIO20 OFFSET(20) NUMBITS(1) [],
    GPIO19 OFFSET(19) NUMBITS(1) [],
    GPIO18 OFFSET(18) NUMBITS(1) [],
    GPIO17 OFFSET(17) NUMBITS(1) [],
    GPIO16 OFFSET(16) NUMBITS(1) [],
    GPIO15 OFFSET(15) NUMBITS(1) [],
    GPIO14 OFFSET(14) NUMBITS(1) [],
    GPIO13 OFFSET(13) NUMBITS(1) [],
    GPIO12 OFFSET(12) NUMBITS(1) [],
    GPIO11 OFFSET(11) NUMBITS(1) [],
    GPIO10 OFFSET(10) NUMBITS(1) [],
    GPIO09 OFFSET(09) NUMBITS(1) [],
    GPIO08 OFFSET(08) NUMBITS(1) [],
    GPIO07 OFFSET(07) NUMBITS(1) [],
    GPIO06 OFFSET(06) NUMBITS(1) [],
    GPIO05 OFFSET(05) NUMBITS(1) [],
    GPIO04 OFFSET(04) NUMBITS(1) [],
    GPIO03 OFFSET(03) NUMBITS(1) [],
    GPIO02 OFFSET(02) NUMBITS(1) [],
    GPIO01 OFFSET(01) NUMBITS(1) [],
    GPIO00 OFFSET(00) NUMBITS(1) [],
  ],
    IRQSUMMARY_PROC0_SECURE1 [
    GPIO47 OFFSET(15) NUMBITS(1) [],
    GPIO46 OFFSET(14) NUMBITS(1) [],
    GPIO45 OFFSET(13) NUMBITS(1) [],
    GPIO44 OFFSET(12) NUMBITS(1) [],
    GPIO43 OFFSET(11) NUMBITS(1) [],
    GPIO42 OFFSET(10) NUMBITS(1) [],
    GPIO41 OFFSET(09) NUMBITS(1) [],
    GPIO40 OFFSET(08) NUMBITS(1) [],
    GPIO39 OFFSET(07) NUMBITS(1) [],
    GPIO38 OFFSET(06) NUMBITS(1) [],
    GPIO37 OFFSET(05) NUMBITS(1) [],
    GPIO36 OFFSET(04) NUMBITS(1) [],
    GPIO35 OFFSET(03) NUMBITS(1) [],
    GPIO34 OFFSET(02) NUMBITS(1) [],
    GPIO33 OFFSET(01) NUMBITS(1) [],
    GPIO32 OFFSET(00) NUMBITS(1) [],
],
    IRQSUMMARY_PROC0_NSECURE0 [
    GPIO31 OFFSET(31) NUMBITS(1) [],
    GPIO30 OFFSET(30) NUMBITS(1) [],
    GPIO29 OFFSET(29) NUMBITS(1) [],
    GPIO28 OFFSET(28) NUMBITS(1) [],
    GPIO27 OFFSET(27) NUMBITS(1) [],
    GPIO26 OFFSET(26) NUMBITS(1) [],
    GPIO25 OFFSET(25) NUMBITS(1) [],
    GPIO24 OFFSET(24) NUMBITS(1) [],
    GPIO23 OFFSET(23) NUMBITS(1) [],
    GPIO22 OFFSET(22) NUMBITS(1) [],
    GPIO21 OFFSET(21) NUMBITS(1) [],
    GPIO20 OFFSET(20) NUMBITS(1) [],
    GPIO19 OFFSET(19) NUMBITS(1) [],
    GPIO18 OFFSET(18) NUMBITS(1) [],
    GPIO17 OFFSET(17) NUMBITS(1) [],
    GPIO16 OFFSET(16) NUMBITS(1) [],
    GPIO15 OFFSET(15) NUMBITS(1) [],
    GPIO14 OFFSET(14) NUMBITS(1) [],
    GPIO13 OFFSET(13) NUMBITS(1) [],
    GPIO12 OFFSET(12) NUMBITS(1) [],
    GPIO11 OFFSET(11) NUMBITS(1) [],
    GPIO10 OFFSET(10) NUMBITS(1) [],
    GPIO09 OFFSET(09) NUMBITS(1) [],
    GPIO08 OFFSET(08) NUMBITS(1) [],
    GPIO07 OFFSET(07) NUMBITS(1) [],
    GPIO06 OFFSET(06) NUMBITS(1) [],
    GPIO05 OFFSET(05) NUMBITS(1) [],
    GPIO04 OFFSET(04) NUMBITS(1) [],
    GPIO03 OFFSET(03) NUMBITS(1) [],
    GPIO02 OFFSET(02) NUMBITS(1) [],
    GPIO01 OFFSET(01) NUMBITS(1) [],
    GPIO00 OFFSET(00) NUMBITS(1) [],
  ],
    IRQSUMMARY_PROC0_NSECURE1 [
    GPIO47 OFFSET(15) NUMBITS(1) [],
    GPIO46 OFFSET(14) NUMBITS(1) [],
    GPIO45 OFFSET(13) NUMBITS(1) [],
    GPIO44 OFFSET(12) NUMBITS(1) [],
    GPIO43 OFFSET(11) NUMBITS(1) [],
    GPIO42 OFFSET(10) NUMBITS(1) [],
    GPIO41 OFFSET(09) NUMBITS(1) [],
    GPIO40 OFFSET(08) NUMBITS(1) [],
    GPIO39 OFFSET(07) NUMBITS(1) [],
    GPIO38 OFFSET(06) NUMBITS(1) [],
    GPIO37 OFFSET(05) NUMBITS(1) [],
    GPIO36 OFFSET(04) NUMBITS(1) [],
    GPIO35 OFFSET(03) NUMBITS(1) [],
    GPIO34 OFFSET(02) NUMBITS(1) [],
    GPIO33 OFFSET(01) NUMBITS(1) [],
    GPIO32 OFFSET(00) NUMBITS(1) [],
],
   IRQSUMMARY_PROC1_SECURE0 [
    GPIO31 OFFSET(31) NUMBITS(1) [],
    GPIO30 OFFSET(30) NUMBITS(1) [],
    GPIO29 OFFSET(29) NUMBITS(1) [],
    GPIO28 OFFSET(28) NUMBITS(1) [],
    GPIO27 OFFSET(27) NUMBITS(1) [],
    GPIO26 OFFSET(26) NUMBITS(1) [],
    GPIO25 OFFSET(25) NUMBITS(1) [],
    GPIO24 OFFSET(24) NUMBITS(1) [],
    GPIO23 OFFSET(23) NUMBITS(1) [],
    GPIO22 OFFSET(22) NUMBITS(1) [],
    GPIO21 OFFSET(21) NUMBITS(1) [],
    GPIO20 OFFSET(20) NUMBITS(1) [],
    GPIO19 OFFSET(19) NUMBITS(1) [],
    GPIO18 OFFSET(18) NUMBITS(1) [],
    GPIO17 OFFSET(17) NUMBITS(1) [],
    GPIO16 OFFSET(16) NUMBITS(1) [],
    GPIO15 OFFSET(15) NUMBITS(1) [],
    GPIO14 OFFSET(14) NUMBITS(1) [],
    GPIO13 OFFSET(13) NUMBITS(1) [],
    GPIO12 OFFSET(12) NUMBITS(1) [],
    GPIO11 OFFSET(11) NUMBITS(1) [],
    GPIO10 OFFSET(10) NUMBITS(1) [],
    GPIO09 OFFSET(09) NUMBITS(1) [],
    GPIO08 OFFSET(08) NUMBITS(1) [],
    GPIO07 OFFSET(07) NUMBITS(1) [],
    GPIO06 OFFSET(06) NUMBITS(1) [],
    GPIO05 OFFSET(05) NUMBITS(1) [],
    GPIO04 OFFSET(04) NUMBITS(1) [],
    GPIO03 OFFSET(03) NUMBITS(1) [],
    GPIO02 OFFSET(02) NUMBITS(1) [],
    GPIO01 OFFSET(01) NUMBITS(1) [],
    GPIO00 OFFSET(00) NUMBITS(1) [],
  ],
    IRQSUMMARY_PROC1_SECURE1 [
    GPIO47 OFFSET(15) NUMBITS(1) [],
    GPIO46 OFFSET(14) NUMBITS(1) [],
    GPIO45 OFFSET(13) NUMBITS(1) [],
    GPIO44 OFFSET(12) NUMBITS(1) [],
    GPIO43 OFFSET(11) NUMBITS(1) [],
    GPIO42 OFFSET(10) NUMBITS(1) [],
    GPIO41 OFFSET(09) NUMBITS(1) [],
    GPIO40 OFFSET(08) NUMBITS(1) [],
    GPIO39 OFFSET(07) NUMBITS(1) [],
    GPIO38 OFFSET(06) NUMBITS(1) [],
    GPIO37 OFFSET(05) NUMBITS(1) [],
    GPIO36 OFFSET(04) NUMBITS(1) [],
    GPIO35 OFFSET(03) NUMBITS(1) [],
    GPIO34 OFFSET(02) NUMBITS(1) [],
    GPIO33 OFFSET(01) NUMBITS(1) [],
    GPIO32 OFFSET(00) NUMBITS(1) [],
],
    IRQSUMMARY_PROC1_NSECURE0 [
    GPIO31 OFFSET(31) NUMBITS(1) [],
    GPIO30 OFFSET(30) NUMBITS(1) [],
    GPIO29 OFFSET(29) NUMBITS(1) [],
    GPIO28 OFFSET(28) NUMBITS(1) [],
    GPIO27 OFFSET(27) NUMBITS(1) [],
    GPIO26 OFFSET(26) NUMBITS(1) [],
    GPIO25 OFFSET(25) NUMBITS(1) [],
    GPIO24 OFFSET(24) NUMBITS(1) [],
    GPIO23 OFFSET(23) NUMBITS(1) [],
    GPIO22 OFFSET(22) NUMBITS(1) [],
    GPIO21 OFFSET(21) NUMBITS(1) [],
    GPIO20 OFFSET(20) NUMBITS(1) [],
    GPIO19 OFFSET(19) NUMBITS(1) [],
    GPIO18 OFFSET(18) NUMBITS(1) [],
    GPIO17 OFFSET(17) NUMBITS(1) [],
    GPIO16 OFFSET(16) NUMBITS(1) [],
    GPIO15 OFFSET(15) NUMBITS(1) [],
    GPIO14 OFFSET(14) NUMBITS(1) [],
    GPIO13 OFFSET(13) NUMBITS(1) [],
    GPIO12 OFFSET(12) NUMBITS(1) [],
    GPIO11 OFFSET(11) NUMBITS(1) [],
    GPIO10 OFFSET(10) NUMBITS(1) [],
    GPIO09 OFFSET(09) NUMBITS(1) [],
    GPIO08 OFFSET(08) NUMBITS(1) [],
    GPIO07 OFFSET(07) NUMBITS(1) [],
    GPIO06 OFFSET(06) NUMBITS(1) [],
    GPIO05 OFFSET(05) NUMBITS(1) [],
    GPIO04 OFFSET(04) NUMBITS(1) [],
    GPIO03 OFFSET(03) NUMBITS(1) [],
    GPIO02 OFFSET(02) NUMBITS(1) [],
    GPIO01 OFFSET(01) NUMBITS(1) [],
    GPIO00 OFFSET(00) NUMBITS(1) [],
    ],

    IRQSUMMARY_PROC1_NSECURE1 [
    GPIO47 OFFSET(15) NUMBITS(1) [],
    GPIO46 OFFSET(14) NUMBITS(1) [],
    GPIO45 OFFSET(13) NUMBITS(1) [],
    GPIO44 OFFSET(12) NUMBITS(1) [],
    GPIO43 OFFSET(11) NUMBITS(1) [],
    GPIO42 OFFSET(10) NUMBITS(1) [],
    GPIO41 OFFSET(09) NUMBITS(1) [],
    GPIO40 OFFSET(08) NUMBITS(1) [],
    GPIO39 OFFSET(07) NUMBITS(1) [],
    GPIO38 OFFSET(06) NUMBITS(1) [],
    GPIO37 OFFSET(05) NUMBITS(1) [],
    GPIO36 OFFSET(04) NUMBITS(1) [],
    GPIO35 OFFSET(03) NUMBITS(1) [],
    GPIO34 OFFSET(02) NUMBITS(1) [],
    GPIO33 OFFSET(01) NUMBITS(1) [],
    GPIO32 OFFSET(00) NUMBITS(1) [],
    ],

    IRQSUMMARY_COMA_WAKE_SECURE0 [
    GPIO31 OFFSET(31) NUMBITS(1) [],
    GPIO30 OFFSET(30) NUMBITS(1) [],
    GPIO29 OFFSET(29) NUMBITS(1) [],
    GPIO28 OFFSET(28) NUMBITS(1) [],
    GPIO27 OFFSET(27) NUMBITS(1) [],
    GPIO26 OFFSET(26) NUMBITS(1) [],
    GPIO25 OFFSET(25) NUMBITS(1) [],
    GPIO24 OFFSET(24) NUMBITS(1) [],
    GPIO23 OFFSET(23) NUMBITS(1) [],
    GPIO22 OFFSET(22) NUMBITS(1) [],
    GPIO21 OFFSET(21) NUMBITS(1) [],
    GPIO20 OFFSET(20) NUMBITS(1) [],
    GPIO19 OFFSET(19) NUMBITS(1) [],
    GPIO18 OFFSET(18) NUMBITS(1) [],
    GPIO17 OFFSET(17) NUMBITS(1) [],
    GPIO16 OFFSET(16) NUMBITS(1) [],
    GPIO15 OFFSET(15) NUMBITS(1) [],
    GPIO14 OFFSET(14) NUMBITS(1) [],
    GPIO13 OFFSET(13) NUMBITS(1) [],
    GPIO12 OFFSET(12) NUMBITS(1) [],
    GPIO11 OFFSET(11) NUMBITS(1) [],
    GPIO10 OFFSET(10) NUMBITS(1) [],
    GPIO09 OFFSET(09) NUMBITS(1) [],
    GPIO08 OFFSET(08) NUMBITS(1) [],
    GPIO07 OFFSET(07) NUMBITS(1) [],
    GPIO06 OFFSET(06) NUMBITS(1) [],
    GPIO05 OFFSET(05) NUMBITS(1) [],
    GPIO04 OFFSET(04) NUMBITS(1) [],
    GPIO03 OFFSET(03) NUMBITS(1) [],
    GPIO02 OFFSET(02) NUMBITS(1) [],
    GPIO01 OFFSET(01) NUMBITS(1) [],
    GPIO00 OFFSET(00) NUMBITS(1) [],
    ],
    IRQSUMMARY_COMA_WAKE_SECURE1 [
    GPIO47 OFFSET(15) NUMBITS(1) [],
    GPIO46 OFFSET(14) NUMBITS(1) [],
    GPIO45 OFFSET(13) NUMBITS(1) [],
    GPIO44 OFFSET(12) NUMBITS(1) [],
    GPIO43 OFFSET(11) NUMBITS(1) [],
    GPIO42 OFFSET(10) NUMBITS(1) [],
    GPIO41 OFFSET(09) NUMBITS(1) [],
    GPIO40 OFFSET(08) NUMBITS(1) [],
    GPIO39 OFFSET(07) NUMBITS(1) [],
    GPIO38 OFFSET(06) NUMBITS(1) [],
    GPIO37 OFFSET(05) NUMBITS(1) [],
    GPIO36 OFFSET(04) NUMBITS(1) [],
    GPIO35 OFFSET(03) NUMBITS(1) [],
    GPIO34 OFFSET(02) NUMBITS(1) [],
    GPIO33 OFFSET(01) NUMBITS(1) [],
    GPIO32 OFFSET(00) NUMBITS(1) [],
    ],

    IRQSUMMARY_COMA_WAKE_NSECURE0 [
    GPIO31 OFFSET(31) NUMBITS(1) [],
    GPIO30 OFFSET(30) NUMBITS(1) [],
    GPIO29 OFFSET(29) NUMBITS(1) [],
    GPIO28 OFFSET(28) NUMBITS(1) [],
    GPIO27 OFFSET(27) NUMBITS(1) [],
    GPIO26 OFFSET(26) NUMBITS(1) [],
    GPIO25 OFFSET(25) NUMBITS(1) [],
    GPIO24 OFFSET(24) NUMBITS(1) [],
    GPIO23 OFFSET(23) NUMBITS(1) [],
    GPIO22 OFFSET(22) NUMBITS(1) [],
    GPIO21 OFFSET(21) NUMBITS(1) [],
    GPIO20 OFFSET(20) NUMBITS(1) [],
    GPIO19 OFFSET(19) NUMBITS(1) [],
    GPIO18 OFFSET(18) NUMBITS(1) [],
    GPIO17 OFFSET(17) NUMBITS(1) [],
    GPIO16 OFFSET(16) NUMBITS(1) [],
    GPIO15 OFFSET(15) NUMBITS(1) [],
    GPIO14 OFFSET(14) NUMBITS(1) [],
    GPIO13 OFFSET(13) NUMBITS(1) [],
    GPIO12 OFFSET(12) NUMBITS(1) [],
    GPIO11 OFFSET(11) NUMBITS(1) [],
    GPIO10 OFFSET(10) NUMBITS(1) [],
    GPIO09 OFFSET(09) NUMBITS(1) [],
    GPIO08 OFFSET(08) NUMBITS(1) [],
    GPIO07 OFFSET(07) NUMBITS(1) [],
    GPIO06 OFFSET(06) NUMBITS(1) [],
    GPIO05 OFFSET(05) NUMBITS(1) [],
    GPIO04 OFFSET(04) NUMBITS(1) [],
    GPIO03 OFFSET(03) NUMBITS(1) [],
    GPIO02 OFFSET(02) NUMBITS(1) [],
    GPIO01 OFFSET(01) NUMBITS(1) [],
    GPIO00 OFFSET(00) NUMBITS(1) [],
    ],
    IRQSUMMARY_COMA_WAKE_NSECURE1 [
    GPIO47 OFFSET(15) NUMBITS(1) [],
    GPIO46 OFFSET(14) NUMBITS(1) [],
    GPIO45 OFFSET(13) NUMBITS(1) [],
    GPIO44 OFFSET(12) NUMBITS(1) [],
    GPIO43 OFFSET(11) NUMBITS(1) [],
    GPIO42 OFFSET(10) NUMBITS(1) [],
    GPIO41 OFFSET(09) NUMBITS(1) [],
    GPIO40 OFFSET(08) NUMBITS(1) [],
    GPIO39 OFFSET(07) NUMBITS(1) [],
    GPIO38 OFFSET(06) NUMBITS(1) [],
    GPIO37 OFFSET(05) NUMBITS(1) [],
    GPIO36 OFFSET(04) NUMBITS(1) [],
    GPIO35 OFFSET(03) NUMBITS(1) [],
    GPIO34 OFFSET(02) NUMBITS(1) [],
    GPIO33 OFFSET(01) NUMBITS(1) [],
    GPIO32 OFFSET(00) NUMBITS(1) [],
    ],
INTR_0 [
        GPIO7_EDGE_HIGH OFFSET(31) NUMBITS(1) [],
        GPIO7_EDGE_LOW OFFSET(30) NUMBITS(1) [],
        GPIO7_LEVEL_HIGH OFFSET(29) NUMBITS(1) [],
        GPIO7_LEVEL_LOW OFFSET(28) NUMBITS(1) [],

        GPIO6_EDGE_HIGH OFFSET(27) NUMBITS(1) [],
        GPIO6_EDGE_LOW OFFSET(26) NUMBITS(1) [],
        GPIO6_LEVEL_HIGH OFFSET(25) NUMBITS(1) [],
        GPIO6_LEVEL_LOW OFFSET(24) NUMBITS(1) [],

        GPIO5_EDGE_HIGH OFFSET(23) NUMBITS(1) [],
        GPIO5_EDGE_LOW OFFSET(22) NUMBITS(1) [],
        GPIO5_LEVEL_HIGH OFFSET(21) NUMBITS(1) [],
        GPIO5_LEVEL_LOW OFFSET(20) NUMBITS(1) [],

        GPIO4_EDGE_HIGH OFFSET(19) NUMBITS(1) [],
        GPIO4_EDGE_LOW OFFSET(18) NUMBITS(1) [],
        GPIO4_LEVEL_HIGH OFFSET(17) NUMBITS(1) [],
        GPIO4_LEVEL_LOW OFFSET(16) NUMBITS(1) [],

        GPIO3_EDGE_HIGH OFFSET(15) NUMBITS(1) [],
        GPIO3_EDGE_LOW OFFSET(14) NUMBITS(1) [],
        GPIO3_LEVEL_HIGH OFFSET(13) NUMBITS(1) [],
        GPIO3_LEVEL_LOW OFFSET(12) NUMBITS(1) [],

        GPIO2_EDGE_HIGH OFFSET(11) NUMBITS(1) [],
        GPIO2_EDGE_LOW OFFSET(10) NUMBITS(1) [],
        GPIO2_LEVEL_HIGH OFFSET(9) NUMBITS(1) [],
        GPIO2_LEVEL_LOW OFFSET(8) NUMBITS(1) [],

        GPIO1_EDGE_HIGH OFFSET(7) NUMBITS(1) [],
        GPIO1_EDGE_LOW OFFSET(6) NUMBITS(1) [],
        GPIO1_LEVEL_HIGH OFFSET(5) NUMBITS(1) [],
        GPIO1_LEVEL_LOW OFFSET(4) NUMBITS(1) [],

        GPIO0_EDGE_HIGH OFFSET(3) NUMBITS(1) [],
        GPIO0_EDGE_LOW OFFSET(2) NUMBITS(1) [],
        GPIO0_LEVEL_HIGH OFFSET(1) NUMBITS(1) [],
        GPIO0_LEVEL_LOW OFFSET(0) NUMBITS(1) []
],
INTR_1 [
        GPIO15_EDGE_HIGH OFFSET(31) NUMBITS(1) [],
        GPIO15_EDGE_LOW OFFSET(30) NUMBITS(1) [],
        GPIO15_LEVEL_HIGH OFFSET(29) NUMBITS(1) [],
        GPIO15_LEVEL_LOW OFFSET(28) NUMBITS(1) [],

        GPIO14_EDGE_HIGH OFFSET(27) NUMBITS(1) [],
        GPIO14_EDGE_LOW OFFSET(26) NUMBITS(1) [],
        GPIO14_LEVEL_HIGH OFFSET(25) NUMBITS(1) [],
        GPIO14_LEVEL_LOW OFFSET(24) NUMBITS(1) [],

        GPIO13_EDGE_HIGH OFFSET(23) NUMBITS(1) [],
        GPIO13_EDGE_LOW OFFSET(22) NUMBITS(1) [],
        GPIO13_LEVEL_HIGH OFFSET(21) NUMBITS(1) [],
        GPIO13_LEVEL_LOW OFFSET(20) NUMBITS(1) [],

        GPIO12_EDGE_HIGH OFFSET(19) NUMBITS(1) [],
        GPIO12_EDGE_LOW OFFSET(18) NUMBITS(1) [],
        GPIO12_LEVEL_HIGH OFFSET(17) NUMBITS(1) [],
        GPIO12_LEVEL_LOW OFFSET(16) NUMBITS(1) [],

        GPIO11_EDGE_HIGH OFFSET(15) NUMBITS(1) [],
        GPIO11_EDGE_LOW OFFSET(14) NUMBITS(1) [],
        GPIO11_LEVEL_HIGH OFFSET(13) NUMBITS(1) [],
        GPIO11_LEVEL_LOW OFFSET(12) NUMBITS(1) [],

        GPIO10_EDGE_HIGH OFFSET(11) NUMBITS(1) [],
        GPIO10_EDGE_LOW OFFSET(10) NUMBITS(1) [],
        GPIO10_LEVEL_HIGH OFFSET(9) NUMBITS(1) [],
        GPIO10_LEVEL_LOW OFFSET(8) NUMBITS(1) [],

        GPIO09_EDGE_HIGH OFFSET(7) NUMBITS(1) [],
        GPIO09_EDGE_LOW OFFSET(6) NUMBITS(1) [],
        GPIO09_LEVEL_HIGH OFFSET(5) NUMBITS(1) [],
        GPIO09_LEVEL_LOW OFFSET(4) NUMBITS(1) [],

        GPIO08_EDGE_HIGH OFFSET(3) NUMBITS(1) [],
        GPIO08_EDGE_LOW OFFSET(2) NUMBITS(1) [],
        GPIO08_LEVEL_HIGH OFFSET(1) NUMBITS(1) [],
        GPIO08_LEVEL_LOW OFFSET(0) NUMBITS(1) []
],
INTR_2 [
        GPIO23_EDGE_HIGH OFFSET(31) NUMBITS(1) [],
        GPIO23_EDGE_LOW OFFSET(30) NUMBITS(1) [],
        GPIO23_LEVEL_HIGH OFFSET(29) NUMBITS(1) [],
        GPIO23_LEVEL_LOW OFFSET(28) NUMBITS(1) [],

        GPIO22_EDGE_HIGH OFFSET(27) NUMBITS(1) [],
        GPIO22_EDGE_LOW OFFSET(26) NUMBITS(1) [],
        GPIO22_LEVEL_HIGH OFFSET(25) NUMBITS(1) [],
        GPIO22_LEVEL_LOW OFFSET(24) NUMBITS(1) [],

        GPIO21_EDGE_HIGH OFFSET(23) NUMBITS(1) [],
        GPIO21_EDGE_LOW OFFSET(22) NUMBITS(1) [],
        GPIO21_LEVEL_HIGH OFFSET(21) NUMBITS(1) [],
        GPIO21_LEVEL_LOW OFFSET(20) NUMBITS(1) [],

        GPIO20_EDGE_HIGH OFFSET(19) NUMBITS(1) [],
        GPIO20_EDGE_LOW OFFSET(18) NUMBITS(1) [],
        GPIO20_LEVEL_HIGH OFFSET(17) NUMBITS(1) [],
        GPIO20_LEVEL_LOW OFFSET(16) NUMBITS(1) [],

        GPIO19_EDGE_HIGH OFFSET(15) NUMBITS(1) [],
        GPIO19_EDGE_LOW OFFSET(14) NUMBITS(1) [],
        GPIO19_LEVEL_HIGH OFFSET(13) NUMBITS(1) [],
        GPIO19_LEVEL_LOW OFFSET(12) NUMBITS(1) [],

        GPIO18_EDGE_HIGH OFFSET(11) NUMBITS(1) [],
        GPIO18_EDGE_LOW OFFSET(10) NUMBITS(1) [],
        GPIO18_LEVEL_HIGH OFFSET(9) NUMBITS(1) [],
        GPIO18_LEVEL_LOW OFFSET(8) NUMBITS(1) [],

        GPIO17_EDGE_HIGH OFFSET(7) NUMBITS(1) [],
        GPIO17_EDGE_LOW OFFSET(6) NUMBITS(1) [],
        GPIO17_LEVEL_HIGH OFFSET(5) NUMBITS(1) [],
        GPIO17_LEVEL_LOW OFFSET(4) NUMBITS(1) [],

        GPIO16_EDGE_HIGH OFFSET(3) NUMBITS(1) [],
        GPIO16_EDGE_LOW OFFSET(2) NUMBITS(1) [],
        GPIO16_LEVEL_HIGH OFFSET(1) NUMBITS(1) [],
        GPIO16_LEVEL_LOW OFFSET(0) NUMBITS(1) []
],

INTR_3 [
        GPIO31_EDGE_HIGH OFFSET(31) NUMBITS(1) [],
        GPIO31_EDGE_LOW OFFSET(30) NUMBITS(1) [],
        GPIO31_LEVEL_HIGH OFFSET(29) NUMBITS(1) [],
        GPIO31_LEVEL_LOW OFFSET(28) NUMBITS(1) [],

        GPIO30_EDGE_HIGH OFFSET(27) NUMBITS(1) [],
        GPIO30_EDGE_LOW OFFSET(26) NUMBITS(1) [],
        GPIO30_LEVEL_HIGH OFFSET(25) NUMBITS(1) [],
        GPIO30_LEVEL_LOW OFFSET(24) NUMBITS(1) [],

        GPIO29_EDGE_HIGH OFFSET(23) NUMBITS(1) [],
        GPIO29_EDGE_LOW OFFSET(22) NUMBITS(1) [],
        GPIO29_LEVEL_HIGH OFFSET(21) NUMBITS(1) [],
        GPIO29_LEVEL_LOW OFFSET(20) NUMBITS(1) [],

        GPIO28_EDGE_HIGH OFFSET(19) NUMBITS(1) [],
        GPIO28_EDGE_LOW OFFSET(18) NUMBITS(1) [],
        GPIO28_LEVEL_HIGH OFFSET(17) NUMBITS(1) [],
        GPIO28_LEVEL_LOW OFFSET(16) NUMBITS(1) [],

        GPIO27_EDGE_HIGH OFFSET(15) NUMBITS(1) [],
        GPIO27_EDGE_LOW OFFSET(14) NUMBITS(1) [],
        GPIO27_LEVEL_HIGH OFFSET(13) NUMBITS(1) [],
        GPIO27_LEVEL_LOW OFFSET(12) NUMBITS(1) [],

        GPIO26_EDGE_HIGH OFFSET(11) NUMBITS(1) [],
        GPIO26_EDGE_LOW OFFSET(10) NUMBITS(1) [],
        GPIO26_LEVEL_HIGH OFFSET(9) NUMBITS(1) [],
        GPIO26_LEVEL_LOW OFFSET(8) NUMBITS(1) [],

        GPIO25_EDGE_HIGH OFFSET(7) NUMBITS(1) [],
        GPIO25_EDGE_LOW OFFSET(6) NUMBITS(1) [],
        GPIO25_LEVEL_HIGH OFFSET(5) NUMBITS(1) [],
        GPIO25_LEVEL_LOW OFFSET(4) NUMBITS(1) [],

        GPIO24_EDGE_HIGH OFFSET(3) NUMBITS(1) [],
        GPIO24_EDGE_LOW OFFSET(2) NUMBITS(1) [],
        GPIO24_LEVEL_HIGH OFFSET(1) NUMBITS(1) [],
        GPIO24_LEVEL_LOW OFFSET(0) NUMBITS(1) []
],
INTR_4 [
        GPIO39_EDGE_HIGH OFFSET(31) NUMBITS(1) [],
        GPIO39_EDGE_LOW OFFSET(30) NUMBITS(1) [],
        GPIO39_LEVEL_HIGH OFFSET(29) NUMBITS(1) [],
        GPIO39_LEVEL_LOW OFFSET(28) NUMBITS(1) [],

        GPIO38_EDGE_HIGH OFFSET(27) NUMBITS(1) [],
        GPIO38_EDGE_LOW OFFSET(26) NUMBITS(1) [],
        GPIO38_LEVEL_HIGH OFFSET(25) NUMBITS(1) [],
        GPIO38_LEVEL_LOW OFFSET(24) NUMBITS(1) [],

        GPIO37_EDGE_HIGH OFFSET(23) NUMBITS(1) [],
        GPIO37_EDGE_LOW OFFSET(22) NUMBITS(1) [],
        GPIO37_LEVEL_HIGH OFFSET(21) NUMBITS(1) [],
        GPIO37_LEVEL_LOW OFFSET(20) NUMBITS(1) [],

        GPIO36_EDGE_HIGH OFFSET(19) NUMBITS(1) [],
        GPIO36_EDGE_LOW OFFSET(18) NUMBITS(1) [],
        GPIO36_LEVEL_HIGH OFFSET(17) NUMBITS(1) [],
        GPIO36_LEVEL_LOW OFFSET(16) NUMBITS(1) [],

        GPIO35_EDGE_HIGH OFFSET(15) NUMBITS(1) [],
        GPIO35_EDGE_LOW OFFSET(14) NUMBITS(1) [],
        GPIO35_LEVEL_HIGH OFFSET(13) NUMBITS(1) [],
        GPIO35_LEVEL_LOW OFFSET(12) NUMBITS(1) [],

        GPIO34_EDGE_HIGH OFFSET(11) NUMBITS(1) [],
        GPIO34_EDGE_LOW OFFSET(10) NUMBITS(1) [],
        GPIO34_LEVEL_HIGH OFFSET(9) NUMBITS(1) [],
        GPIO34_LEVEL_LOW OFFSET(8) NUMBITS(1) [],

        GPIO33_EDGE_HIGH OFFSET(7) NUMBITS(1) [],
        GPIO33_EDGE_LOW OFFSET(6) NUMBITS(1) [],
        GPIO33_LEVEL_HIGH OFFSET(5) NUMBITS(1) [],
        GPIO33_LEVEL_LOW OFFSET(4) NUMBITS(1) [],

        GPIO32_EDGE_HIGH OFFSET(3) NUMBITS(1) [],
        GPIO32_EDGE_LOW OFFSET(2) NUMBITS(1) [],
        GPIO32_LEVEL_HIGH OFFSET(1) NUMBITS(1) [],
        GPIO32_LEVEL_LOW OFFSET(0) NUMBITS(1) []
],

INTR_5 [
        GPIO47_EDGE_HIGH OFFSET(31) NUMBITS(1) [],
        GPIO47_EDGE_LOW OFFSET(30) NUMBITS(1) [],
        GPIO47_LEVEL_HIGH OFFSET(29) NUMBITS(1) [],
        GPIO47_LEVEL_LOW OFFSET(28) NUMBITS(1) [],

        GPIO46_EDGE_HIGH OFFSET(27) NUMBITS(1) [],
        GPIO46_EDGE_LOW OFFSET(26) NUMBITS(1) [],
        GPIO46_LEVEL_HIGH OFFSET(25) NUMBITS(1) [],
        GPIO46_LEVEL_LOW OFFSET(24) NUMBITS(1) [],

        GPIO45_EDGE_HIGH OFFSET(23) NUMBITS(1) [],
        GPIO45_EDGE_LOW OFFSET(22) NUMBITS(1) [],
        GPIO45_LEVEL_HIGH OFFSET(21) NUMBITS(1) [],
        GPIO45_LEVEL_LOW OFFSET(20) NUMBITS(1) [],

        GPIO44_EDGE_HIGH OFFSET(19) NUMBITS(1) [],
        GPIO44_EDGE_LOW OFFSET(18) NUMBITS(1) [],
        GPIO44_LEVEL_HIGH OFFSET(17) NUMBITS(1) [],
        GPIO44_LEVEL_LOW OFFSET(16) NUMBITS(1) [],

        GPIO43_EDGE_HIGH OFFSET(15) NUMBITS(1) [],
        GPIO43_EDGE_LOW OFFSET(14) NUMBITS(1) [],
        GPIO43_LEVEL_HIGH OFFSET(13) NUMBITS(1) [],
        GPIO43_LEVEL_LOW OFFSET(12) NUMBITS(1) [],

        GPIO42_EDGE_HIGH OFFSET(11) NUMBITS(1) [],
        GPIO42_EDGE_LOW OFFSET(10) NUMBITS(1) [],
        GPIO42_LEVEL_HIGH OFFSET(9) NUMBITS(1) [],
        GPIO42_LEVEL_LOW OFFSET(8) NUMBITS(1) [],

        GPIO41_EDGE_HIGH OFFSET(7) NUMBITS(1) [],
        GPIO41_EDGE_LOW OFFSET(6) NUMBITS(1) [],
        GPIO41_LEVEL_HIGH OFFSET(5) NUMBITS(1) [],
        GPIO41_LEVEL_LOW OFFSET(4) NUMBITS(1) [],

        GPIO40_EDGE_HIGH OFFSET(3) NUMBITS(1) [],
        GPIO40_EDGE_LOW OFFSET(2) NUMBITS(1) [],
        GPIO40_LEVEL_HIGH OFFSET(1) NUMBITS(1) [],
        GPIO40_LEVEL_LOW OFFSET(0) NUMBITS(1) []
],
    GPIO_INTxx [
        GPIO7_EDGE_HIGH OFFSET(31) NUMBITS(1) [],
        GPIO7_EDGE_LOW OFFSET(30) NUMBITS(1) [],
        GPIO7_LEVEL_HIGH OFFSET(29) NUMBITS(1) [],
        GPIO7_LEVEL_LOW OFFSET(28) NUMBITS(1) [],

        GPIO6_EDGE_HIGH OFFSET(27) NUMBITS(1) [],
        GPIO6_EDGE_LOW OFFSET(26) NUMBITS(1) [],
        GPIO6_LEVEL_HIGH OFFSET(25) NUMBITS(1) [],
        GPIO6_LEVEL_LOW OFFSET(24) NUMBITS(1) [],

        GPIO5_EDGE_HIGH OFFSET(23) NUMBITS(1) [],
        GPIO5_EDGE_LOW OFFSET(22) NUMBITS(1) [],
        GPIO5_LEVEL_HIGH OFFSET(21) NUMBITS(1) [],
        GPIO5_LEVEL_LOW OFFSET(20) NUMBITS(1) [],

        GPIO4_EDGE_HIGH OFFSET(19) NUMBITS(1) [],
        GPIO4_EDGE_LOW OFFSET(18) NUMBITS(1) [],
        GPIO4_LEVEL_HIGH OFFSET(17) NUMBITS(1) [],
        GPIO4_LEVEL_LOW OFFSET(16) NUMBITS(1) [],

        GPIO3_EDGE_HIGH OFFSET(15) NUMBITS(1) [],
        GPIO3_EDGE_LOW OFFSET(14) NUMBITS(1) [],
        GPIO3_LEVEL_HIGH OFFSET(13) NUMBITS(1) [],
        GPIO3_LEVEL_LOW OFFSET(12) NUMBITS(1) [],

        GPIO2_EDGE_HIGH OFFSET(11) NUMBITS(1) [],
        GPIO2_EDGE_LOW OFFSET(10) NUMBITS(1) [],
        GPIO2_LEVEL_HIGH OFFSET(9) NUMBITS(1) [],
        GPIO2_LEVEL_LOW OFFSET(8) NUMBITS(1) [],

        GPIO1_EDGE_HIGH OFFSET(7) NUMBITS(1) [],
        GPIO1_EDGE_LOW OFFSET(6) NUMBITS(1) [],
        GPIO1_LEVEL_HIGH OFFSET(5) NUMBITS(1) [],
        GPIO1_LEVEL_LOW OFFSET(4) NUMBITS(1) [],

        GPIO0_EDGE_HIGH OFFSET(3) NUMBITS(1) [],
        GPIO0_EDGE_LOW OFFSET(2) NUMBITS(1) [],
        GPIO0_LEVEL_HIGH OFFSET(1) NUMBITS(1) [],
        GPIO0_LEVEL_LOW OFFSET(0) NUMBITS(1) []
    ],
    VOLTAGE_SELECT[
        VOLTAGE OFFSET(0) NUMBITS(1) [
            Set3V3 = 0,
            Set1V8 = 1
        ]
    ],
    GPIO_PAD [
        ISO OFFSET(8) NUMBITS(1) [],
        OD OFFSET(7) NUMBITS(1) [],
        IE OFFSET(6) NUMBITS(1) [],
        DRIVE OFFSET(4) NUMBITS(2) [],
        PUE OFFSET(3) NUMBITS(1) [],
        PDE OFFSET(2) NUMBITS(1) [],
        SCHMITT OFFSET(1) NUMBITS(1) [],
        SLEWFAST OFFSET(0) NUMBITS(1) []
    ],
    GPIO_IN [
        ///Input value for GPIO0..31
        IN OFFSET(0) NUMBITS(32) []
    ],
    GPIO_HI_IN [
        ///Input value for GPIO32..47
        QSPI_SD OFFSET(28) NUMBITS(4) [],

        QSPI_CSN OFFSET(27) NUMBITS(1) [],

        QSPI_SCK OFFSET(26) NUMBITS(1) [],

        USB_DM OFFSET(25) NUMBITS(1) [],

        USB_DP OFFSET(24) NUMBITS(1) [],
        GPIO OFFSET(0) NUMBITS(16) [],
    ],
    GPIO_OUT [
        ///Set output level (1/0 → high/low) for GPIO0...29.
        OUT OFFSET(0) NUMBITS(32) []
    ],
    GPIO_HI_OUT [
        ///Input value for GPIO32..47
        QSPI_SD OFFSET(28) NUMBITS(4) [],

        QSPI_CSN OFFSET(27) NUMBITS(1) [],

        QSPI_SCK OFFSET(26) NUMBITS(1) [],

        USB_DM OFFSET(25) NUMBITS(1) [],

        USB_DP OFFSET(24) NUMBITS(1) [],
        GPIO OFFSET(0) NUMBITS(16) [],
    ],

    GPIO_OUT_SET [
        ///Perform an atomic bit-set on GPIO_OUT
        OUT OFFSET(0) NUMBITS(30) []
    ],
    GPIO_HI_OUT_SET [
        ///Input value for GPIO32..47
        QSPI_SD OFFSET(28) NUMBITS(4) [],
        QSPI_CSN OFFSET(27) NUMBITS(1) [],
        QSPI_SCK OFFSET(26) NUMBITS(1) [],
        USB_DM OFFSET(25) NUMBITS(1) [],
        USB_DP OFFSET(24) NUMBITS(1) [],
        GPIO OFFSET(0) NUMBITS(16) [],
    ],

    GPIO_OUT_CLR [
        ///Perform an atomic bit-clear on GPIO_OUT
        OUT OFFSET(0) NUMBITS(30) []
    ],
    GPIO_HI_OUT_CLR [
        ///Input value for GPIO32..47
        QSPI_SD OFFSET(28) NUMBITS(4) [],
        QSPI_CSN OFFSET(27) NUMBITS(1) [],
        QSPI_SCK OFFSET(26) NUMBITS(1) [],
        USB_DM OFFSET(25) NUMBITS(1) [],
        USB_DP OFFSET(24) NUMBITS(1) [],
        GPIO OFFSET(0) NUMBITS(16) [],
    ],

    GPIO_OUT_XOR [
        ///Perform an atomic bitwise XOR on GPIO_OUT
        OUT OFFSET(0) NUMBITS(30) []
    ],
    GPIO_HI_OUT_XOR [
        ///Input value for GPIO32..47
        QSPI_SD OFFSET(28) NUMBITS(4) [],
        QSPI_CSN OFFSET(27) NUMBITS(1) [],
        QSPI_SCK OFFSET(26) NUMBITS(1) [],
        USB_DM OFFSET(25) NUMBITS(1) [],
        USB_DP OFFSET(24) NUMBITS(1) [],
        GPIO OFFSET(0) NUMBITS(16) [],
    ],

    GPIO_OE [
        ///Set output enable (1/0 → output/input) for GPIO0...29
        OE OFFSET(0) NUMBITS(30) []
    ],
    GPIO_HI_OE [
        ///Input value for GPIO32..47
        QSPI_SD OFFSET(28) NUMBITS(4) [],
        QSPI_CSN OFFSET(27) NUMBITS(1) [],
        QSPI_SCK OFFSET(26) NUMBITS(1) [],
        USB_DM OFFSET(25) NUMBITS(1) [],
        USB_DP OFFSET(24) NUMBITS(1) [],
        GPIO OFFSET(0) NUMBITS(16) [],
    ],

    GPIO_OE_SET [
        ///Perform an atomic bit-set on GPIO_OE
        OE OFFSET(0) NUMBITS(30) []
    ],
    GPIO_HI_OE_SET [
        ///Input value for GPIO32..47
        QSPI_SD OFFSET(28) NUMBITS(4) [],
        QSPI_CSN OFFSET(27) NUMBITS(1) [],
        QSPI_SCK OFFSET(26) NUMBITS(1) [],
        USB_DM OFFSET(25) NUMBITS(1) [],
        USB_DP OFFSET(24) NUMBITS(1) [],
        GPIO OFFSET(0) NUMBITS(16) [],
    ],

    GPIO_OE_CLR [
        ///Perform an atomic bit-clear on GPIO_OE
        OE OFFSET(0) NUMBITS(30) []
    ],
    GPIO_HI_OE_CLR [
        ///Input value for GPIO32..47
        QSPI_SD OFFSET(28) NUMBITS(4) [],
        QSPI_CSN OFFSET(27) NUMBITS(1) [],
        QSPI_SCK OFFSET(26) NUMBITS(1) [],
        USB_DM OFFSET(25) NUMBITS(1) [],
        USB_DP OFFSET(24) NUMBITS(1) [],
        GPIO OFFSET(0) NUMBITS(16) [],
    ],

    GPIO_OE_XOR [
        ///Perform an atomic bit-clear on GPIO_OE
        OE OFFSET(0) NUMBITS(30) []
    ],
    GPIO_HI_OE_XOR [
        ///Input value for GPIO32..47
        QSPI_SD OFFSET(28) NUMBITS(4) [],
        QSPI_CSN OFFSET(27) NUMBITS(1) [],
        QSPI_SCK OFFSET(26) NUMBITS(1) [],
        USB_DM OFFSET(25) NUMBITS(1) [],
        USB_DP OFFSET(24) NUMBITS(1) [],
        GPIO OFFSET(0) NUMBITS(16) [],
    ],


    CPUID [
        VALUE OFFSET(0) NUMBITS (32)
    ],
    FIFO_ST [
        /// FIFO read when empy
        ROE OFFSET(3) NUMBITS(1) [],
        /// FIFO written when full
        WOF OFFSET(2) NUMBITS(1) [],
        /// FIFO not full
        RDY OFFSET(1) NUMBITS(1) [],
        /// FIFO not empty
        VLD OFFSET(0) NUMBITS(1) []
    ],
    FIFO_WR [
        /// FIFO Write
        VALUE OFFSET(0) NUMBITS(32)
    ],
    FIFO_RD [
        /// FIFO Read
        VALUE OFFSET(0) NUMBITS(32)
    ],
    GPIO_Proc0_0 [
        GPIO0 OFFSET(0) NUMBITS(1),
        GPIO1 OFFSET(1) NUMBITS(1),
        GPIO2 OFFSET(2) NUMBITS(1),
        GPIO3 OFFSET(3) NUMBITS(1),
        GPIO4 OFFSET(4) NUMBITS(1),
        GPIO5 OFFSET(5) NUMBITS(1),
        GPIO6 OFFSET(6) NUMBITS(1),
        GPIO7 OFFSET(7) NUMBITS(1),
        GPIO8 OFFSET(8) NUMBITS(1),
        GPIO9 OFFSET(9) NUMBITS(1),
        GPIO10 OFFSET(10) NUMBITS(1),
        GPIO11 OFFSET(11) NUMBITS(1),
        GPIO12 OFFSET(12) NUMBITS(1),
        GPIO13 OFFSET(13) NUMBITS(1),
        GPIO14 OFFSET(14) NUMBITS(1),
        GPIO15 OFFSET(15) NUMBITS(1),
        GPIO16 OFFSET(16) NUMBITS(1),
        GPIO17 OFFSET(17) NUMBITS(1),
        GPIO18 OFFSET(18) NUMBITS(1),
        GPIO19 OFFSET(19) NUMBITS(1),
        GPIO20 OFFSET(20) NUMBITS(1),
        GPIO21 OFFSET(21) NUMBITS(1),
        GPIO22 OFFSET(22) NUMBITS(1),
        GPIO23 OFFSET(23) NUMBITS(1),
        GPIO24 OFFSET(24) NUMBITS(1),
        GPIO25 OFFSET(25) NUMBITS(1),
        GPIO26 OFFSET(26) NUMBITS(1),
        GPIO27 OFFSET(27) NUMBITS(1),
        GPIO28 OFFSET(28) NUMBITS(1),
        GPIO29 OFFSET(29) NUMBITS(1),
        GPIO30 OFFSET(30) NUMBITS(1),
        GPIO31 OFFSET(31) NUMBITS(1),
    ],
    GPIO_Proc0_1 [
        GPIO32 OFFSET(0) NUMBITS(1),
        GPIO33 OFFSET(1) NUMBITS(1),
        GPIO34 OFFSET(2) NUMBITS(1),
        GPIO35 OFFSET(3) NUMBITS(1),
        GPIO36 OFFSET(4) NUMBITS(1),
        GPIO37 OFFSET(5) NUMBITS(1),
        GPIO38 OFFSET(6) NUMBITS(1),
        GPIO39 OFFSET(7) NUMBITS(1),
        GPIO40 OFFSET(8) NUMBITS(1),
        GPIO41 OFFSET(9) NUMBITS(1),
        GPIO42 OFFSET(10) NUMBITS(1),
        GPIO43 OFFSET(11) NUMBITS(1),
        GPIO44 OFFSET(12) NUMBITS(1),
        GPIO45 OFFSET(13) NUMBITS(1),
        GPIO46 OFFSET(14) NUMBITS(1),
        GPIO47 OFFSET(15) NUMBITS(1),
    ],
    SWCLK [
    iso OFFSET(8) NUMBITS(1) [],
    od OFFSET(7) NUMBITS(1) [],
    ie OFFSET(8) NUMBITS(1) [],
    drive OFFSET(4) NUMBITS(2) [
        S2MA = 0,
        S4MA = 1,
        S8MA = 2,
        S12MA = 3,
    ],
    pue OFFSET(3) NUMBITS(1) [],
    pde OFFSET(2) NUMBITS(1) [],
    schmitt OFFSET(1) NUMBITS(1) [],
    slewfast OFFSET(0) NUMBITS(1) [],
],
SWD [
    iso OFFSET(8) NUMBITS(1) [],
    od OFFSET(7) NUMBITS(1) [],
    ie OFFSET(8) NUMBITS(1) [],
    drive OFFSET(4) NUMBITS(2) [
        S2MA = 0,
        S4MA = 1,
        S8MA = 2,
        S12MA = 3,
    ],
    pue OFFSET(3) NUMBITS(1) [],
    pde OFFSET(2) NUMBITS(1) [],
    schmitt OFFSET(1) NUMBITS(1) [],
    slewfast OFFSET(0) NUMBITS(1) [],
],



];

const GPIO_BASE_ADDRESS: usize = 0x40028000;
const GPIO_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(GPIO_BASE_ADDRESS as *const GpioRegisters) };

const GPIO_PAD_BASE_ADDRESS: usize = 0x40038000;
const GPIO_PAD_BASE: StaticRef<GpioPadRegisters> =
    unsafe { StaticRef::new(GPIO_PAD_BASE_ADDRESS as *const GpioPadRegisters) };

const SIO_BASE_ADDRESS: usize = 0xd0000000;
const SIO_BASE: StaticRef<SIORegisters> =
    unsafe { StaticRef::new(SIO_BASE_ADDRESS as *const SIORegisters) };

pub struct RPPins<'a> {
    pub pins: [RPGpioPin<'a>; 48],
    gpio_registers: StaticRef<GpioRegisters>,
}

impl<'a> RPPins<'a> {
    #[inline(never)]
    pub const fn new() -> Self {
        Self {
            pins: [
                RPGpioPin::new(RPGpio::GPIO0),
                RPGpioPin::new(RPGpio::GPIO1),
                RPGpioPin::new(RPGpio::GPIO2),
                RPGpioPin::new(RPGpio::GPIO3),
                RPGpioPin::new(RPGpio::GPIO4),
                RPGpioPin::new(RPGpio::GPIO5),
                RPGpioPin::new(RPGpio::GPIO6),
                RPGpioPin::new(RPGpio::GPIO7),
                RPGpioPin::new(RPGpio::GPIO8),
                RPGpioPin::new(RPGpio::GPIO9),
                RPGpioPin::new(RPGpio::GPIO10),
                RPGpioPin::new(RPGpio::GPIO11),
                RPGpioPin::new(RPGpio::GPIO12),
                RPGpioPin::new(RPGpio::GPIO13),
                RPGpioPin::new(RPGpio::GPIO14),
                RPGpioPin::new(RPGpio::GPIO15),
                RPGpioPin::new(RPGpio::GPIO16),
                RPGpioPin::new(RPGpio::GPIO17),
                RPGpioPin::new(RPGpio::GPIO18),
                RPGpioPin::new(RPGpio::GPIO19),
                RPGpioPin::new(RPGpio::GPIO20),
                RPGpioPin::new(RPGpio::GPIO21),
                RPGpioPin::new(RPGpio::GPIO22),
                RPGpioPin::new(RPGpio::GPIO23),
                RPGpioPin::new(RPGpio::GPIO24),
                RPGpioPin::new(RPGpio::GPIO25),
                RPGpioPin::new(RPGpio::GPIO26),
                RPGpioPin::new(RPGpio::GPIO27),
                RPGpioPin::new(RPGpio::GPIO28),
                RPGpioPin::new(RPGpio::GPIO29),
                RPGpioPin::new(RPGpio::GPIO30),
                RPGpioPin::new(RPGpio::GPIO31),
                RPGpioPin::new(RPGpio::GPIO32),
                RPGpioPin::new(RPGpio::GPIO33),
                RPGpioPin::new(RPGpio::GPIO34),
                RPGpioPin::new(RPGpio::GPIO35),
                RPGpioPin::new(RPGpio::GPIO36),
                RPGpioPin::new(RPGpio::GPIO37),
                RPGpioPin::new(RPGpio::GPIO38),
                RPGpioPin::new(RPGpio::GPIO39),
                RPGpioPin::new(RPGpio::GPIO40),
                RPGpioPin::new(RPGpio::GPIO41),
                RPGpioPin::new(RPGpio::GPIO42),
                RPGpioPin::new(RPGpio::GPIO43),
                RPGpioPin::new(RPGpio::GPIO44),
                RPGpioPin::new(RPGpio::GPIO45),
                RPGpioPin::new(RPGpio::GPIO46),
                RPGpioPin::new(RPGpio::GPIO47),
            ],
            gpio_registers: GPIO_BASE,
        }
    }

    pub fn get_pin(&self, pin: RPGpio) -> &'a RPGpioPin {
        &self.pins[pin as usize]
    }

    pub fn handle_interrupt(&self) {
        for bank_no in 0..4 {
            let current_val = self.gpio_registers.intr[bank_no].get();
            let enabled_val = self.gpio_registers.interrupt_proc[0].enable[bank_no].get();
            for pin in 0..8 {
                let l_low_reg_no = pin * 4;
                if (current_val & enabled_val & (1 << l_low_reg_no)) != 0 {
                    self.pins[pin + bank_no * 8].handle_interrupt();
                } else if (current_val & enabled_val & (1 << (l_low_reg_no + 1))) != 0 {
                    self.pins[pin + bank_no * 8].handle_interrupt();
                } else if (current_val & enabled_val & (1 << (l_low_reg_no + 2))) != 0 {
                    self.gpio_registers.intr[bank_no].set(current_val & (1 << (l_low_reg_no + 2)));
                    self.pins[pin + bank_no * 8].handle_interrupt();
                } else if (current_val & enabled_val & (1 << (l_low_reg_no + 3))) != 0 {
                    self.gpio_registers.intr[bank_no].set(current_val & (1 << (l_low_reg_no + 3)));
                    self.pins[pin + bank_no * 8].handle_interrupt();
                }
            }
        }
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(usize)]
    #[rustfmt::skip]
    pub enum RPGpio {
        GPIO0=0, GPIO1=1, GPIO2=2, GPIO3=3, GPIO4=4, GPIO5=5, GPIO6=6, GPIO7=7,
        GPIO8=8, GPIO9=9, GPIO10=10, GPIO11=11, GPIO12=12, GPIO13=13, GPIO14=14, GPIO15=15,
        GPIO16=16, GPIO17=17, GPIO18=18, GPIO19=19, GPIO20=20, GPIO21=21, GPIO22=22, GPIO23=23,
        GPIO24=24, GPIO25=25, GPIO26=26, GPIO27=27, GPIO28=28, GPIO29=29,
        GPIO30=30,GPIO31=31,GPIO32=32,GPIO33=33,GPIO34=34,GPIO35=35,GPIO36=36,GPIO37=37,GPIO38=38,GPIO39=39,
        GPIO40=40,GPIO41=41,GPIO42=42,GPIO43=43,GPIO44=44,GPIO45=45,GPIO46=46,GPIO47=47
    }
}
enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u32)]
    #[rustfmt::skip]

    pub enum GpioFunction {
       SPI = 1,
       UART = 2,
       I2C = 3,
       PWM = 4,
       SIO = 5,
       PIO0 = 6,
       PIO1 = 7,
       GPCK = 8,
       USB = 9,
       NULL = 0x1f
    }
}

pub struct RPGpioPin<'a> {
    pin: usize,
    client: OptionalCell<&'a dyn hil::gpio::Client>,
    gpio_registers: StaticRef<GpioRegisters>,
    gpio_pad_registers: StaticRef<GpioPadRegisters>,
    sio_registers: StaticRef<SIORegisters>,
}

impl<'a> RPGpioPin<'a> {
    pub const fn new(pin: RPGpio) -> RPGpioPin<'a> {
        RPGpioPin {
            pin: pin as usize,
            client: OptionalCell::empty(),
            gpio_registers: GPIO_BASE,
            gpio_pad_registers: GPIO_PAD_BASE,
            sio_registers: SIO_BASE,
        }
    }

    pub(crate) fn pin(&self) -> usize {
        self.pin
    }

    fn get_mode(&self) -> hil::gpio::Configuration {
        //TODO - read alternate function
        let pad_output_disable = !self.gpio_pad_registers.gpio_pad[self.pin].is_set(GPIO_PAD::OD);
        let pin_mask = 1 << self.pin;
        let sio_output_enable = (self.sio_registers.gpio_oe.read(GPIO_OE::OE) & pin_mask) != 0;

        match (pad_output_disable, sio_output_enable) {
            (true, true) => hil::gpio::Configuration::Output,
            (true, false) => hil::gpio::Configuration::Input,
            (false, _) => hil::gpio::Configuration::LowPower,
        }
    }

    fn read_pin(&self) -> bool {
        //TODO - read alternate function
        let value = self.sio_registers.gpio_out.read(GPIO_OUT::OUT) & (1 << self.pin);
        value != 0
    }

    pub fn set_function(&self, f: GpioFunction) {
        self.activate_pads();
        self.gpio_registers.pin[self.pin]
            .ctrl
            .write(GPIOx_CTRL::FUNCSEL.val(f as u32));
    }

    fn get_pullup_pulldown(&self) -> hil::gpio::FloatingState {
        //TODO - read alternate function
        let pullup = self.gpio_pad_registers.gpio_pad[self.pin].read(GPIO_PAD::PUE);
        let pulldown = self.gpio_pad_registers.gpio_pad[self.pin].read(GPIO_PAD::PDE);

        match (pullup, pulldown) {
            (0, 0) => hil::gpio::FloatingState::PullNone,
            (0, 1) => hil::gpio::FloatingState::PullDown,
            (1, 0) => hil::gpio::FloatingState::PullUp,
            _ => panic!("Invalid GPIO floating state."),
        }
    }

    pub fn activate_pads(&self) {
        self.gpio_pad_registers.gpio_pad[self.pin]
            .modify(GPIO_PAD::OD::CLEAR + GPIO_PAD::ISO::CLEAR + GPIO_PAD::IE::SET);
    }

    pub fn deactivate_pads(&self) {
        self.gpio_pad_registers.gpio_pad[self.pin]
            .modify(GPIO_PAD::OD::SET + GPIO_PAD::ISO::SET + GPIO_PAD::IE::CLEAR);
    }

    pub fn handle_interrupt(&self) {
        self.client.map(|client| client.fired());
    }

    // needed for usb errata https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf#RP2040-E5

    pub fn start_usb_errata(&self) -> (u32, u32) {
        let prev_ctrl = self.gpio_registers.pin[self.pin].ctrl.get();
        let prev_pad = self.gpio_pad_registers.gpio_pad[self.pin].get();

        self.gpio_pad_registers.gpio_pad[self.pin].modify(GPIO_PAD::PUE::SET + GPIO_PAD::PDE::SET);
        self.gpio_registers.pin[self.pin]
            .ctrl
            .modify(GPIOx_CTRL::OEOVER::Disable);

        self.set_function(GpioFunction::GPCK);

        self.gpio_registers.pin[self.pin]
            .ctrl
            .modify(GPIOx_CTRL::INOVER::DriveHigh);

        (prev_ctrl, prev_pad)
    }

    pub fn finish_usb_errata(&self, prev_ctrl: u32, prev_pad: u32) {
        self.gpio_registers.pin[self.pin].ctrl.set(prev_ctrl);
        self.gpio_pad_registers.gpio_pad[self.pin].set(prev_pad);
    }
}

impl<'a> hil::gpio::Interrupt<'a> for RPGpioPin<'a> {
    fn set_client(&self, client: &'a dyn hil::gpio::Client) {
        self.client.set(client);
    }

    fn is_pending(&self) -> bool {
        let interrupt_bank_no = self.pin / 8;
        let l_low_reg_no = (self.pin * 4) % 32;
        let current_val = self.gpio_registers.interrupt_proc[0].status[interrupt_bank_no].get();
        (current_val
            & (1 << l_low_reg_no)
            & (1 << (l_low_reg_no + 1))
            & (1 << (l_low_reg_no + 2))
            & (1 << (l_low_reg_no + 3)))
            != 0
    }

    fn enable_interrupts(&self, mode: hil::gpio::InterruptEdge) {
        let interrupt_bank_no = self.pin / 8;
        match mode {
            hil::gpio::InterruptEdge::RisingEdge => {
                let high_reg_no = (self.pin * 4 + 3) % 32;
                let current_val =
                    self.gpio_registers.interrupt_proc[0].enable[interrupt_bank_no].get();
                self.gpio_registers.interrupt_proc[0].enable[interrupt_bank_no]
                    .set((1 << high_reg_no) | current_val);
            }
            hil::gpio::InterruptEdge::FallingEdge => {
                let low_reg_no = (self.pin * 4 + 2) % 32;
                let current_val =
                    self.gpio_registers.interrupt_proc[0].enable[interrupt_bank_no].get();
                self.gpio_registers.interrupt_proc[0].enable[interrupt_bank_no]
                    .set((1 << low_reg_no) | current_val);
            }
            hil::gpio::InterruptEdge::EitherEdge => {
                let low_reg_no = (self.pin * 4 + 2) % 32;
                let high_reg_no = low_reg_no + 1;
                let current_val =
                    self.gpio_registers.interrupt_proc[0].enable[interrupt_bank_no].get();
                self.gpio_registers.interrupt_proc[0].enable[interrupt_bank_no]
                    .set((1 << high_reg_no) | (1 << low_reg_no) | current_val);
            }
        }
    }

    fn disable_interrupts(&self) {
        let interrupt_bank_no = self.pin / 8;
        let low_reg_no = (self.pin * 4 + 2) % 32;
        let high_reg_no = low_reg_no + 1;
        let current_val = self.gpio_registers.interrupt_proc[0].enable[interrupt_bank_no].get();
        self.gpio_registers.interrupt_proc[0].enable[interrupt_bank_no]
            .set(current_val & !(1 << high_reg_no) & !(1 << low_reg_no));
    }
}

impl hil::gpio::Configure for RPGpioPin<'_> {
    fn configuration(&self) -> hil::gpio::Configuration {
        self.get_mode()
    }
    /// Set output mode
    fn make_output(&self) -> hil::gpio::Configuration {
        self.set_function(GpioFunction::SIO);
        self.activate_pads();
        self.sio_registers.gpio_oe_set.set(1 << self.pin);
        self.get_mode()
    }
    /// Disable pad output
    fn disable_output(&self) -> hil::gpio::Configuration {
        self.set_function(GpioFunction::SIO);
        self.gpio_pad_registers.gpio_pad[self.pin].modify(GPIO_PAD::OD::SET);
        self.get_mode()
    }
    /// Set input mode
    fn make_input(&self) -> hil::gpio::Configuration {
        self.set_function(GpioFunction::SIO);
        self.activate_pads();
        self.sio_registers.gpio_oe_clr.set(1 << self.pin);
        self.get_mode()
    }
    /// Disable input mode, will set pin to output mode
    fn disable_input(&self) -> hil::gpio::Configuration {
        self.make_output()
    }
    fn deactivate_to_low_power(&self) {
        self.set_function(GpioFunction::SIO);
        self.gpio_pad_registers.gpio_pad[self.pin].modify(GPIO_PAD::OD::SET);
    }

    fn set_floating_state(&self, mode: hil::gpio::FloatingState) {
        match mode {
            hil::gpio::FloatingState::PullUp => self.gpio_pad_registers.gpio_pad[self.pin]
                .modify(GPIO_PAD::PUE::SET + GPIO_PAD::PDE::CLEAR),
            hil::gpio::FloatingState::PullDown => self.gpio_pad_registers.gpio_pad[self.pin]
                .modify(GPIO_PAD::PUE::CLEAR + GPIO_PAD::PDE::SET),
            hil::gpio::FloatingState::PullNone => self.gpio_pad_registers.gpio_pad[self.pin]
                .modify(GPIO_PAD::PUE::CLEAR + GPIO_PAD::PDE::CLEAR),
        }
    }

    fn floating_state(&self) -> hil::gpio::FloatingState {
        self.get_pullup_pulldown()
    }

    fn is_input(&self) -> bool {
        let mode = self.get_mode();
        match mode {
            hil::gpio::Configuration::Input => true,
            hil::gpio::Configuration::InputOutput => true,
            _ => false,
        }
    }

    fn is_output(&self) -> bool {
        let mode = self.get_mode();
        match mode {
            hil::gpio::Configuration::Output => true,
            hil::gpio::Configuration::InputOutput => true,
            _ => false,
        }
    }
}

impl hil::gpio::Output for RPGpioPin<'_> {
    fn set(&self) {
        // For performance this match might be skipped
        match self.get_mode() {
            hil::gpio::Configuration::Output | hil::gpio::Configuration::InputOutput => {
                self.sio_registers.gpio_out_set.set(1 << self.pin);
            }
            _ => {}
        }
    }

    fn clear(&self) {
        // For performance this match might be skipped
        match self.get_mode() {
            hil::gpio::Configuration::Output | hil::gpio::Configuration::InputOutput => {
                self.sio_registers.gpio_out_clr.set(1 << self.pin);
            }
            _ => {}
        }
    }

    fn toggle(&self) -> bool {
        match self.get_mode() {
            hil::gpio::Configuration::Output | hil::gpio::Configuration::InputOutput => {
                self.sio_registers.gpio_out_xor.set(1 << self.pin);
            }
            _ => {}
        }
        self.read_pin()
    }
}

impl hil::gpio::Input for RPGpioPin<'_> {
    fn read(&self) -> bool {
        let value = self.sio_registers.gpio_in.read(GPIO_IN::IN) & (1 << self.pin);
        value != 0
    }
}



pub struct SIO {
    registers: StaticRef<SIORegisters>,
}

impl SIO {
    #[inline(never)]
    pub const fn new() -> Self {
        Self {
            registers: SIO_BASE,
        }
    }

    pub fn handle_proc_interrupt(&self, for_processor: Processor) {
        match for_processor {
            Processor::Processor0 => {
                // read data from the fifo
                self.registers.fifo_rd.get();
                self.registers.fifo_st.set(0xff);
            }
            Processor::Processor1 => {
                if self.registers.cpuid.get() == 1 {
                    panic!("Kernel should not run on processor 1");
                } else {
                    panic!("SIO_PROC1_IRQ should be ignored for processor 1");
                }
            }
        }
    }

    pub fn handle_fifo_interrupt(&self) {
        let _st = self.registers.fifo_st.get();
    }

    pub fn get_processor(&self) -> Processor {
        let proc_id = self.registers.cpuid.get();
        match proc_id {
            0 => Processor::Processor0,
            1 => Processor::Processor1,
            _ => panic!("SIO CPUID cannot be {}", proc_id),
        }
    }
}
