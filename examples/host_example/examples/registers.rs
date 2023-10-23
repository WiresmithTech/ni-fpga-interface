//! Example of reading and writing registers on the FPGA

mod fpga_defs {
    include!(concat!(env!("OUT_DIR"), "/NiFpga_Main.rs"));
}

fn main() {
    let session = host_example::connect_fpga();

    let output_reg = fpga_defs::registers::U8Control;
    let input_1_reg = fpga_defs::registers::U8Result;
    let value = input_1_reg.read(&session).unwrap();
    println!("Value: {value}");
    output_reg.write(&session, 0x55).unwrap();
    let value = input_1_reg.read(&session).unwrap();
    println!("Value: {value}");

    let output_array = fpga_defs::registers::U8ControlArray;
    let input_array = fpga_defs::registers::U8ResultArray;
    output_array.write(&session, &[1u8, 2, 3, 4]).unwrap();
    let array: [u8; 4] = input_array.read(&session).unwrap();
    println!("Array: {:?}", array);
}
