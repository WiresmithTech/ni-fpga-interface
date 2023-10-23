//! Provides the low level interface to the FPGA FIFOs.
//!
//! In general we recommend using the [`crate::fifos`] module for a higher level interface.

use crate::error::{to_fpga_result, NiFpgaStatus, Result};
use libc::size_t;

use super::{Session, SessionHandle};

pub type FifoAddress = u32;
pub type PeerToPeerEndpoint = u32;

#[link(name = "ni_fpga")]
extern "C" {
    fn NiFpga_ConfigureFifo2(
        session: SessionHandle,
        fifo: FifoAddress,
        requested_depth: size_t,
        actual_depth: *mut size_t,
    ) -> NiFpgaStatus;

    fn NiFpga_StartFifo(session: SessionHandle, fifo: FifoAddress) -> NiFpgaStatus;

    fn NiFpga_StopFifo(session: SessionHandle, fifo: FifoAddress) -> NiFpgaStatus;

    fn NiFpga_ReleaseFifoElements(
        session: SessionHandle,
        fifo: FifoAddress,
        number_of_elements: size_t,
    ) -> NiFpgaStatus;

    fn NiFpga_GetPeerToPeerFifoEndpoint(
        session: SessionHandle,
        fifo: FifoAddress,
        endpoint: *mut PeerToPeerEndpoint,
    ) -> NiFpgaStatus;
}

impl Session {
    /// Specify the depth of the host memory part of the FIFO.
    ///
    /// Returns the actual size configured which may be larger than the request.
    pub fn configure_fifo(&self, fifo: FifoAddress, requested_depth: usize) -> Result<usize> {
        let mut actual_depth: size_t = 0;
        let result = unsafe {
            NiFpga_ConfigureFifo2(
                self.handle,
                fifo,
                requested_depth,
                &mut actual_depth as *mut size_t,
            )
        };
        to_fpga_result(actual_depth, result)
    }

    /// Start the FIFO.
    pub fn start_fifo(&self, fifo: FifoAddress) -> Result<()> {
        let result = unsafe { NiFpga_StartFifo(self.handle, fifo) };
        to_fpga_result((), result)
    }

    /// Stop the FIFO.
    pub fn stop_fifo(&self, fifo: FifoAddress) -> Result<()> {
        let result = unsafe { NiFpga_StopFifo(self.handle, fifo) };
        to_fpga_result((), result)
    }

    /// Releases previously acquired FIFO elements.
    /// The FPGA target cannot read elements acquired by the host.
    /// Therefore, the host must release elements after acquiring them.
    ///
    /// Always release all acquired elements before closing the session.
    /// Do not attempt to access FIFO elements after the elements are released or the session is closed.
    pub fn release_fifo_elements(
        &self,
        fifo: FifoAddress,
        number_of_elements: usize,
    ) -> Result<()> {
        let result = unsafe { NiFpga_ReleaseFifoElements(self.handle, fifo, number_of_elements) };
        to_fpga_result((), result)
    }

    /// Gets the endpoint number of a peer-to-peer FIFO.
    pub fn get_peer_to_peer_fifo_endpoint(&self, fifo: FifoAddress) -> Result<PeerToPeerEndpoint> {
        let mut endpoint: PeerToPeerEndpoint = 0;
        let result = unsafe { NiFpga_GetPeerToPeerFifoEndpoint(self.handle, fifo, &mut endpoint) };
        to_fpga_result(endpoint, result)
    }
}
