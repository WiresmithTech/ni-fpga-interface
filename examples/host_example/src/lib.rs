//! This module just holds common components
//! to be used by the examples in the examples folder.

use ni_fpga_interface::session::Session;
use std::path::Path;

mod fpga_defs {
    include!(concat!(env!("OUT_DIR"), "/NiFpga_Main.rs"));
}

pub fn connect_fpga() -> Session {
    let bitfile = Path::new("../fpga_c_interface/NiFpga_Main.lvbitx");
    let session = Session::new(
        bitfile.to_str().unwrap(),
        fpga_defs::SIGNATURE,
        "rio://192.168.10.17/RIO0",
    )
    .unwrap();

    session
}
