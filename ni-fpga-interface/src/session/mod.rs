//! Holds session management functions for the FPGA.
//!
mod data_interfaces;
pub mod fifo_control;
use std::ffi::c_char;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::error::{to_fpga_result, FPGAError, NiFpgaStatus};
pub use data_interfaces::*;

#[link(name = "ni_fpga")]
extern "C" {
    fn NiFpga_Initialize() -> NiFpgaStatus;
    fn NiFpga_Finalize() -> NiFpgaStatus;
    fn NiFpga_Open(
        bitfile: *const c_char,
        signature: *const c_char,
        resource: *const c_char,
        attribute: u32,
        session: *mut SessionHandle,
    ) -> NiFpgaStatus;
    fn NiFpga_Reset(session: SessionHandle) -> NiFpgaStatus;
    fn NiFpga_Abort(session: SessionHandle) -> NiFpgaStatus;
    fn NiFpga_Run(session: SessionHandle) -> NiFpgaStatus;
    fn NiFpga_Close(session: SessionHandle, attribute: u32) -> NiFpgaStatus;

}

static CONTEXT_ACTIVE: AtomicBool = AtomicBool::new(false);

/// An NI FPGA context must be initialized and active for the FPGA
/// functions to work.
///
/// We wrap this type in an Arc so that it can be shared between sessions
/// and automatically destruct when no more sessions are active or it is out of scope.
pub struct NiFpgaContext();

impl NiFpgaContext {
    /// Create a new NI FPGA context which is required to open a session.
    ///
    /// This can only be called once per application and will return an error
    /// if you call it more than once.
    pub fn new() -> Result<Arc<Self>, FPGAError> {
        // Use an atomic to prevent multiple contexts from being active at once.
        if CONTEXT_ACTIVE
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return Err(FPGAError::ContextAlreadyActive);
        }

        let status = unsafe { NiFpga_Initialize() };
        to_fpga_result(Arc::new(Self {}), status)
        //Ok(Arc::new(Self {}))
    }
}

impl Drop for NiFpgaContext {
    fn drop(&mut self) {
        unsafe {
            NiFpga_Finalize();
        }
        CONTEXT_ACTIVE.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

pub type SessionHandle = u32;

/// Options for the session.
pub struct SessionOptions {
    /// Reset the FPGA on close (default: True)
    pub reset_on_close: bool,
    /// Run the FPGA when you open the session (default: True)
    pub run_on_open: bool,
}

impl Default for SessionOptions {
    fn default() -> Self {
        Self {
            reset_on_close: true,
            run_on_open: true,
        }
    }
}

pub struct Session {
    pub handle: SessionHandle,
    _context: Arc<NiFpgaContext>,
}

impl Session {
    pub fn new(
        context: &Arc<NiFpgaContext>,
        bitfile: &str,
        signature: &str,
        resource: &str,
    ) -> Result<Self, crate::error::FPGAError> {
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
        to_fpga_result(
            Self {
                handle,
                _context: context.clone(),
            },
            result,
        )
    }

    pub fn reset(&mut self) -> Result<(), crate::error::FPGAError> {
        let result = unsafe { NiFpga_Reset(self.handle) };

        to_fpga_result((), result)
    }

    pub fn run(&mut self) -> Result<(), crate::error::FPGAError> {
        let result = unsafe { NiFpga_Run(self.handle) };

        to_fpga_result((), result)
    }

    /// Abort the FPGA VI.
    pub fn abort(&mut self) -> Result<(), crate::error::FPGAError> {
        let result = unsafe { NiFpga_Abort(self.handle) };

        to_fpga_result((), result)
    }

    /// Close the session to the FPGA and resets it if set for the session.
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
