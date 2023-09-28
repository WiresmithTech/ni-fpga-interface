//! This example demonstrates the zero copy interface to the DMAs.
//!
//! This involves getting a read or write region from the DMA and writing to that.
//!
//! When it is dropped then that memory is committed back to the DMA.

mod fpga_defs {
    include!(concat!(env!("OUT_DIR"), "/NiFpga_Main.rs"));
}

fn main() {
    let session = host_example::connect_fpga();

    // The FPGA bit file should take a stream of u32s and return the lower half of each.

    let inputs = [0x12345678, 0x9ABCDEF0, 0x13579BDF, 0x2468ACE0];
    let expected_outputs = [0x5678, 0xDEF0, 0x9BDF, 0xACE0];

    let mut to_fpga_fifo = fpga_defs::fifos::NumbersToFPGA;
    let mut from_fpga_fifo = fpga_defs::fifos::NumbersFromFPGA;

    println!("Writing to FIFO");
    let (write_region, input_remaining) = to_fpga_fifo.get_write_region(&session, 4, None).unwrap();
    println!("{input_remaining} free space in input FIFO");
    write_region.elements.copy_from_slice(&inputs);
    drop(write_region);

    println!("Reading from FIFO");
    let mut outputs = vec![0u16; 4];

    // Read into a mutable buffer of the data type. Notice here we are just using the slice
    // of the first 2 elements indicating we want to read 2 elements.
    let (read_region, output_remaining) =
        from_fpga_fifo.get_read_region(&session, 2, None).unwrap();
    assert_eq!(output_remaining, 2);
    println!("{output_remaining} elements remaining in output FIFO after first read");
    outputs[0..2].copy_from_slice(read_region.elements);
    drop(read_region);

    // This will read the next two elements into the rest of the buffer.
    let (read_region, output_remaining) =
        from_fpga_fifo.get_read_region(&session, 2, None).unwrap();
    assert_eq!(output_remaining, 0);
    outputs[2..4].copy_from_slice(read_region.elements);

    assert_eq!(outputs, expected_outputs);

    //Last read region is implicitly dropped here.
}
