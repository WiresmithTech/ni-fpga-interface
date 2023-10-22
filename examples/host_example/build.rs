use std::env;

fn main() {
    let mut fpga_c_interface = ni_fpga_interface_build::FpgaCInterface::from_custom_header(
        "../fpga_c_interface/NiFpga_Main.h",
    );

    // Right now I'm developing on windows so detecting linux is a good option.
    // This will need to be changed to a more robust solution in case of a linux host.
    let target = env::var("TARGET").unwrap();

    match target.as_str() {
        "x86_64-unknown-linux-gnu" => {
            fpga_c_interface.sysroot("C:\\build\\2023\\x64\\sysroots\\core2-64-nilrt-linux");
        }
        "armv7-unknown-linux-gnueabi" => {
            fpga_c_interface
                .sysroot("C:\\build\\18.0\\arm\\sysroots\\cortexa9-vfpv3-nilrt-linux-gnueabi");
        }
        _ => {}
    }

    fpga_c_interface.build();
}
