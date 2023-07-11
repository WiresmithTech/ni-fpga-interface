use super::register_definitions_visitor::{LocationDefinition, LocationKind, RegisterSet};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::str::FromStr;

/// Generates a rust module containing the register definitions.
/// including the required include statements.
///
/// The module is declared public for easy use.
pub fn generate_register_module(registers: &RegisterSet) -> impl ToTokens {
    let mut tokens = quote! {};
    for (def, address) in registers {
        match def.kind {
            LocationKind::Control | LocationKind::Indicator => {
                let register = generate_register_definition(def, *address, None);
                tokens.append_all(quote! {
                    #register
                });
            }
            LocationKind::ControlArray | LocationKind::IndicatorArray => {
                let mut size_def = def.clone();
                size_def.kind = def.kind.with_size();
                let array_size = registers.get(&size_def).expect("Array size not found.");
                let register = generate_register_definition(def, *address, Some(*array_size));
                tokens.append_all(quote! {
                    #register
                });
            }
            LocationKind::ControlArraySize | LocationKind::IndicatorArraySize => {
                continue;
            }
        }
    }
    quote! {
            pub mod registers {
            use ni_fpga_interface::registers::{ ArrayRegister, Register};
            #tokens
        }
    }
}

/// Generates the definition for a single register.
/// This is a const definition of a Register<T> or ArrayRegister<T, N> depending on the kind of register.
///
/// # Arguments
/// array_size: The size of the array if the register is an array. None if note.
///
/// If you pass an array size type then nothing is generated as this should be folded into the array register
/// at a higher level.
fn generate_register_definition(
    definition: &LocationDefinition,
    address: u32,
    array_size: Option<u32>,
) -> impl ToTokens {
    let name = format_ident!("{}", definition.name);
    let ty = format_ident!("{}", definition.datatype.to_ascii_lowercase());
    let address = TokenStream::from_str(&format!("0x{:X}", address)).unwrap();

    match definition.kind {
        LocationKind::Control | LocationKind::Indicator => {
            quote! {
                pub const #name: Register<#ty> = Register::new(#address);
            }
        }
        LocationKind::ControlArray | LocationKind::IndicatorArray => {
            let array_size = array_size.expect("Need size to generate an array register.");
            let array_size = TokenStream::from_str(&format!("{array_size}")).unwrap();
            quote! {
                pub const #name: ArrayRegister<#ty, #array_size> = Register::new(#address);
            }
        }
        LocationKind::ControlArraySize | LocationKind::IndicatorArraySize => {
            quote! {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_control_register() {
        let definition = LocationDefinition {
            name: "control".to_string(),
            datatype: "U8".to_string(),
            kind: LocationKind::Control,
        };

        let address = 0x1800A;

        let tokens = generate_register_definition(&definition, address, None);

        let expected = quote! {
            pub const control: Register<u8> = Register::new(0x1800A);
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn test_generate_indicator_register() {
        let definition = LocationDefinition {
            name: "indicator".to_string(),
            datatype: "I64".to_string(),
            kind: LocationKind::Indicator,
        };

        let address = 0x1802A;

        let tokens = generate_register_definition(&definition, address, None);

        let expected = quote! {
            pub const indicator: Register<i64> = Register::new(0x1802A);
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn test_generate_control_array_register() {
        let definition = LocationDefinition {
            name: "control".to_string(),
            datatype: "U8".to_string(),
            kind: LocationKind::ControlArray,
        };

        let address = 0x1800A;

        let tokens = generate_register_definition(&definition, address, Some(5));

        let expected = quote! {
            pub const control: ArrayRegister<u8, 5> = Register::new(0x1800A);
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn test_generate_array_indicator_register() {
        let definition = LocationDefinition {
            name: "indicator".to_string(),
            datatype: "I64".to_string(),
            kind: LocationKind::IndicatorArray,
        };

        let address = 0x1802A;

        let tokens = generate_register_definition(&definition, address, Some(3));

        let expected = quote! {
            pub const indicator: ArrayRegister<i64, 3> = Register::new(0x1802A);
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }

    /// Should do nothing as this is handled at a higher level.
    #[test]
    fn test_generate_control_array_size() {
        let definition = LocationDefinition {
            name: "control".to_string(),
            datatype: "U8".to_string(),
            kind: LocationKind::ControlArraySize,
        };

        let address = 0x1800A;

        let tokens = generate_register_definition(&definition, address, Some(5));

        let expected = quote! {
            pub const CONTROL: ArrayRegister<u8; 5> = Register::new(0x1800A);
        };

        assert!(tokens.to_token_stream().is_empty());
    }

    #[test]
    fn test_generate_array_indicator_size() {
        let definition = LocationDefinition {
            name: "indicator".to_string(),
            datatype: "I64".to_string(),
            kind: LocationKind::IndicatorArraySize,
        };

        let address = 0x1802A;

        let tokens = generate_register_definition(&definition, address, Some(3));

        assert!(tokens.to_token_stream().is_empty());
    }

    #[test]
    fn test_should_generate_a_public_module_with_registers() {
        let mut registers = RegisterSet::new();
        registers.insert(
            LocationDefinition {
                name: "control".to_string(),
                datatype: "U8".to_string(),
                kind: LocationKind::Control,
            },
            0x1800A,
        );

        let tokens = generate_register_module(&registers);

        let expected = quote! {
            pub mod registers {
                use ni_fpga_interface::registers::{ ArrayRegister, Register};

                pub const control: Register<u8> = Register::new(0x1800A);
            }
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn test_should_extract_size_from_set_for_array_register() {
        let mut registers = RegisterSet::new();
        registers.insert(
            LocationDefinition {
                name: "control".to_string(),
                datatype: "U8".to_string(),
                kind: LocationKind::ControlArray,
            },
            0x1800A,
        );
        registers.insert(
            LocationDefinition {
                name: "control".to_string(),
                datatype: "U8".to_string(),
                kind: LocationKind::ControlArraySize,
            },
            5,
        );

        let tokens = generate_register_module(&registers);

        let expected = quote! {
            pub mod registers {
                use ni_fpga_interface::registers::{ ArrayRegister, Register};

                pub const control: ArrayRegister<u8, 5> = Register::new(0x1800A);
            }
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }
}
