//! This crate provides an interface to NI FPGA devices.
//!
//! It requires the use of a build script from the C interface in the
//! linked ni-fpga-interface-build crate.
//!
//! This module has two conceptual levels:
//!
//! * [`session::Session`] - This is the main wrapper for the NI FPGA C interface.
//!   Some elements will be used directly but some will be easier to use in the higher level.
//! * The other modules define FPGA resources and can be used with session as a higher level interface. These include:
//!   * [`registers`] - For reading and writing registers i.e. front panel controls and indicators.
//!   * [`fifos`] - For reading and writing DMA FIFOs.
//!   * [`irq`] - For waiting on and acknowledging IRQs.
//!
//! Registers and FIFOs are dynamic according to the particular bitfile you load.
//! For this reason, the build module generates a module with the definitions of the registers and FIFOs for you.

mod error;
pub mod fifos;
pub mod irq;
mod nifpga_sys;
pub mod registers;
pub mod session;
mod types;

pub use error::*;
