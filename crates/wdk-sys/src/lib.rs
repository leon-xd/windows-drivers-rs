// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

//! Direct bindings to APIs available in the Windows Development Kit (WDK)

#![no_std]

#[cfg(any(driver_type = "wdm", driver_type = "kmdf", driver_type = "umdf"))]
mod constants;
#[cfg(any(driver_type = "wdm", driver_type = "kmdf", driver_type = "umdf"))]
mod types;
#[cfg(any(driver_type = "wdm", driver_type = "kmdf", driver_type = "umdf"))]
pub use crate::{constants::*, types::*};

#[cfg(any(driver_type = "wdm", driver_type = "kmdf", driver_type = "umdf"))]
mod macros;
#[cfg(any(driver_type = "wdm", driver_type = "kmdf", driver_type = "umdf"))]
pub use wdk_macros as __proc_macros;

#[cfg(any(driver_type = "wdm", driver_type = "kmdf"))]
pub mod ntddk;
#[cfg(driver_type = "umdf")]
pub mod windows;
#[cfg(any(driver_type = "kmdf", driver_type = "umdf"))]
pub mod wdf;

#[cfg(feature = "test-stubs")]
pub mod test_stubs;

#[cfg(any(driver_type = "kmdf", driver_type = "umdf"))]
use lazy_static::lazy_static;

// This is fine because we don't actually have any floating point instruction in
// our binary, thanks to our target defining soft-floats. fltused symbol is
// necessary due to LLVM being too eager to set it: it checks the LLVM IR for
// floating point instructions - even if soft-float is enabled!
#[allow(missing_docs)]
#[no_mangle]
pub static _fltused: () = ();

// FIXME: Is there any way to avoid this stub? See https://github.com/rust-lang/rust/issues/101134
#[allow(missing_docs)]
#[allow(clippy::missing_const_for_fn)] // const extern is not yet supported: https://github.com/rust-lang/rust/issues/64926
#[no_mangle]
pub extern "system" fn __CxxFrameHandler3() -> i32 {
    0
}

// FIXME: replace lazy_static with std::Lazy once available: https://github.com/rust-lang/rust/issues/109736
#[cfg(any(driver_type = "kmdf", driver_type = "umdf"))]
lazy_static! {
    #[allow(missing_docs)]
    pub static ref WDF_FUNCTION_TABLE: &'static [WDFFUNC] = {
        // SAFETY: `WdfFunctions` is generated as a mutable static, but is not supposed to be ever mutated by WDF.
        let wdf_function_table = unsafe { WdfFunctions };

        // SAFETY: `WdfFunctionCount` is generated as a mutable static, but is not supposed to be ever mutated by WDF.
        let wdf_function_count = unsafe { WdfFunctionCount } as usize;

        // SAFETY: This is safe because:
        //         1. `WdfFunctions` is valid for reads for `WdfFunctionCount` * `core::mem::size_of::<WDFFUNC>()`
        //            bytes, and is guaranteed to be aligned and it must be properly aligned.
        //         2. `WdfFunctions` points to `WdfFunctionCount` consecutive properly initialized values of
        //            type `WDFFUNC`.
        //         3. WDF does not mutate the memory referenced by the returned slice for for its entire `'static' lifetime.
        //         4. The total size, `WdfFunctionCount` * `core::mem::size_of::<WDFFUNC>()`, of the slice must be no
        //            larger than `isize::MAX`. This is proven by the below `debug_assert!`.
        unsafe {
            debug_assert!(isize::try_from(wdf_function_count * core::mem::size_of::<WDFFUNC>()).is_ok());
            core::slice::from_raw_parts(wdf_function_table, wdf_function_count)
        }
    };
}

#[cfg(any(driver_type = "wdm", driver_type = "kmdf", driver_type = "umdf"))]
#[allow(missing_docs)]
#[must_use]
#[allow(non_snake_case)]
pub const fn NT_SUCCESS(nt_status: NTSTATUS) -> bool {
    nt_status >= 0
}

#[cfg(any(driver_type = "wdm", driver_type = "kmdf"))]
#[allow(missing_docs)]
#[macro_export]
#[allow(non_snake_case)]
macro_rules! PAGED_CODE {
    () => {
        debug_assert!(unsafe { KeGetCurrentIrql() <= APC_LEVEL as u8 });
    };
}
