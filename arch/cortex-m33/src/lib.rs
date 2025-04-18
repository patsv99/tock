// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

//! Shared implementations for ARM Cortex-M33 MCUs.

#![crate_name = "cortexm33"]
#![crate_type = "rlib"]
#![no_std]

use core::fmt::Write;

pub mod mpu {
    pub type MPU = cortexm::mpu::MPU<8, 32>;
}

pub use cortexm::dwt;
pub use cortexm::initialize_ram_jump_to_main;
pub use cortexm::interrupt_mask;
pub use cortexm::nvic;
pub use cortexm::scb;
pub use cortexm::support;
pub use cortexm::systick;
pub use cortexm::unhandled_interrupt;
pub use cortexm::CortexMVariant;

// Enum with no variants to ensure that this type is not instantiable. It is
// only used to pass architecture-specific constants and functions via the
// `CortexMVariant` trait.
pub enum CortexM33 {}

impl cortexm::CortexMVariant for CortexM33 {
    const GENERIC_ISR: unsafe extern "C" fn() = cortexv8m::generic_isr_arm_v8m;
    const SYSTICK_HANDLER: unsafe extern "C" fn() = cortexv8m::systick_handler_arm_v8m;
    const SVC_HANDLER: unsafe extern "C" fn() = cortexv8m::svc_handler_arm_v8m;
    const HARD_FAULT_HANDLER: unsafe extern "C" fn() = cortexv8m::hard_fault_handler_arm_v8m;

    #[cfg(all(target_arch = "arm", target_os = "none"))]
    unsafe fn switch_to_user(
        user_stack: *const usize,
        process_regs: &mut [usize; 8],
    ) -> *const usize {
        cortexv8m::switch_to_user_arm_v8m(user_stack, process_regs)
    }

    #[cfg(not(all(target_arch = "arm", target_os = "none")))]
    unsafe fn switch_to_user(
        _user_stack: *const usize,
        _process_regs: &mut [usize; 8],
    ) -> *const usize {
        unimplemented!()
    }

    #[inline]
    unsafe fn print_cortexm_state(writer: &mut dyn Write) {
        cortexm::print_cortexm_state(writer)
    }
}

pub mod syscall {
    pub type SysCall = cortexm::syscall::SysCall<crate::CortexM33>;
}
