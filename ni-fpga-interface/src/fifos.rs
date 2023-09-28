use crate::error::FPGAError;
use crate::session::fifo_control::FifoAddress;
use crate::session::{FifoInterface, NativeFpgaType, Session};
use std::marker::PhantomData;
use std::time::Duration;

pub struct ReadFifo<T: NativeFpgaType> {
    address: FifoAddress,
    phantom: PhantomData<T>,
}

impl<T: NativeFpgaType> ReadFifo<T> {
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
        session.read(self.address, data, timeout)
    }
}

pub struct WriteFifo<T: NativeFpgaType> {
    address: u32,
    phantom: PhantomData<T>,
}

impl<T: NativeFpgaType> WriteFifo<T> {
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
        session.write(self.address, data, timeout)
    }
}
