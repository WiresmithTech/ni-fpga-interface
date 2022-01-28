//! Holds session management functions for the FPGA.
//! 
use nifpga_sys::*;

//todo: need to do one time intialisation of FPGA library
// and cleanup on drop.


pub type SessionHandle = u32;

pub struct Session {
    pub handle: SessionHandle
}
