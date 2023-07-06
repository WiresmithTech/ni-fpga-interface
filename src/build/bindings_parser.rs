//! Generates a new rust module which contains the interface to the FPGA.
//!
//! This is still in rough shape but seems to prove the basic concept.

use super::byte_constant_visitor::ByteConstantVisitor;
use std::ffi::CString;

use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::In,
    visit::Visit,
    ExprLit, LitByteStr,
};

pub struct InterfaceDescription {
    pub signature: LitByteStr,
    pub filename: LitByteStr,
}

impl InterfaceDescription {
    pub fn parse_bindings(prefix: &str, content: &str) -> Self {
        let file = syn::parse_file(content).unwrap();
        let mut sig_visitor = ByteConstantVisitor::new(prefix, "Signature");
        let mut bitfile_visitor = ByteConstantVisitor::new(prefix, "Bitfile");
        sig_visitor.visit_file(&file);
        bitfile_visitor.visit_file(&file);

        InterfaceDescription {
            signature: sig_visitor.value.unwrap(),
            filename: bitfile_visitor.value.unwrap(),
        }
    }

    pub fn generate_rust_output(&self) -> String {
        let signature = &self.signature;
        let signature_length = signature.value().len();
        let bitfile = &self.filename;
        let bitfile_length = bitfile.value().len();
        let file = syn::parse2(quote! {
            const SIGNATURE: [u8; #signature_length] = #signature;
            const FILENAME: [u8; #bitfile_length] = #bitfile;
        })
        .unwrap();
        prettyplease::unparse(&file)
    }
}

#[cfg(test)]
mod tests {
    use super::InterfaceDescription;

    #[test]
    fn test_signature_extraction() {
        let content = r#"
        pub const NiFpga_Main_Bitfile: &[u8; 19] = b"NiFpga_Main.lvbitx\0";
        #[doc = " The signature of the FPGA bitfile."]
        pub const NiFpga_Main_Signature: &[u8; 33] = b"E3E0C23C5F01C0DBA61D947AB8A8F489\0";
        "#;

        let description = InterfaceDescription::parse_bindings("Main", content);

        assert_eq!(
            description.signature.value(),
            b"E3E0C23C5F01C0DBA61D947AB8A8F489\0"
        );
    }

    #[test]
    fn test_filename_extraction() {
        let content = r#"
        pub const NiFpga_Main_Bitfile: &[u8; 19] = b"NiFpga_Main.lvbitx\0";
        #[doc = " The signature of the FPGA bitfile."]
        pub const NiFpga_Main_Signature: &[u8; 33] = b"E3E0C23C5F01C0DBA61D947AB8A8F489\0";
        "#;

        let description = InterfaceDescription::parse_bindings("Main", content);

        assert_eq!(description.filename.value(), b"NiFpga_Main.lvbitx\0");
    }
}
