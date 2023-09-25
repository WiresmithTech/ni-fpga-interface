//! Holds common definitions shared by the
//! different C code visitors.

use lang_c::ast::{Constant, Expression, IntegerBase};

/// The type of register value we have found.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum LocationKind {
    Control,
    Indicator,
    ControlArray,
    IndicatorArray,
    ControlArraySize,
    IndicatorArraySize,
}

impl LocationKind {
    /// Returns if the kind is one of the array types.
    pub fn is_array(&self) -> bool {
        match self {
            LocationKind::ControlArray
            | LocationKind::IndicatorArray
            | LocationKind::ControlArraySize
            | LocationKind::IndicatorArraySize => true,
            _ => false,
        }
    }

    /// If it is an array type, it will return the size version of it.
    pub fn with_size(self) -> Self {
        match self {
            LocationKind::ControlArray => LocationKind::ControlArraySize,
            LocationKind::IndicatorArray => LocationKind::IndicatorArraySize,
            _ => self,
        }
    }

    /// Convert the type to the naming prefix used in the C interface.
    pub const fn prefix(&self) -> &str {
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

/// Extract a numeric value from the expression for the enum value.
pub fn value_from_discriminant(discriminant: &Expression) -> u32 {
    match &discriminant {
        Expression::Constant(node) => match &node.node {
            Constant::Integer(value) => {
                let radix = match value.base {
                    IntegerBase::Decimal => 10,
                    IntegerBase::Hexadecimal => 16,
                    IntegerBase::Octal => 8,
                    IntegerBase::Binary => 2,
                };
                u32::from_str_radix(&value.number, radix).unwrap()
            }
            _ => panic!("Unexpected constant type."),
        },
        _ => panic!("Unexpected expression type"),
    }
}
