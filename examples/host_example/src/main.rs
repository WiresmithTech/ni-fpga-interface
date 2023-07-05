use ni_fpga_interface::{
    registers::{Register, RegisterInterface},
    session::Session,
};
use std::path::Path;

extern "C" {
    fn NiFpga_Initialize() -> i32;
}

fn main() {
    let bitfile = Path::new("../fpga_c_interface/NiFpga_Main.lvbitx");
    let sig = "E3E0C23C5F01C0DBA61D947AB8A8F489";
    let mut session =
        Session::new(bitfile.to_str().unwrap(), sig, "rio://192.168.10.17/RIO0").unwrap();
    //session.run().unwrap();

    let output_reg = Register::<u8>::new(0x1800A);
    let input_1_reg = Register::<u8>::new(0x18002);
    let value = output_reg.read(&session).unwrap();
    println!("Value: {value}");
    input_1_reg.write(&session, 0x55).unwrap();
    let value = output_reg.read(&session).unwrap();
    println!("Value: {value}");

    session.write_array(0x18014, &[1u8, 2, 3, 4]).unwrap();
    let array: [u8; 4] = session.read_array(0x1800C).unwrap();
    println!("Array: {:?}", array);
    session.close().unwrap();
}
