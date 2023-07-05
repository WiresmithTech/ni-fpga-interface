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
    let sig = "728411ED7A6557687BCF28DB1D70ACF2";
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
    session.close().unwrap();
}
