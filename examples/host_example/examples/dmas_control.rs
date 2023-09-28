//! This example highlights the control methods for the DMA FIFOs.
//!
//! To use these you need to also import the FIFOs trait.

use ni_fpga_interface::fifos::Fifo;

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

    //stop the read FIFO for testing.
    from_fpga_fifo.stop(&session).unwrap();

    let actual_depth = to_fpga_fifo.configure(&session, 1024).unwrap();
    // Must call start to apply the configuration.
    to_fpga_fifo.start(&session).unwrap();
    let space_available = to_fpga_fifo.space_available(&session).unwrap();
    assert!(actual_depth == space_available);

    println!("Writing to FIFO");
    let input_remaining = to_fpga_fifo.write(&session, None, &inputs).unwrap();
    println!("{input_remaining} free space in input FIFO");
    assert!(input_remaining == actual_depth - inputs.len());

    println!("Reading from FIFO");
    let mut outputs = vec![0u16; 4];

    from_fpga_fifo.start(&session).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    let elements_available = from_fpga_fifo.elements_available(&session).unwrap();
    assert!(elements_available > 0);

    // This will read the next two elements into the rest of the buffer.
    let output_remaining = from_fpga_fifo
        .read(&session, None, &mut outputs[0..4])
        .unwrap();
    assert_eq!(output_remaining, 0);

    assert_eq!(outputs, expected_outputs);
}
