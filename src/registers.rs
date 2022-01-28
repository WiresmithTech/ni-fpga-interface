//! Implements the register interfaces to the FPGA.
//!
use crate::error::Result;

type RegisterAddress = usize;

/// Provides a binding to a register address including a type.
///
/// By generating these as part of an initialisation step - the registers can then be accessed safely at later steps knowing the address and types are matched.
struct Register<T> {
    address: RegisterAddress,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Register<T> {
    fn new(address: RegisterAddress) -> Self {
        Self {
            address,
            phantom: std::marker::PhantomData,
        }
    }

    fn read(&self, session: &impl RegisterInterface<T>) -> Result<T> {
        session.read(self.address)
    }

    fn write(&self, session: &impl RegisterInterface<T>, value: T) -> Result<()> {
        session.write(self.address, value)
    }
}

trait RegisterInterface<T> {
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


//todo: Implement RegisterInterface for supported types on session.