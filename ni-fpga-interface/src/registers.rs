//! Implements the register interfaces to the FPGA.
//!
use crate::error::{to_fpga_result, NiFpga_Status, Result};
use crate::session::{Session, SessionHandle};
use libc::size_t;
use paste::paste;

type RegisterAddress = u32;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct FpgaBool(u8);

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

/// First entry is the rust type, second is the text used for that type in the FPGA interface.
macro_rules! impl_register_interface {
    ($rust_type:ty, $fpga_type:literal) => {

        extern "C" {
            paste! { fn [<NiFpga_Read $fpga_type >](session: SessionHandle, offset: u32, value: *mut $rust_type) -> NiFpga_Status; }
            paste! { fn [<NiFpga_Write $fpga_type >](session: SessionHandle, offset: u32, value: $rust_type) -> NiFpga_Status; }
            paste! { fn [<NiFpga_ReadArray $fpga_type >](session: SessionHandle, offset: u32, value: *mut $rust_type, size: size_t) -> NiFpga_Status; }
            paste! { fn [<NiFpga_WriteArray $fpga_type >](session: SessionHandle, offset: u32, value: *const $rust_type, size: size_t) -> NiFpga_Status; }
        }

paste! {
        impl RegisterInterface<$rust_type> for Session {
            fn read(&self, address: RegisterAddress) -> Result<$rust_type> {
                let mut value: $rust_type = $rust_type::default();
                let return_code = unsafe {[< NiFpga_Read $fpga_type >](self.handle, address, &mut value)};
                to_fpga_result(value, return_code)
            }
            fn write(&self, address: RegisterAddress, value: $rust_type) -> Result<()> {
                let return_code = unsafe {[< NiFpga_Write $fpga_type >](self.handle, address, value)};
                to_fpga_result((), return_code)
            }
            fn read_array_mut<const N:usize>(&self, address: RegisterAddress, array: &mut [$rust_type; N]) -> Result<()> {
                let return_code = unsafe {[< NiFpga_ReadArray $fpga_type >](self.handle, address, array.as_mut_ptr(), N)};
                to_fpga_result((), return_code)
            }
            fn write_array<const N:usize>(&self, address: RegisterAddress, value: &[$rust_type;N]) -> Result<()> {
                let return_code = unsafe {[< NiFpga_WriteArray $fpga_type >](self.handle, address, value.as_ptr(), N)};
                to_fpga_result((), return_code)
            }
        }
    }
    };
}

impl_register_interface!(u8, "U8");
impl_register_interface!(u16, "U16");
impl_register_interface!(u32, "U32");
impl_register_interface!(u64, "U64");
impl_register_interface!(i8, "I8");
impl_register_interface!(i16, "I16");
impl_register_interface!(i32, "I32");
impl_register_interface!(i64, "I64");
impl_register_interface!(f32, "Sgl");
impl_register_interface!(f64, "Dbl");
impl_register_interface!(FpgaBool, "Bool");
