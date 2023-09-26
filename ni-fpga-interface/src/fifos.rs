use std::marker::PhantomData;

pub struct ReadFifo<T> {
    address: u32,
    phantom: PhantomData<T>,
}

impl<T> ReadFifo<T> {
    pub const fn new(address: u32) -> Self {
        Self {
            address,
            phantom: PhantomData,
        }
    }
}

pub struct WriteFifo<T> {
    address: u32,
    phantom: PhantomData<T>,
}

impl<T> WriteFifo<T> {
    pub const fn new(address: u32) -> Self {
        Self {
            address,
            phantom: PhantomData,
        }
    }
}
