use std::collections::HashMap;

use syn::{
    token::Eq,
    visit::{self, Visit},
    Expr, ItemEnum,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum LocationKind {
    Control,
    Indicator,
    ControlArray,
    IndicatorArray,
    ControlArraySize,
    IndicatorArraySize,
}

impl LocationKind {
    fn is_array(&self) -> bool {
        match self {
            LocationKind::ControlArray
            | LocationKind::IndicatorArray
            | LocationKind::ControlArraySize
            | LocationKind::IndicatorArraySize => true,
            _ => false,
        }
    }

    fn with_size(self) -> Self {
        match self {
            LocationKind::ControlArray => LocationKind::ControlArraySize,
            LocationKind::IndicatorArray => LocationKind::IndicatorArraySize,
            _ => self,
        }
    }

    const fn prefix(&self) -> &str {
        match self {
            LocationKind::Control => "Control",
            LocationKind::Indicator => "Indicator",
            LocationKind::ControlArray => "ControlArray",
            LocationKind::IndicatorArray => "IndicatorArray",
            LocationKind::ControlArraySize => "ControlArray",
            LocationKind::IndicatorArraySize => "IndicatorArray",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct LocationDefinition {
    kind: LocationKind,
    name: String,
    datatype: String,
}

pub struct RegisterDefinitionsVisitor {
    pub registers: HashMap<LocationDefinition, u32>,
    prefix: String,
}

impl RegisterDefinitionsVisitor {
    pub fn new(interface_name: &str) -> Self {
        Self {
            registers: HashMap::new(),
            prefix: format!("NiFpga_{interface_name}_"),
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

fn value_from_discriminant(discriminant: &(Eq, Expr)) -> u32 {
    match &discriminant.1 {
        Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Int(int) => int.base10_parse().unwrap(),
            _ => panic!("Unexpected literal type"),
        },
        _ => panic!("Unexpected expression type"),
    }
}

impl<'ast> Visit<'ast> for RegisterDefinitionsVisitor {
    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        if let Some(enum_name) = &node.ident.to_string().strip_prefix(&self.prefix) {
            let (kind, type_name) = enum_name_to_types(enum_name);

            for variant in node.variants.iter() {
                let ident_string = variant.ident.to_string();
                let name = control_indicator_name_from_full(&ident_string);

                let definition = LocationDefinition {
                    kind,
                    name: name.to_owned(),
                    datatype: type_name.to_owned(),
                };

                let value = value_from_discriminant(variant.discriminant.as_ref().unwrap());

                self.registers.insert(definition, value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_definitions_in_use() {
        let content = r#"
        pub const NiFpga_Main_Bitfile: &[u8; 19] = b"NiFpga_Main.lvbitx\0";
        #[doc = " The signature of the FPGA bitfile."]
        pub const NiFpga_Main_Signature: &[u8; 33] = b"E3E0C23C5F01C0DBA61D947AB8A8F489\0";
        "#;

        let file = syn::parse_file(content).unwrap();
        let mut visitor = RegisterDefinitionsVisitor::new("Main");
        visitor.visit_file(&file);

        assert_eq!(visitor.registers.len(), 0);
    }

    #[test]
    fn test_basic_control_definition() {
        let content = r#"
            #[repr(i32)]
            #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
            pub enum NiFpga_Main_ControlU8 {
                NiFpga_Main_ControlU8_U8Control = 98306,
            }
        "#;

        let file = syn::parse_file(content).unwrap();
        let mut visitor = RegisterDefinitionsVisitor::new("Main");
        visitor.visit_file(&file);

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
            #[repr(i32)]
            #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
            pub enum NiFpga_If_ControlU8 {
                NiFpga_If_ControlU8_U8Control = 98306,
            }
        "#;

        let file = syn::parse_file(content).unwrap();
        let mut visitor = RegisterDefinitionsVisitor::new("If");
        visitor.visit_file(&file);

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
            #[repr(i32)]
            #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
            pub enum NiFpga_Main_ControlU32 {
                NiFpga_Main_ControlU32_U8Control = 98306,
            }
        "#;

        let file = syn::parse_file(content).unwrap();
        let mut visitor = RegisterDefinitionsVisitor::new("Main");
        visitor.visit_file(&file);

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
            #[repr(i32)]
            #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
            pub enum NiFpga_Main_ControlU8 {
                NiFpga_Main_ControlU8_U8Control = 98306,
                NiFpga_Main_ControlU8_U8Sum = 98310,
            }
        "#;

        let file = syn::parse_file(content).unwrap();
        let mut visitor = RegisterDefinitionsVisitor::new("Main");
        visitor.visit_file(&file);

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
            #[repr(i32)]
            #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
            pub enum NiFpga_Main_IndicatorU8 {
                NiFpga_Main_IndicatorU8_U8Result = 98314,
            }
        "#;

        let file = syn::parse_file(content).unwrap();
        let mut visitor = RegisterDefinitionsVisitor::new("Main");
        visitor.visit_file(&file);

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
        #[repr(i32)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub enum NiFpga_Main_ControlArrayU8 {
            NiFpga_Main_ControlArrayU8_U8ControlArray = 98324,
            NiFpga_Main_ControlArrayU8_U8SumArray = 98320,
        }
        impl NiFpga_Main_ControlArrayU8Size {
            pub const NiFpga_Main_ControlArrayU8Size_U8SumArray: NiFpga_Main_ControlArrayU8Size =
                NiFpga_Main_ControlArrayU8Size::NiFpga_Main_ControlArrayU8Size_U8ControlArray;
        }
        #[repr(i32)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub enum NiFpga_Main_ControlArrayU8Size {
            NiFpga_Main_ControlArrayU8Size_U8ControlArray = 4,
        }
        "#;

        let file = syn::parse_file(content).unwrap();
        let mut visitor = RegisterDefinitionsVisitor::new("Main");
        visitor.visit_file(&file);

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
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub enum NiFpga_Main_IndicatorArrayU8 {
        NiFpga_Main_IndicatorArrayU8_U8ResultArray = 98316,
    }
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub enum NiFpga_Main_IndicatorArrayU8Size {
        NiFpga_Main_IndicatorArrayU8Size_U8ResultArray = 4,
    }
        "#;

        let file = syn::parse_file(content).unwrap();
        let mut visitor = RegisterDefinitionsVisitor::new("Main");
        visitor.visit_file(&file);

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
