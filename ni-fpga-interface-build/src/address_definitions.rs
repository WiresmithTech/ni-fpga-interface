//! Holds common definitions shared by the
//! different C code visitors.

use lang_c::ast::{Constant, Expression, IntegerBase};

/// The type of register value we have found.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AddressKind {
    Control,
    Indicator,
    ControlArray,
    IndicatorArray,
    ControlArraySize,
    IndicatorArraySize,
    TargetToHostFifo,
    HostToTargetFifo,
}

impl AddressKind {
    /// Returns if the kind is one of the array types.
    pub fn is_array(&self) -> bool {
        match self {
            AddressKind::ControlArray
            | AddressKind::IndicatorArray
            | AddressKind::ControlArraySize
            | AddressKind::IndicatorArraySize => true,
            _ => false,
        }
    }

    /// If it is an array type, it will return the size version of it.
    pub fn with_size(self) -> Self {
        match self {
            AddressKind::ControlArray => AddressKind::ControlArraySize,
            AddressKind::IndicatorArray => AddressKind::IndicatorArraySize,
            _ => self,
        }
    }

    /// Convert the type to the naming prefix used in the C interface.
    pub const fn prefix(&self) -> &str {
        match self {
            AddressKind::Control => "Control",
            AddressKind::Indicator => "Indicator",
            AddressKind::ControlArray => "ControlArray",
            AddressKind::IndicatorArray => "IndicatorArray",
            AddressKind::ControlArraySize => "ControlArray",
            AddressKind::IndicatorArraySize => "IndicatorArray",
            AddressKind::TargetToHostFifo => "TargetToHostFifo",
            AddressKind::HostToTargetFifo => "HostToTargetFifo",
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
