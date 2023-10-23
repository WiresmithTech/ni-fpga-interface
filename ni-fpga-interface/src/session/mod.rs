//! Holds session management functions for the FPGA.
//!
mod data_interfaces;
pub mod fifo_control;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::error::{to_fpga_result, FPGAError};
use crate::nifpga_sys::*;
pub use data_interfaces::*;

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

/// Options for the session.
pub struct SessionOptions {
    /// Reset the FPGA on close (default: True)
    pub reset_on_close: bool,
    /// Run the FPGA when you open the session (default: True)
    pub run_on_open: bool,
}

impl SessionOptions {
    fn open_attribute(&self) -> u32 {
        let mut attribute = 0;
        if !self.run_on_open {
            attribute |= 1;
        }
        attribute
    }

    fn close_attribute(&self) -> u32 {
        let mut attribute = 0;
        if !self.reset_on_close {
            attribute |= 1;
        }
        attribute
    }
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
    close_attribute: u32,
    _context: Arc<NiFpgaContext>,
}

impl Session {
    /// Create a new session for the specified bitfile and resource.
    ///
    /// You must have an open context to construct the session.
    ///
    /// The bitfile is the compiled FPGA VI and can be provided as a relative or absolute path.
    ///
    /// The session options specify run and reset behaviour for the session.
    /// You can use default to run on start and reset on end.
    ///
    /// # Example
    /// ```no_run
    /// use ni_fpga_interface::session::{NiFpgaContext, Session};
    ///
    /// let fpga_context = NiFpgaContext::new().unwrap();
    /// let session = Session::new(
    ///    &fpga_context,
    ///   "./NiFpga_Main.lvbitx",
    ///  "signature",
    /// "RIO0",
    /// &Default::default(),
    /// ).unwrap();
    /// ```
    pub fn new(
        context: &Arc<NiFpgaContext>,
        bitfile: &str,
        signature: &str,
        resource: &str,
        options: &SessionOptions,
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
                options.open_attribute(),
                &mut handle,
            )
        };
        to_fpga_result(
            Self {
                handle,
                _context: context.clone(),
                close_attribute: options.close_attribute(),
            },
            result,
        )
    }

    /// Update the session options if you want to change the close behaviour.
    ///
    /// # Example
    /// ```no_run
    /// use ni_fpga_interface::session::{NiFpgaContext, Session, SessionOptions};
    ///
    ///#  let fpga_context = NiFpgaContext::new().unwrap();
    ///#  let mut session = Session::new(
    ///#   &fpga_context,
    ///#  "./NiFpga_Main.lvbitx",
    ///# "signature",
    ///# "RIO0",
    ///# &Default::default(),
    ///# ).unwrap();
    ///
    /// session.set_options(&SessionOptions { reset_on_close: false, ..Default::default() });
    /// ```
    pub fn set_options(&mut self, options: &SessionOptions) {
        self.close_attribute = options.close_attribute();
    }

    /// Reset the FPGA back to it's initial state.
    pub fn reset(&mut self) -> Result<(), crate::error::FPGAError> {
        let result = unsafe { NiFpga_Reset(self.handle) };

        to_fpga_result((), result)
    }

    /// Runs the FPGA on the target.
    /// If `wait_until_done` is true this function will block until the FPGA is done running.
    pub fn run(&mut self, wait_until_done: bool) -> Result<(), crate::error::FPGAError> {
        let attributes = if wait_until_done { 1 } else { 0 };

        let result = unsafe { NiFpga_Run(self.handle, attributes) };

        to_fpga_result((), result)
    }

    /// Abort the FPGA VI.
    pub fn abort(&mut self) -> Result<(), crate::error::FPGAError> {
        let result = unsafe { NiFpga_Abort(self.handle) };

        to_fpga_result((), result)
    }

    /// Re-download the bitfile to the FPGA.
    pub fn download(&mut self) -> Result<(), crate::error::FPGAError> {
        let result = unsafe { NiFpga_Download(self.handle) };

        to_fpga_result((), result)
    }

    /// Close the session to the FPGA and resets it if set for the session.
    pub fn close(self) -> Result<(), crate::error::FPGAError> {
        let result = unsafe { NiFpga_Close(self.handle, self.close_attribute) };

        to_fpga_result((), result)
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            NiFpga_Close(self.handle, self.close_attribute);
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_session_options_default() {
        let options = super::SessionOptions::default();
        assert_eq!(options.run_on_open, true);
        assert_eq!(options.reset_on_close, true);
    }

    #[test]
    fn test_session_options_get_open_attribute_run() {
        let mut options = super::SessionOptions::default();
        options.run_on_open = true;
        assert_eq!(options.open_attribute(), 0);
    }

    #[test]
    fn test_session_options_get_open_attribute_no_run() {
        let mut options = super::SessionOptions::default();
        options.run_on_open = false;
        assert_eq!(options.open_attribute(), 1);
    }

    #[test]
    fn test_session_options_get_close_attribute_reset() {
        let mut options = super::SessionOptions::default();
        options.reset_on_close = true;
        assert_eq!(options.close_attribute(), 0);
    }

    #[test]
    fn test_session_options_get_close_attribute_no_reset() {
        let mut options = super::SessionOptions::default();
        options.reset_on_close = false;
        assert_eq!(options.close_attribute(), 1);
    }
}
