//! Error support for the FPGA Interface.

pub enum FPGAError {}

pub type Result<T> = core::result::Result<T, FPGAError>;
