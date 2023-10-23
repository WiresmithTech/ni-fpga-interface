//! This module contains the interfaces for reading and writing data to the FPGA.
//!
//! There are two forms of this supported across the native data types:
//!
//! * Registers which are the front panel controls and indicators of the FPGA VI.
//! * FIFOs which are the DMA FIFOs of the FPGA VI.

use super::fifo_control::FifoAddress;
use crate::error::NiFpgaStatus;
use crate::error::{to_fpga_result, Result};
use crate::session::{Session, SessionHandle};
use crate::types::{FpgaBool, FpgaTimeoutMs};
use libc::size_t;
use paste::paste;
use std::time::Duration;

/// Marker trait for the types that are supported directly by the FPGA interface.
pub trait NativeFpgaType: Copy {}

pub type RegisterAddress = u32;

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

/// The read region is created by calling [`FifoInterface::read_no_copy`] on the FIFO interface.
///
/// This returns this structure where you can use elements to read the data from the FIFO.
///
/// When this structure is dropped the elements are released back to the FIFO automatically.
pub struct FifoReadRegion<'session, 'data, T: NativeFpgaType> {
    session: &'session Session,
    fifo: FifoAddress,
    pub elements: &'data [T],
}

impl<'s, 'd, T: NativeFpgaType> Drop for FifoReadRegion<'s, 'd, T> {
    fn drop(&mut self) {
        // Cant return result from drop so ignore it.
        let _ = self
            .session
            .release_fifo_elements(self.fifo, self.elements.len());
    }
}

/// The write region is created by calling [`FifoInterface::write_no_copy`] on the FIFO interface.
///
/// This returns this structure where you can use elements to write the data to the FIFO as a mutable slice.
///
/// When this structure is dropped the elements are released back to the FIFO automatically.
pub struct FifoWriteRegion<'session, 'data, T: NativeFpgaType> {
    session: &'session Session,
    fifo: FifoAddress,
    pub elements: &'data mut [T],
}

impl<'s, 'd, T: NativeFpgaType> Drop for FifoWriteRegion<'s, 'd, T> {
    fn drop(&mut self) {
        // Cant return result from drop so ignore it.
        let _ = self
            .session
            .release_fifo_elements(self.fifo, self.elements.len());
    }
}

pub trait FifoInterface<T: NativeFpgaType> {
    /// Reads the elements into the provided buffer up to the size of the buffer.
    ///
    /// returns the number of elements left in the buffer to read.
    fn read_fifo(
        &self,
        fifo: FifoAddress,
        buffer: &mut [T],
        timeout: Option<Duration>,
    ) -> Result<usize>;

    /// Writes the elements to the FPGA from the data slice.
    ///
    /// Returns the amount of free space in the FIFO.
    fn write_fifo(&self, fifo: FifoAddress, data: &[T], timeout: Option<Duration>)
        -> Result<usize>;

    /// Provides a region of memory to read from the FIFO.
    fn zero_copy_read(
        &self,
        fifo: FifoAddress,
        elements: usize,
        timeout: Option<Duration>,
    ) -> Result<(FifoReadRegion<T>, usize)>;

    /// Provides a region of memory to write to the FIFO.
    fn zero_copy_write(
        &self,
        fifo: FifoAddress,
        elements: usize,
        timeout: Option<Duration>,
    ) -> Result<(FifoWriteRegion<T>, usize)>;
}

