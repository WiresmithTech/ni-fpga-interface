use crate::error::FPGAError;
use crate::session::fifo_control::FifoAddress;
use crate::session::{FifoInterface, FifoReadRegion, FifoWriteRegion, NativeFpgaType, Session};
use std::marker::PhantomData;
use std::time::Duration;

pub struct ReadFifo<T: NativeFpgaType> {
    address: FifoAddress,
    phantom: PhantomData<T>,
}

// check the 'static - should never have a lifetime in this - can we remove it somehow?
impl<T: NativeFpgaType + 'static> ReadFifo<T> {
    pub const fn new(address: u32) -> Self {
        Self {
            address,
            phantom: PhantomData,
        }
    }

    pub fn read(
        &mut self,
        session: &impl FifoInterface<T>,
        timeout: Option<Duration>,
        data: &mut [T],
    ) -> Result<usize, FPGAError> {
        session.read_fifo(self.address, data, timeout)
    }

    pub fn get_read_region<'d, 's: 'd>(
        &'d mut self,
        session: &'s impl FifoInterface<T>,
        elements: usize,
        timeout: Option<Duration>,
    ) -> Result<(FifoReadRegion<'s, 'd, T>, usize), FPGAError> {
        session.zero_copy_read(self.address, elements, timeout)
    }
}

pub struct WriteFifo<T: NativeFpgaType> {
    address: u32,
    phantom: PhantomData<T>,
}

// see note on 'static above. should try to remove it.
impl<T: NativeFpgaType + 'static> WriteFifo<T> {
    pub const fn new(address: u32) -> Self {
        Self {
            address,
            phantom: PhantomData,
        }
    }

    pub fn write(
        &mut self,
        session: &impl FifoInterface<T>,
        timeout: Option<Duration>,
        data: &[T],
    ) -> Result<usize, FPGAError> {
        session.write_fifo(self.address, data, timeout)
    }

    pub fn get_write_region<'d, 's: 'd>(
        &'d mut self,
        session: &'s impl FifoInterface<T>,
        elements: usize,
        timeout: Option<Duration>,
    ) -> Result<(FifoWriteRegion<'s, 'd, T>, usize), FPGAError> {
        session.zero_copy_write(self.address, elements, timeout)
    }
}
