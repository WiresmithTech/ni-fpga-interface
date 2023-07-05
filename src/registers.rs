//! Implements the register interfaces to the FPGA.
//!
use crate::error::{to_fpga_result, NiFpga_Status, Result};
use crate::session::{Session, SessionHandle};

extern "C" {
    fn NiFpga_ReadU8(session: SessionHandle, offset: u32, value: *mut u8) -> NiFpga_Status;
    fn NiFpga_WriteU8(session: SessionHandle, offset: u32, value: u8) -> NiFpga_Status;
}

type RegisterAddress = u32;

/// Provides a binding to a register address including a type.
///
/// By generating these as part of an initialisation step - the registers can then be accessed safely at later steps knowing the address and types are matched.
pub struct Register<T> {
    address: RegisterAddress,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Register<T> {
    pub fn new(address: RegisterAddress) -> Self {
        Self {
            address,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn read(&self, session: &impl RegisterInterface<T>) -> Result<T> {
        session.read(self.address)
    }

    pub fn write(&self, session: &impl RegisterInterface<T>, value: T) -> Result<()> {
        session.write(self.address, value)
    }
}

pub trait RegisterInterface<T> {
    fn read(&self, address: RegisterAddress) -> Result<T>;
    fn write(&self, address: RegisterAddress, data: T) -> Result<()>;
}

/// Used to allow the implementation of clusters.
///
/// S is size in bytes of the type.
trait CustomRegisterType<const S: usize> {
    fn from_buffer(buffer: &[u8; S]) -> Self;
    fn to_buffer(&self, buffer: &mut [u8; S]);
}

impl RegisterInterface<u8> for Session {
    fn read(&self, address: RegisterAddress) -> Result<u8> {
        let mut value: u8 = 0;
        let return_code = unsafe { NiFpga_ReadU8(self.handle, address, &mut value) };
        to_fpga_result(value, return_code)
    }

    fn write(&self, address: RegisterAddress, value: u8) -> Result<()> {
        let return_code = unsafe { NiFpga_WriteU8(self.handle, address, value) };
        to_fpga_result((), return_code)
    }
}

//todo: Implement RegisterInterface for supported types on session.
