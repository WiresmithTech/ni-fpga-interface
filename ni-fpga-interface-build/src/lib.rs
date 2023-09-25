mod bindings_parser;
mod custom_type_register_visitor;
mod register_definitions;
mod register_definitions_visitor;
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
        }
    }

    /// Build the C interface and generate rust bindings for it.
    pub fn build(&self) {
        self.build_lib();
        self.build_rust_interface();
    }

    fn build_lib(&self) {
        let mut build = cc::Build::new();
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

        std::fs::write(&mod_path, interface_description.generate_rust_output()).unwrap();
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
