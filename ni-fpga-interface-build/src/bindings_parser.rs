//! Generates a new rust module which contains the interface to the FPGA.
//!
//! This is still in rough shape but seems to prove the basic concept.

use super::address_definitions_visitor::AddressDefinitionsVisitor;
use super::registers_generator::{generate_fifo_module, generate_register_module};
use super::{
    address_definitions_visitor::AddressSet, string_constant_visitor::StringConstantVisitor,
};
use lang_c::driver::{parse, parse_preprocessed, Config};
use lang_c::visit::Visit;
use quote::{quote, ToTokens};
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub struct InterfaceDescription {
    pub signature: String,
    pub registers: AddressSet,
}

impl InterfaceDescription {
    /// Parses the C header file for the specific FPGA interface.
    pub fn parse_bindings(prefix: &str, content: &Path) -> Self {
        let new_path = header_to_temp_no_includes(content);
        let mut config = Config::default();
        //use cc to find the best compiler.
        let build = cc::Build::new();
        config.cpp_command = build.get_compiler().path().to_str().unwrap().to_owned();
        config.cpp_options = vec!["-E".to_owned()];
        let file = parse(&config, new_path).unwrap().unit;
        read_ast(prefix, file)
    }

    /// Parses the pre-processed C header file for the specific FPGA interface.
    ///
    /// This is used for testing purposes so we don't have to rely on files.
    ///
    /// Note: preprocessed means no macros, comments etc.
    #[allow(dead_code)]
    pub fn parse_preprocessed_bindings(prefix: &str, content: String) -> Self {
        let config = Config::with_clang();
        let file = parse_preprocessed(&config, content).unwrap().unit;
        read_ast(prefix, file)
    }

    /// Generates a new rust module which contains the interface to the FPGA.
    pub fn generate_rust_output(&self) -> String {
        let metadata = self.generate_metadata_output();
        let registers = generate_register_module(&self.registers);
        let fifos = generate_fifo_module(&self.registers);
        let tokens = quote! {
            #metadata
            #registers
            #fifos
        };
        println!("{}", tokens);
        let file = syn::parse2(tokens).unwrap();
        prettyplease::unparse(&file)
    }

    fn generate_metadata_output(&self) -> impl ToTokens {
        let signature = &self.signature;
        quote! {
            #[allow(dead_code)]
            pub const SIGNATURE: &str = #signature;
        }
    }
}

/// Once the AST has been parsed, we can extract the signature and register definitions.
fn read_ast(prefix: &str, file: lang_c::ast::TranslationUnit) -> InterfaceDescription {
    let mut sig_visitor = StringConstantVisitor::new(prefix, "Signature");
    let mut register_visitor = AddressDefinitionsVisitor::new(prefix);
    sig_visitor.visit_translation_unit(&file);
    register_visitor.visit_translation_unit(&file);
    InterfaceDescription {
        signature: sig_visitor.value.expect("No signature"),
        registers: register_visitor.registers,
    }
}

/// Cludgy hack to stop pre-processor following headers
/// which are causing parsing errors. Also we don't need them.
fn header_to_temp_no_includes(header: &Path) -> PathBuf {
    use std::fs::{File, OpenOptions};
    use std::io::BufRead;
    use std::io::Write;

    let mut temp = std::env::temp_dir();
    temp.push(header.file_name().unwrap());

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&temp)
        .unwrap();

    let input = File::open(header).unwrap();

    //write in a couple of definitions we will commonly need.
    // making assumptions on short and char for the platform
    // as I believe this to be true for Windows and Linux.
    let common_types = r#"
typedef unsigned char uint8_t;
typedef short int16_t;
typedef int int32_t;
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;
typedef long long int64_t;
typedef uint8_t NiFpga_Bool;

typedef struct NiFpga_FxpTypeInfo
{
    NiFpga_Bool isSigned;
    uint8_t wordLength;
    int16_t integerWordLength;
} NiFpga_FxpTypeInfo;

"#;
    output.write_all(common_types.as_bytes()).unwrap();

    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        if !line.starts_with("#include") {
            writeln!(output, "{}", line).unwrap();
        }
    }

    temp
}

#[cfg(test)]
mod tests {
    use super::InterfaceDescription;

    #[test]
    fn test_signature_extraction() {
        let content = r#"
        const char* NiFpga_Main_Signature = "E3E0C23C5F01C0DBA61D947AB8A8F489";
        "#;

        let description =
            InterfaceDescription::parse_preprocessed_bindings("Main", content.to_owned());

        assert_eq!(description.signature, "E3E0C23C5F01C0DBA61D947AB8A8F489");
    }
}
