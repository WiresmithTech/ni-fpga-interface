use syn::{
    visit::{self, Visit},
    ExprLit, LitByteStr,
};

pub struct ByteConstantVisitor {
    name: String,
    pub value: Option<LitByteStr>,
}

impl ByteConstantVisitor {
    pub fn new(prefix: &str, suffix: &str) -> Self {
        Self {
            name: format!("NiFpga_{prefix}_{suffix}"),
            value: None,
        }
    }
}

impl<'ast> Visit<'ast> for ByteConstantVisitor {
    fn visit_item_const(&mut self, node: &'ast syn::ItemConst) {
        if node.ident == self.name {
            match node.expr.as_ref() {
                syn::Expr::Lit(ExprLit {
                    attrs: _,
                    lit: syn::Lit::ByteStr(lit_byte_str),
                }) => {
                    self.value = Some(lit_byte_str.clone());
                }
                _ => {
                    Visit::visit_item_const(self, node);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::ByteConstantVisitor;

    #[test]
    fn test_constant_extraction_happy_path() {
        let content = r#"
        pub const NiFpga_Main_Bitfile: &[u8; 19] = b"NiFpga_Main.lvbitx\0";
        #[doc = " The signature of the FPGA bitfile."]
        pub const NiFpga_Main_Signature: &[u8; 33] = b"E3E0C23C5F01C0DBA61D947AB8A8F489\0";
        "#;

        let file = syn::parse_file(content).unwrap();
        let mut visitor = ByteConstantVisitor::new("Main", "Signature");
        visitor.visit_file(&file);

        assert_eq!(
            visitor.value.unwrap().value(),
            b"E3E0C23C5F01C0DBA61D947AB8A8F489\0"
        );
    }

    #[test]
    fn test_ignores_if_prefix_doesnt_match() {
        let content = r#"
        pub const NiFpga_Main_Bitfile: &[u8; 19] = b"NiFpga_Main.lvbitx\0";
        #[doc = " The signature of the FPGA bitfile."]
        pub const Signature: &[u8; 33] = b"E3E0C23C5F01C0DBA61D947AB8A8F489\0";
        "#;

        let file = syn::parse_file(content).unwrap();
        let mut visitor = ByteConstantVisitor::new("Main", "Signature");
        visitor.visit_file(&file);

        assert_eq!(visitor.value, None);
    }

    #[test]
    fn test_ignores_if_name_doesnt_match() {
        let content = r#"
        pub const NiFpga_Main_Bitfile: &[u8; 19] = b"NiFpga_Main.lvbitx\0";
        "#;

        let file = syn::parse_file(content).unwrap();
        let mut visitor = ByteConstantVisitor::new("Main", "Signature");
        visitor.visit_file(&file);

        assert_eq!(visitor.value, None);
    }
}
