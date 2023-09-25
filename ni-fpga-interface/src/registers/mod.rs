//! Implements the register interfaces to the FPGA.
//!

mod native_types;
pub use native_types::*;

use crate::error::Result;

type RegisterAddress = u32;

/// Provides a binding to a register address including a type.
///
/// By generating these as part of an initialisation step - the registers can then be accessed safely at later steps knowing the address and types are matched.
pub struct Register<T> {
    address: RegisterAddress,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Default + Copy> Register<T> {
    pub const fn new(address: RegisterAddress) -> Self {
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

pub struct ArrayRegister<T, const N: usize> {
    address: RegisterAddress,
    phantom_type: std::marker::PhantomData<T>,
}

impl<T: Default + Copy, const N: usize> ArrayRegister<T, N> {
    pub const fn new(address: RegisterAddress) -> Self {
        Self {
            address,
            phantom_type: std::marker::PhantomData,
        }
    }

    pub fn read(&self, session: &impl RegisterInterface<T>) -> Result<[T; N]> {
        session.read_array(self.address)
    }

    pub fn write(&self, session: &impl RegisterInterface<T>, value: &[T; N]) -> Result<()> {
        session.write_array(self.address, value)
    }
}

pub trait RegisterInterface<T: Default + Copy> {
    fn read(&self, address: RegisterAddress) -> Result<T>;
    fn write(&self, address: RegisterAddress, data: T) -> Result<()>;
    fn read_array<const N: usize>(&self, address: RegisterAddress) -> Result<[T; N]> {
        let mut array: [T; N] = [T::default(); N];
        self.read_array_mut(address, &mut array)?;
        Ok(array)
    }
    fn read_array_mut<const N: usize>(
        &self,
        address: RegisterAddress,
        array: &mut [T; N],
    ) -> Result<()>;
    fn write_array<const N: usize>(&self, address: RegisterAddress, data: &[T; N]) -> Result<()>;
}

/// Used to allow the implementation of clusters.
///
/// S is size in bytes of the type.
trait CustomRegisterType<const S: usize> {
    fn from_buffer(buffer: &[u8; S]) -> Self;
    fn to_buffer(&self, buffer: &mut [u8; S]);
}
