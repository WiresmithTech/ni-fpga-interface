use ni_fpga_interface::session::Session;
use std::path::Path;

mod fpga_defs {
    include!(concat!(env!("OUT_DIR"), "/NiFpga_Main.rs"));
}

fn main() {
    let bitfile = Path::new("../fpga_c_interface/NiFpga_Main.lvbitx");
    let session = Session::new(
        bitfile.to_str().unwrap(),
        fpga_defs::SIGNATURE,
        "rio://192.168.10.17/RIO0",
    )
    .unwrap();
    //session.run().unwrap();

    let output_reg = fpga_defs::registers::U8Control;
    let input_1_reg = fpga_defs::registers::U8Result;
    let value = output_reg.read(&session).unwrap();
    println!("Value: {value}");
    input_1_reg.write(&session, 0x55).unwrap();
    let value = output_reg.read(&session).unwrap();
    println!("Value: {value}");

    let output_array = fpga_defs::registers::U8ControlArray;
    let input_array = fpga_defs::registers::U8ResultArray;
    output_array.write(&session, &[1u8, 2, 3, 4]).unwrap();
    let array: [u8; 4] = input_array.read(&session).unwrap();
    println!("Array: {:?}", array);
    session.close().unwrap();
}
