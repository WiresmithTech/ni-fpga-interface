//! Provides the high level interface for DMA FIFOs.

use crate::error::FPGAError;
use crate::nifpga_sys::*;
use crate::session::{FifoInterface, FifoReadRegion, FifoWriteRegion, NativeFpgaType, Session};
use std::marker::PhantomData;
use std::time::Duration;

/// The elements that are common between read and write FIFOs.
pub trait Fifo {
    fn address(&self) -> FifoAddress;

    /// Begins DMA data transfer between the FPGA target and the host computer. This method is optional.
    /// This method is optional as the DMA is automatically started when you attempt to read or write.
    ///
    /// You might want to use this method if:
    /// *  you want to start data transfer with the DMA FIFO before you read the first element of the FIFO.
    /// * You have reconfigured the FIFO with [`Fifo::configure`] and you want to force the buffer size to be committed.
    ///
    /// ```rust
    /// # use ni_fpga_interface::fifos::{ ReadFifo, Fifo};
    /// # use ni_fpga_interface::session::Session;
    ///
    ///
    /// let session = Session::new("main.lvbitx", "sig", "RIO0").unwrap();
    /// let mut fifo = ReadFifo::<u64>::new(1);
    /// fifo.start(&session).unwrap();
    ///
    /// ```
    fn start(&mut self, session: &Session) -> Result<(), FPGAError> {
        session.start_fifo(self.address())
    }

    /// Stops the DMA data transfer between the FPGA target and the host computer.
    /// This method deletes all data from the host memory and FPGA parts of the FIFO.
    ///
    /// This method is optional. Most applications do not require using the Stop method.
    /// ```rust
    /// # use ni_fpga_interface::fifos::{ ReadFifo, Fifo};
    /// # use ni_fpga_interface::session::Session;
    ///
    ///
    /// let session = Session::new("main.lvbitx", "sig", "RIO0").unwrap();
    /// let mut fifo = ReadFifo::<u64>::new(1);
    /// fifo.stop(&session).unwrap();
    ///
    /// ```
    fn stop(&mut self, session: &Session) -> Result<(), FPGAError> {
        session.stop_fifo(self.address())
    }

    /// Specifies the capacity, or depth, in elements of the host FIFO of the DMA channel.
    /// The new depth is implemented when the next FIFO Start, FIFO Read, or FIFO Write method executes.
    /// Before the new depth is set, the driver empties all data from the host memory and FPGA FIFO.
    ///
    /// This method is optional as the buffer is set by default to 10000 elements or twice the size of the FPGA buffer size.
    ///
    /// NI recommend this is set to 5 times the number of elements you specify to read and write.
    ///
    /// This method returns the actual size configured which may be larger than the request.
    ///
    /// ```rust
    /// # use ni_fpga_interface::fifos::{ ReadFifo, Fifo};
    /// # use ni_fpga_interface::session::Session;
    ///
    ///
    /// let session = Session::new("main.lvbitx", "sig", "RIO0").unwrap();
    /// let mut fifo = ReadFifo::<u64>::new(1);
    ///
    /// let configured_depth = fifo.configure(&session, 10_000).unwrap();
    /// // Start to apply the config.
    /// fifo.start(&session).unwrap();
    ///
    /// ```
    fn configure(&mut self, session: &Session, requested_depth: usize) -> Result<usize, FPGAError> {
        session.configure_fifo(self.address(), requested_depth)
    }

    fn get_peer_to_peer_fifo_endpoint(
        &self,
        session: &Session,
    ) -> Result<PeerToPeerEndpoint, FPGAError> {
        session.get_peer_to_peer_fifo_endpoint(self.address())
    }
}

/// A FIFO that can be read from.
pub struct ReadFifo<T: NativeFpgaType> {
    address: FifoAddress,
    phantom: PhantomData<T>,
}

