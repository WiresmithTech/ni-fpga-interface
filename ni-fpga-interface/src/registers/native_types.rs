//! Code to support registers for the native numeric types.

use super::{RegisterAddress, RegisterInterface};
use crate::error::NiFpgaStatus;
use crate::error::{to_fpga_result, Result};
use crate::session::{Session, SessionHandle};
use libc::size_t;
use paste::paste;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FpgaBool(u8);

/// First entry is the rust type, second is the text used for that type in the FPGA interface.
macro_rules! impl_register_interface {
    ($rust_type:ty, $fpga_type:literal) => {

        extern "C" {
            paste! { fn [<NiFpga_Read $fpga_type >](session: SessionHandle, offset: u32, value: *mut $rust_type) -> NiFpgaStatus; }
            paste! { fn [<NiFpga_Write $fpga_type >](session: SessionHandle, offset: u32, value: $rust_type) -> NiFpgaStatus; }
            paste! { fn [<NiFpga_ReadArray $fpga_type >](session: SessionHandle, offset: u32, value: *mut $rust_type, size: size_t) -> NiFpgaStatus; }
            paste! { fn [<NiFpga_WriteArray $fpga_type >](session: SessionHandle, offset: u32, value: *const $rust_type, size: size_t) -> NiFpgaStatus; }
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
