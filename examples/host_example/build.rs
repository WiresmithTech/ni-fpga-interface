fn main() {
    let fpga_c_interface = ni_fpga_interface_build::FpgaCInterface::from_custom_header(
        "../fpga_c_interface/NiFpga_Main.h",
    );
    fpga_c_interface.build();
}