// check the 'static - should never have a lifetime in this - can we remove it somehow?
impl<T: NativeFpgaType + 'static> ReadFifo<T> {
    pub const fn new(address: FifoAddress) -> Self {
        Self {
            address,
            phantom: PhantomData,
        }
    }

    /// Read from the FIFO into the provided buffer.
    /// The size of the read is determined by the size of the data slice.
    ///
    /// The timeout can be [`None`] to indicate an infinite timeout or a [`Duration`] to indicate a timeout.
    ///
    /// Returns the number of elements still to be read.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use ni_fpga_interface::fifos::{ ReadFifo, Fifo};
    /// # use ni_fpga_interface::session::Session;
    /// use std::time::Duration;
    ///
    /// let session = Session::new("main.lvbitx", "sig", "RIO0").unwrap();
    /// let mut fifo = ReadFifo::<u64>::new(1);
    /// let mut buffer = [0u64; 10];
    /// let remaining = fifo.read(&session, Some(Duration::from_millis(100)), &mut buffer).unwrap();
    /// ```
    pub fn read(
        &mut self,
        session: &impl FifoInterface<T>,
        timeout: Option<Duration>,
        data: &mut [T],
    ) -> Result<usize, FPGAError> {
        session.read_fifo(self.address, data, timeout)
    }

    /// Provides a mechanism to read from the FIFO without copying the data.
    ///
    /// This function returns a read region. This contains a view of the data in the DMA driver.
    /// It also returns the number of elements remaining in the buffer.
    ///
    /// The timeout can be [`None`] to indicate an infinite timeout or a [`Duration`] to indicate a timeout.
    ///
    /// To write to the FPGA you must overwrite the elements in the region and then drop it.
    ///
    /// # Example
    /// ```rust
    /// # use ni_fpga_interface::fifos::{ ReadFifo, Fifo};
    /// # use ni_fpga_interface::session::Session;
    ///
    /// let session = Session::new("main.lvbitx", "sig", "RIO0").unwrap();
    /// let mut fifo = ReadFifo::<u64>::new(1);
    /// let (read_region, remaining) = fifo.get_read_region(&session, 1000, None).unwrap();
    /// // Do something with the data in the read region.
    /// println!("{:?}", read_region.elements);
    /// // Drop the read region to commit the data back to the DMA driver.
    /// drop(read_region);
    /// ```
    pub fn get_read_region<'d, 's: 'd>(
        &'d mut self,
        session: &'s impl FifoInterface<T>,
        elements: usize,
        timeout: Option<Duration>,
    ) -> Result<(FifoReadRegion<'s, 'd, T>, usize), FPGAError> {
        session.zero_copy_read(self.address, elements, timeout)
    }

    /// Returns the number of elements available to read.
    ///
    /// Warning: This achieves this by reading zero elements from the FIFO so it will start the FIFO if stopped.
    pub fn elements_available(&self, session: &impl FifoInterface<T>) -> Result<usize, FPGAError> {
        let mut empty_buffer: [T; 0] = [];
        session.read_fifo(self.address, &mut empty_buffer, None)
    }
}

impl<T: NativeFpgaType> Fifo for ReadFifo<T> {
    fn address(&self) -> FifoAddress {
        self.address
    }
}

/// A FIFO that can be written to.
pub struct WriteFifo<T: NativeFpgaType> {
    address: FifoAddress,
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

    /// Write to the FIFO from the provided buffer.
    /// The size of the write is determined by the size of the data slice.
    ///
    /// The timeout can be [`None`] to indicate an infinite timeout or a [`Duration`] to indicate a timeout.
    ///
    /// Returns the number of elements free in the FIFO.
    ///
    /// # Example
    /// ```rust
    /// # use ni_fpga_interface::fifos::{ WriteFifo, Fifo};
    /// # use ni_fpga_interface::session::Session;
    /// use std::time::Duration;
    ///
    /// let session = Session::new("main.lvbitx", "sig", "RIO0").unwrap();
    /// let mut fifo = WriteFifo::<u64>::new(1);
    /// let buffer = [0u64; 10];
    /// let remaining = fifo.write(&session, Some(Duration::from_millis(100)), &buffer).unwrap();
    /// ```
    pub fn write(
        &mut self,
        session: &impl FifoInterface<T>,
        timeout: Option<Duration>,
        data: &[T],
    ) -> Result<usize, FPGAError> {
        session.write_fifo(self.address, data, timeout)
    }

    /// Provides a way to get a reference to the write region of the FIFO.
    ///
    /// This enables you to write into the FIFO buffer without an additional copy.
    ///
    /// This function returns a write region. This contains a view of the data in the DMA driver.
    /// It also returns the number of elements remaining in the buffer.
    ///
    /// The timeout can be [`None`] to indicate an infinite timeout or a [`Duration`] to indicate a timeout.
    ///
    /// To write to the FPGA you must overwrite the elements in the region and then drop it.
    ///
    /// # Example
    /// ```rust
    /// # use ni_fpga_interface::fifos::{ WriteFifo, Fifo};
    /// # use ni_fpga_interface::session::Session;
    ///
    /// let session = Session::new("main.lvbitx", "sig", "RIO0").unwrap();
    /// let mut fifo = WriteFifo::<u64>::new(1);
    /// let (write_region, remaining) = fifo.get_write_region(&session, 1000, None).unwrap();
    /// // Do something with the data in the write region.
    /// write_region.elements[0] = 1;
    /// // Drop the write region to commit the data back to the DMA driver.
    /// drop(write_region);
    /// ```
    pub fn get_write_region<'d, 's: 'd>(
        &'d mut self,
        session: &'s impl FifoInterface<T>,
        elements: usize,
        timeout: Option<Duration>,
    ) -> Result<(FifoWriteRegion<'s, 'd, T>, usize), FPGAError> {
        session.zero_copy_write(self.address, elements, timeout)
    }

    /// Returns the number of elements free to write.
    /// Warning: This achieves this by writing zero elements to the FIFO so it will start the FIFO if stopped.
    pub fn space_available(&self, session: &impl FifoInterface<T>) -> Result<usize, FPGAError> {
        let empty_buffer: [T; 0] = [];
        session.write_fifo(self.address, &empty_buffer, None)
    }
}

impl<T: NativeFpgaType> Fifo for WriteFifo<T> {
    fn address(&self) -> FifoAddress {
        self.address
    }
}
