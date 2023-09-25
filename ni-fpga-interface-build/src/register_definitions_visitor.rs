use crate::register_definitions::value_from_discriminant;

use super::register_definitions::LocationKind;
use lang_c::ast::*;
use lang_c::span::{Node, Span};
use lang_c::visit::Visit;
use std::collections::HashMap;

/// Defines a register location.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LocationDefinition {
    pub kind: LocationKind,
    pub name: String,
    pub datatype: String,
}

/// The set of registers that this visitor will return.
pub type RegisterSet = HashMap<LocationDefinition, u32>;

/// Extracts the register definitions from the AST.
pub struct RegisterDefinitionsVisitor {
    pub registers: RegisterSet,
    prefix: String,
}

impl RegisterDefinitionsVisitor {
    /// Create the new visitor with the FPGA project prefix.
    ///
    /// e.g. if the file is called `NiFpga_Main.h` then the prefix is Main.
    pub fn new(interface_name: &str) -> Self {
        Self {
            registers: HashMap::new(),
            prefix: format!("NiFpga_{interface_name}_"),
        }
    }

    fn process_enum_type<'ast>(&mut self, node: &'ast EnumType, type_name: &str) {
        let enum_name = type_name.strip_prefix(&self.prefix).unwrap();
        let (kind, type_name) = enum_name_to_types(enum_name);

        for Node { node: variant, .. } in node.enumerators.iter() {
            let ident_string = &variant.identifier.node.name;
            let name = control_indicator_name_from_full(ident_string);

            let definition = LocationDefinition {
                kind,
                name: name.to_owned(),
                datatype: type_name.to_owned(),
            };

            let assignment_express = &variant.expression.as_ref().unwrap().node;
            let value = value_from_discriminant(&assignment_express);

            self.registers.insert(definition, value);
        }
    }
}

/// Extract the terms for the register kind and return the time
/// and what is left.
fn enum_name_to_types(name: &str) -> (LocationKind, &str) {
    let mut kind = extract_type_from_start(name).unwrap();
    let mut type_name = name.strip_prefix(kind.prefix()).unwrap();

    if kind.is_array() && name.ends_with("Size") {
        kind = kind.with_size();
        type_name = type_name.strip_suffix("Size").unwrap();
    }

    return (kind, type_name);
}

/// Run through the options confirming the prefix.
fn extract_type_from_start(name: &str) -> Option<LocationKind> {
    // Must go from more specific to more general to avoid
    // false matches.
    // Also no point in including the size since that is marked
    // by a suffix.
    let options = [
        LocationKind::ControlArray,
        LocationKind::IndicatorArray,
        LocationKind::Control,
        LocationKind::Indicator,
    ];

    for kind in options {
        if name.starts_with(kind.prefix()) {
            return Some(kind);
        }
    }

    return None;
}

/// Extract the name of the individual control which is
/// the text after the last '_' in the name.
fn control_indicator_name_from_full(full_name: &str) -> &str {
    full_name.rsplit_once('_').unwrap().1
}

/// Check if the declaration is a typedef.
fn is_typedef(node: &Declaration) -> bool {
    for specifier in &node.specifiers {
        match specifier.node {
            DeclarationSpecifier::StorageClass(Node {
                node: StorageClassSpecifier::Typedef,
                ..
            }) => return true,
            _ => (),
        }
    }
    return false;
}

/// Extract the name of a typedef declaration.
///
/// If we can find an identifier then we return none.
fn get_typedef_name(node: &Declaration) -> Option<String> {
    for declarator in node.declarators.iter() {
        match &declarator.node.declarator.node.kind.node {
            DeclaratorKind::Identifier(identifier) => return Some(identifier.node.name.clone()),
            _ => (),
        }
    }
    None
}

