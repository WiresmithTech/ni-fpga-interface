//! Holds session management functions for the FPGA.
//!
mod data_interfaces;
pub mod fifo_control;
use std::sync::Once;

use crate::error::{to_fpga_result, NiFpgaStatus};
pub use data_interfaces::*;

static LIB_INIT: Once = Once::new();

extern "C" {
    fn NiFpga_Initialize() -> i32;
    //fn NiFpga_Finalize() -> NiFpga_Status;
    fn NiFpga_Open(
        bitfile: *const i8,
        signature: *const i8,
        resource: *const i8,
        attribute: u32,
        session: *mut SessionHandle,
    ) -> NiFpgaStatus;
    fn NiFpga_Reset(session: SessionHandle) -> NiFpgaStatus;
    fn NiFpga_Run(session: SessionHandle) -> NiFpgaStatus;
    fn NiFpga_Close(session: SessionHandle, attribute: u32) -> NiFpgaStatus;
}

pub type SessionHandle = u32;

pub struct Session {
    pub handle: SessionHandle,
}

impl Session {
    pub fn new(
        bitfile: &str,
        signature: &str,
        resource: &str,
    ) -> Result<Self, crate::error::FPGAError> {
        LIB_INIT.call_once(|| unsafe {
            NiFpga_Initialize();
        });

        let mut handle: SessionHandle = 0;
        let bitfile = std::ffi::CString::new(bitfile).unwrap();
        let signature = std::ffi::CString::new(signature).unwrap();
        let resource = std::ffi::CString::new(resource).unwrap();
        let result = unsafe {
            NiFpga_Open(
                bitfile.as_ptr(),
                signature.as_ptr(),
                resource.as_ptr(),
                0,
                &mut handle,
            )
        };
        to_fpga_result(Self { handle }, result)
    }

    pub fn reset(&mut self) -> Result<(), crate::error::FPGAError> {
        let result = unsafe { NiFpga_Reset(self.handle) };

        to_fpga_result((), result)
    }

    pub fn run(&mut self) -> Result<(), crate::error::FPGAError> {
        let result = unsafe { NiFpga_Run(self.handle) };

        to_fpga_result((), result)
    }

    pub fn close(self) -> Result<(), crate::error::FPGAError> {
        let result = unsafe { NiFpga_Close(self.handle, 0) };

        to_fpga_result((), result)
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            NiFpga_Close(self.handle, 0);
        }
    }
}
