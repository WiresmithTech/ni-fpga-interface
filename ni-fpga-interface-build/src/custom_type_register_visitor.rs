//! This module handles visiting the definitions of
//! registers for "custom types" including:
//!
//! - FXP Number
//! - Clusters
//!
//! These differ from normal registers in using a series of
//! constants rather than enums.
//!
//! What is expected is:
//!
//! - _TypeInfo constant for FXP.
//! - _Type for clusters.
//! - _Resource constant for the address.
//! - _Size constant for arrays.
//! - _PackedSizeInBytes for clusters.
//!
//! Similar clusters will have their own types.
//!
//!

use std::{collections::HashMap, hash::Hash};
use thiserror::Error;

use lang_c::{
    ast::{
        Declaration, DeclarationSpecifier, DeclaratorKind, Expression, Initializer, TypeQualifier,
    },
    visit::Visit,
};

use crate::address_definitions::value_from_discriminant;

#[derive(Debug, Error)]
pub enum CustomTypeVisitorError {
    #[error("Did not find address for {0}")]
    MissingAddress(String),
    #[error("Missing declarator for {0:?} when getting initializer")]
    MissingDeclaratorForInitializer(Declaration),
    #[error("Missing initializer for {0:?}")]
    MissingInitializer(Declaration),
    #[error("Missing initializer in fxp item")]
    UnexpectedNestingInFxpItem,
    #[error("Insufficient Items in Fxp Initializer")]
    InsufficientItemsInFxpInitializer,
    #[error("Initializer for FXP type data is not a list")]
    FxpInitializerNotList,
    #[error("Unknown type field {0}")]
    UnknownTypeField(String),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct FxpTypeInfo {
    signed: bool,
    word_length: u32,
    integer_word_length: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FxpRegister {
    name: String,
    fxp_type_info: FxpTypeInfo,
    address: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ClusterRegister {
    cluster_type: String,
    address: u32,
    packed_size: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
struct CustomTypeData {
    address: Option<u32>,
    fxp_type_info: Option<FxpTypeInfo>,
    cluster_type: Option<String>,
    packed_size: Option<u32>,
    array_size: Option<u32>,
}

pub struct CustomTypeVisitor {
    prefix: String,
    types: HashMap<String, CustomTypeData>,
}

impl CustomTypeVisitor {
    pub fn new(interface_name: &str) -> Self {
        Self {
            prefix: format!("NiFpga_{}_", interface_name),
            types: Default::default(),
        }
    }

    pub fn get_registers(
        &self,
    ) -> Result<(Vec<FxpRegister>, Vec<ClusterRegister>), CustomTypeVisitorError> {
        let mut fxp_regs = Vec::new();
        for value in &self.types {
            fxp_regs.push(FxpRegister {
                name: value.0.clone(),
                fxp_type_info: value
                    .1
                    .fxp_type_info
                    .clone()
                    .expect("Should have FXP type info"),
                address: value
                    .1
                    .address
                    .ok_or(CustomTypeVisitorError::MissingAddress(value.0.clone()))?,
            });
        }

        Ok((fxp_regs, Vec::new()))
    }
}

impl<'ast> Visit<'ast> for CustomTypeVisitor {
    fn visit_declaration(
        &mut self,
        declaration: &'ast lang_c::ast::Declaration,
        span: &'ast lang_c::span::Span,
    ) {
        if !is_constant_definition(declaration) {
            return;
        }

        let name = get_constant_name(declaration);
        println!("Found constant {}", name);

        if let Some(reg_details) = get_constant_type_from_name(&self.prefix, name) {
            println!("Found {:?}", reg_details);
            if !self.types.contains_key(reg_details.control_name) {
                self.types
                    .insert(reg_details.control_name.to_owned(), Default::default());
            }

            let control_definition = self
                .types
                .get_mut(reg_details.control_name)
                .expect("Last step should have created this");

            match reg_details.suffix {
                "Resource" => {
                    let value =
                        read_init_value_number(declaration).expect("Should be a register value");
                    control_definition.address = Some(value);
                }
                "TypeInfo" => {
                    control_definition.fxp_type_info =
                        Some(read_init_fixed_type(declaration).expect("FXP type"));
                }
                _ => {}
            }
        }
    }
}

fn is_constant_definition(declaration: &lang_c::ast::Declaration) -> bool {
    for specifier in declaration.specifiers.iter() {
        match &specifier.node {
            DeclarationSpecifier::TypeQualifier(type_qualifier) => {
                if type_qualifier.node == TypeQualifier::Const {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn get_init_declarators(
    declaration: &lang_c::ast::Declaration,
) -> impl Iterator<Item = &lang_c::ast::InitDeclarator> {
    declaration
        .declarators
        .iter()
        .map(|init_declarator| &init_declarator.node)
}

fn get_constant_name(declaration: &lang_c::ast::Declaration) -> &str {
    let init_declarators = get_init_declarators(declaration);
    let mut names = init_declarators.filter_map(|init_declarator| {
        match &init_declarator.declarator.node.kind.node {
            DeclaratorKind::Identifier(identifier) => Some(identifier.node.name.as_str()),
            _ => None,
        }
    });
    names.next().expect("Must be at least one identifier")
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ConstantType<'a> {
    control_type: &'a str,
    control_name: &'a str,
    suffix: &'a str,
}

fn get_constant_type_from_name<'b>(prefix: &str, name: &'b str) -> Option<ConstantType<'b>> {
    let short_name = name.strip_prefix(prefix);

    if let Some(name) = short_name {
        let mut parts = name.split('_');
        let control_type = parts.next()?;
        let control_name = parts.next()?;
        let suffix = parts.next()?;
        if parts.next().is_some() {
            return None;
        }
        Some(ConstantType {
            control_type,
            control_name,
            suffix,
        })
    } else {
        None
    }
}

fn get_initializer(
    declaration: &lang_c::ast::Declaration,
) -> Result<&Initializer, CustomTypeVisitorError> {
    let init_declarator = get_init_declarators(declaration).next().ok_or_else(|| {
        CustomTypeVisitorError::MissingDeclaratorForInitializer(declaration.clone())
    })?;
    let initializer = init_declarator
        .initializer
        .as_ref()
        .ok_or_else(|| CustomTypeVisitorError::MissingInitializer(declaration.clone()))?;
    println!("Found initializer {:?}", initializer.node);
    Ok(&initializer.node)
}

fn get_constant_express(
    declaration: &lang_c::ast::Declaration,
) -> Result<&Expression, CustomTypeVisitorError> {
    let initializer = get_initializer(declaration)?;
    match initializer {
        lang_c::ast::Initializer::Expression(expression) => Ok(&expression.as_ref().node),
        _ => panic!("Should be a expression initializer"),
    }
}

fn read_init_value_number(
    declaration: &lang_c::ast::Declaration,
) -> Result<u32, CustomTypeVisitorError> {
    let expression = get_constant_express(declaration)?;
    Ok(value_from_discriminant(expression))
}

fn read_init_fixed_type(declaration: &Declaration) -> Result<FxpTypeInfo, CustomTypeVisitorError> {
    let initializer = get_initializer(declaration)?;
    match initializer {
        Initializer::List(items) => {
            let values: Result<Vec<u32>, CustomTypeVisitorError> = items
                .iter()
                .map(|item| {
                    let item_initializer = &item.node.initializer.as_ref().node;
                    match item_initializer {
                        Initializer::Expression(expression) => {
                            Ok(value_from_discriminant(&expression.node))
                        }
                        _ => Err(CustomTypeVisitorError::UnexpectedNestingInFxpItem),
                    }
                })
                .collect();

            let values = values?;

            if values.len() < 3 {
                return Err(CustomTypeVisitorError::InsufficientItemsInFxpInitializer);
            }

            Ok(FxpTypeInfo {
                signed: values[0] != 0,
                word_length: values[1],
                integer_word_length: values[2],
            })
        }
        _ => Err(CustomTypeVisitorError::FxpInitializerNotList),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use lang_c::driver::{parse_preprocessed, Config};
    use lang_c::visit::Visit;

    fn visit_c_code(content: &str, visitor: &mut CustomTypeVisitor) {
        let config = Config::default();
        let file = parse_preprocessed(&config, content.to_owned()).unwrap();
        visitor.visit_translation_unit(&file.unit);
    }

    #[test]
    fn test_name_deconstruction_valid() {
        let name = "NiFpga_Main_ControlFxp_FxpControl_Resource";
        let constant_types = get_constant_type_from_name("NiFpga_Main_", name);
        assert_eq!(
            constant_types,
            Some(ConstantType {
                control_type: "ControlFxp",
                control_name: "FxpControl",
                suffix: "Resource"
            })
        );
    }

    #[test]
    fn test_name_deconstruction_no_prefix() {
        let name = "ControlFxp_FxpControl_Resource";
        let constant_types = get_constant_type_from_name("NiFpga_Main_", name);
        assert!(constant_types.is_none());
    }

    #[test]
    fn test_name_deconstruction_non_structure() {
        let name = "NiFpga_Main_Wut";
        let constant_types = get_constant_type_from_name("NiFpga_Main_", name);
        assert!(constant_types.is_none());
    }

    #[ignore = "still brittle and unfinished"]
    #[test]
    fn test_fxp_control_and_indicator() {
        let content = r#"

typedef unsigned char uint8_t;

typedef short int16_t;

typedef unsigned int uint32_t;

typedef uint8_t NiFpga_Bool;

typedef struct NiFpga_FxpTypeInfo
{
    NiFpga_Bool isSigned;
    uint8_t wordLength;
    int16_t integerWordLength;
} NiFpga_FxpTypeInfo;

const NiFpga_FxpTypeInfo NiFpga_Main_IndicatorFxp_FxpResult_TypeInfo = {1,33,17};

const uint32_t NiFpga_Main_IndicatorFxp_FxpResult_Resource = 0x1803C;

const NiFpga_FxpTypeInfo NiFpga_Main_ControlFxp_FxpSum_TypeInfo =
    {
        1,
        32,
        16};

const uint32_t NiFpga_Main_ControlFxp_FxpSum_Resource = 0x18040;
        "#;

        let mut visitor = CustomTypeVisitor::new("Main");
        visit_c_code(content, &mut visitor);

        let (fxp_regs, _) = visitor.get_registers().unwrap();

        let expected = vec![
            FxpRegister {
                name: "FxpResult".to_owned(),
                fxp_type_info: FxpTypeInfo {
                    signed: true,
                    word_length: 33,
                    integer_word_length: 17,
                },
                address: 0x1803C,
            },
            FxpRegister {
                name: "FxpSum".to_owned(),
                fxp_type_info: FxpTypeInfo {
                    signed: true,
                    word_length: 32,
                    integer_word_length: 16,
                },
                address: 0x18040,
            },
        ];
        assert_eq!(fxp_regs, expected);
    }
}
