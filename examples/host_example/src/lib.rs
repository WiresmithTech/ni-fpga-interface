//! This module just holds common components
//! to be used by the examples in the examples folder.

use ni_fpga_interface::session::{NiFpgaContext, Session};
use std::path::Path;

mod fpga_defs {
    include!(concat!(env!("OUT_DIR"), "/NiFpga_Main.rs"));
}

#[cfg(target_arch = "arm")]
const RESOURCE: &str = "RIO0";
#[cfg(target_arch = "arm")]
const BITFILE: &str = "./NiFpga_Main.lvbitx";

#[cfg(not(target_arch = "arm"))]
const RESOURCE: &str = "rio://192.168.10.17/RIO0";
#[cfg(not(target_arch = "arm"))]
const BITFILE: &str = "../fpga_c_interface/NiFpga_Main.lvbitx";

pub fn connect_fpga() -> Session {
    let fpga_context = NiFpgaContext::new().unwrap();
    let bitfile = Path::new(BITFILE);
    let session = Session::new(
        &fpga_context,
        bitfile.to_str().unwrap(),
        fpga_defs::SIGNATURE,
        RESOURCE,
    )
    .unwrap();

    session
}
