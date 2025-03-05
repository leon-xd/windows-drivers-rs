// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

//! Direct bindings to APIs available in the Windows Development Kit (WDK)

#![no_std]

#[cfg(any(driver_type = "WDM", driver_type = "KMDF", driver_type = "UMDF"))]
mod constants;
#[cfg(any(driver_type = "WDM", driver_type = "KMDF", driver_type = "UMDF"))]
mod types;
#[cfg(any(driver_type = "WDM", driver_type = "KMDF", driver_type = "UMDF"))]
pub use crate::{constants::*, types::*};

#[cfg(any(driver_type = "WDM", driver_type = "KMDF", driver_type = "UMDF"))]
mod macros;
#[cfg(any(driver_type = "WDM", driver_type = "KMDF", driver_type = "UMDF"))]
pub use wdk_macros as __proc_macros;

#[cfg(any(driver_type = "WDM", driver_type = "KMDF"))]
pub mod ntddk;
#[cfg(any(driver_type = "KMDF", driver_type = "UMDF"))]
pub mod wdf;
#[cfg(any(driver_type = "KMDF", driver_type = "UMDF"))]
pub use wdf::WDF_FUNCTION_TABLE;

// FIXME: Gate this properly
pub mod hid;
pub mod parallel_ports;
pub mod spb;
pub mod usb;

#[cfg(driver_type = "UMDF")]
pub mod windows;

#[cfg(feature = "test-stubs")]
pub mod test_stubs;

// This is fine because we don't actually have any floating point instruction in
// our binary, thanks to our target defining soft-floats. fltused symbol is
// necessary due to LLVM being too eager to set it: it checks the LLVM IR for
// floating point instructions - even if soft-float is enabled!
#[allow(missing_docs)]
#[no_mangle]
pub static _fltused: () = ();

// FIXME: Is there any way to avoid this stub? See https://github.com/rust-lang/rust/issues/101134
#[cfg(panic = "abort")]
#[allow(missing_docs)]
#[allow(clippy::missing_const_for_fn)] // const extern is not yet supported: https://github.com/rust-lang/rust/issues/64926
#[no_mangle]
pub extern "system" fn __CxxFrameHandler3() -> i32 {
    0
}

#[cfg(any(driver_type = "WDM", driver_type = "KMDF", driver_type = "UMDF"))]
#[allow(missing_docs)]
#[must_use]
#[allow(non_snake_case)]
pub const fn NT_SUCCESS(nt_status: NTSTATUS) -> bool {
    nt_status >= 0
}

#[cfg(any(driver_type = "WDM", driver_type = "KMDF"))]
#[allow(missing_docs)]
#[macro_export]
#[allow(non_snake_case)]
macro_rules! PAGED_CODE {
    () => {
        debug_assert!(unsafe { $crate::ntddk::KeGetCurrentIrql() <= $crate::APC_LEVEL as u8 });
    };
}
