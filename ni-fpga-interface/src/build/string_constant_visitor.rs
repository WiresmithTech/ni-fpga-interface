use lang_c::ast::*;
use lang_c::span::{Node, Span};
use lang_c::visit::Visit;

pub struct StringConstantVisitor {
    name: String,
    pub value: Option<String>,
}

impl StringConstantVisitor {
    pub fn new(prefix: &str, suffix: &str) -> Self {
        Self {
            name: format!("NiFpga_{prefix}_{suffix}"),
            value: None,
        }
    }
}

/// Extract the constant value from the AST initialiser.
///
/// If the initalizer is not a string literal, then return None.
fn string_constant_from_initializer(initializer: &Initializer) -> Option<&str> {
    println!("Initializer: {:?}", initializer);
    match initializer {
        Initializer::Expression(expression) => match &expression.node {
            Expression::StringLiteral(constant) => {
                let literal = &constant.node[0];
                let trimmed = literal.trim_start_matches('"');
                let trimmed = trimmed.trim_end_matches('"');
                Some(trimmed)
            }
            _ => None,
        },
        _ => None,
    }
}

/// Extract the identifier from the AST init declarator.
fn identifier_from_init_declarator(declarator: &InitDeclarator) -> Option<&String> {
    match &declarator.declarator.node.kind.node {
        DeclaratorKind::Identifier(identifier) => Some(&identifier.node.name),
        _ => None,
    }
}

impl<'ast> Visit<'ast> for StringConstantVisitor {
    fn visit_init_declarator(&mut self, declaration: &'ast InitDeclarator, _span: &'ast Span) {
        // If we've already found the value, then we can stop.
        if self.value.is_some() {
            return;
        }

        if let Some(name) = identifier_from_init_declarator(declaration) {
            if name == &self.name {
                if let Some(Node {
                    node: initializer, ..
                }) = &declaration.initializer
                {
                    self.value =
                        string_constant_from_initializer(initializer).map(|s| s.to_owned());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StringConstantVisitor;

    use lang_c::driver::{parse_preprocessed, Config};
    use lang_c::visit::Visit;

    fn visit_c_code(content: &str, visitor: &mut StringConstantVisitor) {
        let config = Config::default();
        let file = parse_preprocessed(&config, content.to_owned()).unwrap();
        visitor.visit_translation_unit(&file.unit);
    }

    #[test]
    fn test_constant_extraction_happy_path() {
        let content = r#"
        static const char* const NiFpga_Main_Signature = "E3E0C23C5F01C0DBA61D947AB8A8F489";

        "#;

        let mut visitor = StringConstantVisitor::new("Main", "Signature");
        visit_c_code(content, &mut visitor);

        assert_eq!(&visitor.value.unwrap(), "E3E0C23C5F01C0DBA61D947AB8A8F489");
    }

    #[test]
    fn test_ignores_if_prefix_doesnt_match() {
        let content = r#"
        static const char* const Signature = "E3E0C23C5F01C0DBA61D947AB8A8F489";


        "#;

        let mut visitor = StringConstantVisitor::new("Main", "Signature");
        visit_c_code(content, &mut visitor);

        assert_eq!(visitor.value, None);
    }

    #[test]
    fn test_ignores_if_name_doesnt_match() {
        let content = r#"
        static const char* const NiFpga_Main_Test = "E3E0C23C5F01C0DBA61D947AB8A8F489";

        "#;

        let mut visitor = StringConstantVisitor::new("Main", "Signature");
        visit_c_code(content, &mut visitor);

        assert_eq!(visitor.value, None);
    }
}
