mod bindings_parser;

use std::{
    env,
    path::{Path, PathBuf},
};

pub struct FpgaCInterface {
    common_c: PathBuf,
    common_h: PathBuf,
    custom_h: PathBuf,
    custom_c: Option<PathBuf>,
    interface_name: String,
}

impl FpgaCInterface {
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
        let common_h = interface_folder.join("NiFpga.h");
        let custom_c = interface_folder.join(format!("NiFpga_{}.c", interface_name));

        let custom_c = if custom_c.exists() {
            Some(custom_c)
        } else {
            None
        };

        Self {
            common_c,
            common_h,
            custom_h: fpga_header,
            custom_c,
            interface_name,
        }
    }

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

        build.compile("fpga");
    }

    fn build_rust_interface(&self) {
        let allow_string = format!("NiFpga_{}\\w*", self.interface_name);
        let bindings = bindgen::Builder::default()
            .header(self.custom_h.as_os_str().to_str().unwrap())
            .allowlist_function(&allow_string)
            .allowlist_type(&allow_string)
            .allowlist_var(&allow_string)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .default_enum_style(bindgen::EnumVariation::Rust {
                non_exhaustive: false,
            })
            .prepend_enum_name(false)
            .generate()
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        let bindings_file = out_path.join("bindings.rs");
        let mod_path = out_path.join(format!("NiFpga_{}.rs", self.interface_name));
        bindings
            .write_to_file(&bindings_file)
            .expect("Couldn't write bindings!");

        let interface_description = bindings_parser::InterfaceDescription::parse_bindings(
            &self.interface_name,
            &std::fs::read_to_string(&bindings_file).unwrap(),
        );

        std::fs::write(&mod_path, interface_description.generate_rust_output()).unwrap();
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::build::FpgaCInterface;

    #[test]
    fn test_constructs_from_custom_header_relative_path() {
        let fpga_header = "./NiFpga_fpga.h";
        let fpga_interface = FpgaCInterface::from_custom_header(fpga_header);
        assert_eq!(fpga_interface.common_c, PathBuf::from("./NiFpga.c"));
        assert_eq!(fpga_interface.common_h, PathBuf::from("./NiFpga.h"));
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
        assert_eq!(fpga_interface.common_h, PathBuf::from("C:\\fpga\\NiFpga.h"));
        //this is none since it wont exist in test environment.
        assert_eq!(fpga_interface.custom_c, None);
        assert_eq!(
            fpga_interface.custom_h,
            PathBuf::from("C:\\fpga\\NiFpga_fpga.h")
        );
        assert_eq!(fpga_interface.interface_name, "fpga");
    }
}