impl<'ast> Visit<'ast> for RegisterDefinitionsVisitor {
    fn visit_declaration(&mut self, declaration: &'ast Declaration, _span: &'ast Span) {
        if is_typedef(declaration) {
            if let Some(name) = get_typedef_name(declaration) {
                for specifier in declaration.specifiers.iter() {
                    match &specifier.node {
                        DeclarationSpecifier::TypeSpecifier(Node {
                            node: TypeSpecifier::Enum(node),
                            ..
                        }) => self.process_enum_type(&node.node, &name),
                        _ => (),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lang_c::driver::{parse_preprocessed, Config};
    use lang_c::visit::Visit;

    fn visit_c_code(content: &str, visitor: &mut RegisterDefinitionsVisitor) {
        let config = Config::default();
        let file = parse_preprocessed(&config, content.to_owned()).unwrap();
        visitor.visit_translation_unit(&file.unit);
    }

    #[test]
    fn no_definitions_in_use() {
        let content = r#"

            static const char* const NiFpga_Main_Signature = "E3E0C23C5F01C0DBA61D947AB8A8F489";

        "#;

        let mut visitor = RegisterDefinitionsVisitor::new("Main");
        visit_c_code(content, &mut visitor);
        assert_eq!(visitor.registers.len(), 0);
    }

    #[test]
    fn test_basic_control_definition() {
        let content = r#"
        typedef enum
        {
        NiFpga_Main_ControlU8_U8Control = 0x18002,
        } NiFpga_Main_ControlU8;
        "#;

        let mut visitor = RegisterDefinitionsVisitor::new("Main");
        visit_c_code(content, &mut visitor);

        let expected = vec![(
            LocationDefinition {
                kind: LocationKind::Control,
                name: "U8Control".to_owned(),
                datatype: "U8".to_owned(),
            },
            98306,
        )];

        for (key, value) in expected {
            assert_eq!(visitor.registers.get(&key).unwrap(), &value);
        }
    }

    #[test]
    fn test_different_interface_name() {
        let content = r#"
        typedef enum
        {
        NiFpga_If_ControlU8_U8Control = 0x18002,
        } NiFpga_If_ControlU8;
        "#;

        let mut visitor = RegisterDefinitionsVisitor::new("If");
        visit_c_code(content, &mut visitor);

        let expected = vec![(
            LocationDefinition {
                kind: LocationKind::Control,
                name: "U8Control".to_owned(),
                datatype: "U8".to_owned(),
            },
            98306,
        )];

        for (key, value) in expected {
            assert_eq!(visitor.registers.get(&key).unwrap(), &value);
        }
    }

    #[test]
    fn test_type_extraction() {
        let content = r#"
        typedef enum
        {
        NiFpga_Main_ControlU32_U8Control = 0x18002,
        } NiFpga_Main_ControlU32;
        "#;

        let mut visitor = RegisterDefinitionsVisitor::new("Main");
        visit_c_code(content, &mut visitor);

        let expected = vec![(
            LocationDefinition {
                kind: LocationKind::Control,
                name: "U8Control".to_owned(),
                datatype: "U32".to_owned(),
            },
            98306,
        )];

        for (key, value) in expected {
            assert_eq!(visitor.registers.get(&key).unwrap(), &value);
        }
    }

    #[test]
    fn test_multiple_of_same_type() {
        let content = r#"
            typedef enum
            {
            NiFpga_Main_ControlU8_U8Control = 0x18002,
            NiFpga_Main_ControlU8_U8Sum = 0x18006,
            } NiFpga_Main_ControlU8;
        "#;

        let mut visitor = RegisterDefinitionsVisitor::new("Main");
        visit_c_code(content, &mut visitor);

        let expected = vec![
            (
                LocationDefinition {
                    kind: LocationKind::Control,
                    name: "U8Control".to_owned(),
                    datatype: "U8".to_owned(),
                },
                98306,
            ),
            (
                LocationDefinition {
                    kind: LocationKind::Control,
                    name: "U8Sum".to_owned(),
                    datatype: "U8".to_owned(),
                },
                98310,
            ),
        ];

        for (key, value) in expected {
            assert_eq!(visitor.registers.get(&key).unwrap(), &value);
        }
    }

    #[test]
    fn test_indicators() {
        let content = r#"
        typedef enum
        {
           NiFpga_Main_IndicatorU8_U8Result = 0x1800A,
        } NiFpga_Main_IndicatorU8;
        "#;

        let mut visitor = RegisterDefinitionsVisitor::new("Main");
        visit_c_code(content, &mut visitor);

        let expected = vec![(
            LocationDefinition {
                kind: LocationKind::Indicator,
                name: "U8Result".to_owned(),
                datatype: "U8".to_owned(),
            },
            98314,
        )];

        for (key, value) in expected {
            assert_eq!(visitor.registers.get(&key).unwrap(), &value);
        }
    }

    #[test]
    fn test_control_arrays_multiple() {
        let content = r#"
            typedef enum
            {
            NiFpga_Main_ControlArrayU8_U8ControlArray = 0x18014,
            NiFpga_Main_ControlArrayU8_U8SumArray = 0x18010,
            } NiFpga_Main_ControlArrayU8;

            typedef enum
            {
            NiFpga_Main_ControlArrayU8Size_U8ControlArray = 4,
            NiFpga_Main_ControlArrayU8Size_U8SumArray = 4,
            } NiFpga_Main_ControlArrayU8Size;
        "#;

        let mut visitor = RegisterDefinitionsVisitor::new("Main");
        visit_c_code(content, &mut visitor);

        let expected = vec![
            (
                LocationDefinition {
                    kind: LocationKind::ControlArray,
                    name: "U8ControlArray".to_owned(),
                    datatype: "U8".to_owned(),
                },
                98324,
            ),
            (
                LocationDefinition {
                    kind: LocationKind::ControlArray,
                    name: "U8SumArray".to_owned(),
                    datatype: "U8".to_owned(),
                },
                98320,
            ),
            (
                LocationDefinition {
                    kind: LocationKind::ControlArraySize,
                    name: "U8ControlArray".to_owned(),
                    datatype: "U8".to_owned(),
                },
                4,
            ),
            (
                LocationDefinition {
                    kind: LocationKind::ControlArraySize,
                    name: "U8SumArray".to_owned(),
                    datatype: "U8".to_owned(),
                },
                4,
            ),
        ];

        for (key, value) in expected {
            assert_eq!(visitor.registers.get(&key).unwrap(), &value);
        }
    }

    #[test]
    fn test_indicator_arrays() {
        let content = r#"

        typedef enum
        {
           NiFpga_Main_IndicatorArrayU8_U8ResultArray = 0x1800C,
        } NiFpga_Main_IndicatorArrayU8;
        
        typedef enum
        {
           NiFpga_Main_IndicatorArrayU8Size_U8ResultArray = 4,
        } NiFpga_Main_IndicatorArrayU8Size;
        "#;

        let mut visitor = RegisterDefinitionsVisitor::new("Main");
        visit_c_code(content, &mut visitor);

        let expected = vec![
            (
                LocationDefinition {
                    kind: LocationKind::IndicatorArray,
                    name: "U8ResultArray".to_owned(),
                    datatype: "U8".to_owned(),
                },
                98316,
            ),
            (
                LocationDefinition {
                    kind: LocationKind::IndicatorArraySize,
                    name: "U8ResultArray".to_owned(),
                    datatype: "U8".to_owned(),
                },
                4,
            ),
        ];

        for (key, value) in expected {
            assert_eq!(visitor.registers.get(&key).unwrap(), &value);
        }
    }
}