/// First entry is the rust type, second is the text used for that type in the FPGA interface.
macro_rules! impl_type_session_interface {
    ($rust_type:ty, $fpga_type:literal) => {

        #[link(name = "ni_fpga")]
        extern "C" {
            paste! { fn [<NiFpga_Read $fpga_type >](session: SessionHandle, offset: u32, value: *mut $rust_type) -> NiFpgaStatus; }
            paste! { fn [<NiFpga_Write $fpga_type >](session: SessionHandle, offset: u32, value: $rust_type) -> NiFpgaStatus; }
            paste! { fn [<NiFpga_ReadArray $fpga_type >](session: SessionHandle, offset: u32, value: *mut $rust_type, size: size_t) -> NiFpgaStatus; }
            paste! { fn [<NiFpga_WriteArray $fpga_type >](session: SessionHandle, offset: u32, value: *const $rust_type, size: size_t) -> NiFpgaStatus; }
            paste! { fn [<NiFpga_ReadFifo $fpga_type >](session: SessionHandle, fifo: u32, data: *mut $rust_type, number_of_elements: size_t, timeout_ms: FpgaTimeoutMs, elements_remaining: *mut size_t) -> NiFpgaStatus; }
            paste! { fn [<NiFpga_WriteFifo $fpga_type >](session: SessionHandle, fifo: u32, data: *const $rust_type, number_of_elements: size_t, timeout_ms: FpgaTimeoutMs, elements_remaining: *mut size_t) -> NiFpgaStatus;}
            paste! { fn [<NiFpga_AcquireFifoReadElements $fpga_type >](session: SessionHandle, fifo: u32, elements: *mut *const $rust_type, elements_requested: size_t, timeout_ms: FpgaTimeoutMs, elements_acquired: *mut size_t, elements_remaining: *mut size_t) -> NiFpgaStatus; }
            paste! { fn [<NiFpga_AcquireFifoWriteElements $fpga_type >](session: SessionHandle, fifo: u32, elements: *mut *mut $rust_type, elements_requested: size_t, timeout_ms: FpgaTimeoutMs, elements_acquired: *mut size_t, elements_remaining: *mut size_t) -> NiFpgaStatus; }
        }

        paste! {
            impl NativeFpgaType for $rust_type {}

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

            impl FifoInterface<$rust_type> for Session {
                fn read_fifo(&self, fifo: u32, data: &mut [$rust_type], timeout: Option<Duration>) -> Result< usize> {
                    let mut elements_remaining: size_t = 0;
                    let return_code = unsafe {[< NiFpga_ReadFifo $fpga_type >](self.handle, fifo, data.as_mut_ptr(), data.len(), timeout.into(), &mut elements_remaining)};
                    to_fpga_result(elements_remaining, return_code)
                }
                fn write_fifo(&self, fifo: u32, data: &[$rust_type], timeout: Option<Duration>) -> Result<usize> {
                    let mut elements_remaining: size_t = 0;
                    let return_code = unsafe {[< NiFpga_WriteFifo $fpga_type >](self.handle, fifo, data.as_ptr(), data.len(), timeout.into(), &mut elements_remaining)};
                    to_fpga_result(elements_remaining, return_code)
                }
                fn zero_copy_read(&self, fifo: u32, elements: usize, timeout: Option<Duration>) -> Result<(FifoReadRegion<$rust_type>, usize)> {
                    let mut elements_acquired: size_t = 0;
                    let mut elements_remaining: size_t = 0;
                    let mut data: *const $rust_type = std::ptr::null();
                    let return_code = unsafe {[< NiFpga_AcquireFifoReadElements $fpga_type >](self.handle, fifo, &mut data, elements, timeout.into(), &mut elements_acquired, &mut elements_remaining)};
                    let read_region = FifoReadRegion{session: self, fifo, elements: unsafe {std::slice::from_raw_parts(data, elements_acquired)}};
                    to_fpga_result((read_region, elements_remaining), return_code)
                }
                fn zero_copy_write(&self, fifo: u32, elements: usize, timeout: Option<Duration>) -> Result<(FifoWriteRegion<$rust_type>, usize)> {
                    let mut elements_acquired: size_t = 0;
                    let mut elements_remaining: size_t = 0;
                    let mut data: *mut $rust_type = std::ptr::null_mut();
                    let return_code = unsafe {[< NiFpga_AcquireFifoWriteElements $fpga_type >](self.handle, fifo, &mut data, elements, timeout.into(), &mut elements_acquired, &mut elements_remaining)};
                    let write_region = FifoWriteRegion{session: self, fifo, elements: unsafe {std::slice::from_raw_parts_mut(data, elements_acquired)}};
                    to_fpga_result((write_region, elements_remaining), return_code)
                }
            }
        }
    }
}

impl_type_session_interface!(u8, "U8");
impl_type_session_interface!(u16, "U16");
impl_type_session_interface!(u32, "U32");
impl_type_session_interface!(u64, "U64");
impl_type_session_interface!(i8, "I8");
impl_type_session_interface!(i16, "I16");
impl_type_session_interface!(i32, "I32");
impl_type_session_interface!(i64, "I64");
impl_type_session_interface!(f32, "Sgl");
impl_type_session_interface!(f64, "Dbl");
impl_type_session_interface!(FpgaBool, "Bool");
