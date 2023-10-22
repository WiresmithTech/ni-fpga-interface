//! The NI Fpga Interface Builder is used to generate a Rust module
//! with the definition of the interface of a given FPGA bitfile.
//!
//! This depends on the NI Fpga Interface crate and generates the
//! definitions that it can use.
//!
//! # Example
//!
//! ```no_run
//! use ni_fpga_interface_build::FpgaCInterface;
//!
//! FpgaCInterface::from_custom_header("NiFpga_prefix.h").build();
//! ```
//!
//! This will generate a file in the output directory with the name `NiFpga_Main.rs`.
//!
//! This will contain a module with the definitions of registers and DMA FIFOs.
//!
//! # Example Output
//!
//! ```rust,ignore
//! pub const SIGNATURE: &str = "A0613989B20F45FC6E79EB71383493E8";
//!
//! pub mod registers {
//!     use ni_fpga_interface::registers::{ArrayRegister, Register};
//!     pub const SglSumArray: ArrayRegister<f32, 4> = ArrayRegister::new(0x1801C);
//!     pub const U8ControlArray: ArrayRegister<u8, 4> = ArrayRegister::new(0x18014);
//!     pub const U8SumArray: ArrayRegister<u8, 4> = ArrayRegister::new(0x18010);
//!     pub const SglControl: Register<f32> = Register::new(0x1802C);
//!     pub const U8Sum: Register<u8> = Register::new(0x18006);
//!     pub const U8Control: Register<u8> = Register::new(0x18002);
//!     pub const SglSum: Register<f32> = Register::new(0x18028);
//!     pub const SglResult: Register<f32> = Register::new(0x18024);
//!     pub const SglResultArray: ArrayRegister<f32, 4> = ArrayRegister::new(0x18018);
//!     pub const IRQs: Register<u32> = Register::new(0x18060);
//!     pub const U8Result: Register<u8> = Register::new(0x1800A);
//!     pub const U8ResultArray: ArrayRegister<u8, 4> = ArrayRegister::new(0x1800C);
//!     pub const SglControlArray: ArrayRegister<f32, 4> = ArrayRegister::new(0x18020);
//! }
//!
//! pub mod fifos {
//!     use ni_fpga_interface::fifos::{ReadFifo, WriteFifo};
//!     pub const NumbersFromFPGA: ReadFifo<u16> = ReadFifo::new(0x1);
//!     pub const NumbersToFPGA: WriteFifo<u32> = WriteFifo::new(0x0);
//! }
//! ```
//!
//! To then use this in your system you can import it into a module.
//!
//! ```rust,ignore
//! mod fpga_defs {
//!    include!(concat!(env!("OUT_DIR"), "/NiFpga_Main.rs"));
//!}
//! ```

mod address_definitions;
mod address_definitions_visitor;
mod bindings_parser;
mod custom_type_register_visitor;
mod registers_generator;
mod string_constant_visitor;

use std::{
    env,
    path::{Path, PathBuf},
};

/// Defines the generated C interface for the FPGA project.
pub struct FpgaCInterface {
    common_c: PathBuf,
    custom_h: PathBuf,
    custom_c: Option<PathBuf>,
    interface_name: String,
    sysroot: Option<String>,
}

impl FpgaCInterface {
    /// Constructs a new interface from the given custom header.
    ///
    /// This is the header file which includes the project specific prefix.
    /// e.g. NiFpga_prefix.h not NiFpga.h.
    ///
    /// This finds the other files assuming they are in the same folder.
    pub fn from_custom_header(fpga_header: impl AsRef<Path>) -> Self {
        let fpga_header = fpga_header.as_ref();
        let fpga_header = fpga_header.to_owned();
        let interface_folder = fpga_header.parent().unwrap();
        let interface_name = fpga_header
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_prefix("NiFpga_")
            .unwrap()
            .to_owned();

        let common_c = interface_folder.join("NiFpga.c");
        let custom_c = interface_folder.join(format!("NiFpga_{}.c", interface_name));

        let custom_c = if custom_c.exists() {
            Some(custom_c)
        } else {
            None
        };

        Self {
            common_c,
            custom_h: fpga_header,
            custom_c,
            interface_name,
            sysroot: None,
        }
    }

    /// Sets the sysroot for the C compiler.
    /// This is useful for cross compiling.
    /// ```no_run
    /// use ni_fpga_interface_build::FpgaCInterface;
    /// FpgaCInterface::from_custom_header("NiFpga_prefix.h")
    ///    .sysroot("C:\\build\\2023\\x64\\sysroots\\core2-64-nilrt-linux")
    ///   .build();
    /// ```
    pub fn sysroot(&mut self, sysroot: impl Into<String>) -> &mut Self {
        self.sysroot = Some(sysroot.into());
        self
    }

    /// Build the C interface and generate rust bindings for it.
    pub fn build(&self) {
        self.build_lib();
        self.build_rust_interface();
    }

    fn build_lib(&self) {
        let mut build = cc::Build::new();

        if let Some(path) = &self.sysroot {
            build.flag(&format!("--sysroot={path}"));
        }

        build.file(&self.common_c);

        if let Some(custom_c) = &self.custom_c {
            build.file(custom_c);
        }

        build.compile("ni_fpga");
    }

    fn build_rust_interface(&self) {
        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        let mod_path = out_path.join(format!("NiFpga_{}.rs", self.interface_name));
        println!("cargo:rerun-if-changed={}", self.custom_h.display());

        let interface_description = bindings_parser::InterfaceDescription::parse_bindings(
            &self.interface_name,
            &PathBuf::from(&self.custom_h),
        );

        std::fs::write(mod_path, interface_description.generate_rust_output()).unwrap();
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::FpgaCInterface;

    #[test]
    fn test_constructs_from_custom_header_relative_path() {
        let fpga_header = "./NiFpga_fpga.h";
        let fpga_interface = FpgaCInterface::from_custom_header(fpga_header);
        assert_eq!(fpga_interface.common_c, PathBuf::from("./NiFpga.c"));
        //this is none since it wont exist in test environment.
        assert_eq!(fpga_interface.custom_c, None);
        assert_eq!(fpga_interface.custom_h, PathBuf::from("./NiFpga_fpga.h"));
        assert_eq!(fpga_interface.interface_name, "fpga");
    }

    #[test]
    fn test_constructs_from_custom_header_absolute_path() {
        let fpga_header = "C:\\fpga\\NiFpga_fpga.h";
        let fpga_interface = FpgaCInterface::from_custom_header(fpga_header);
        assert_eq!(fpga_interface.common_c, PathBuf::from("C:\\fpga\\NiFpga.c"));
        //this is none since it wont exist in test environment.
        assert_eq!(fpga_interface.custom_c, None);
        assert_eq!(
            fpga_interface.custom_h,
            PathBuf::from("C:\\fpga\\NiFpga_fpga.h")
        );
        assert_eq!(fpga_interface.interface_name, "fpga");
    }
}
