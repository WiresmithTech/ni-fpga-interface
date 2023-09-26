use super::address_definitions::AddressKind;
use super::address_definitions_visitor::{AddressSet, LocationDefinition};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::str::FromStr;

/// Generates a rust module containing the register definitions.
/// including the required include statements.
///
/// The module is declared public for easy use.
pub fn generate_register_module(registers: &AddressSet) -> impl ToTokens {
    let mut tokens = quote! {};
    for (def, address) in registers {
        match def.kind {
            AddressKind::Control | AddressKind::Indicator => {
                let register = generate_address_definition(def, *address, None);
                tokens.append_all(quote! {
                    #register
                });
            }
            AddressKind::ControlArray | AddressKind::IndicatorArray => {
                let mut size_def = def.clone();
                size_def.kind = def.kind.with_size();
                let array_size = registers.get(&size_def).expect("Array size not found.");
                let register = generate_address_definition(def, *address, Some(*array_size));
                tokens.append_all(quote! {
                    #register
                });
            }
            AddressKind::ControlArraySize | AddressKind::IndicatorArraySize => {
                continue;
            }
            AddressKind::HostToTargetFifo | AddressKind::TargetToHostFifo => {
                continue;
            }
        }
    }

    // Seeing as we can't control the input naming conventions we allow non-upper-case.
    // probably we could assume Camel Case and convert but I bet that isn't very consistent.
    quote! {
            #[allow(non_upper_case_globals)]
            #[allow(dead_code)]
            pub mod registers {
            use ni_fpga_interface::registers::{ ArrayRegister, Register};

            #tokens
        }
    }
}

/// Generate a seperate module with the FIFO definitions.
pub fn generate_fifo_module(addresses: &AddressSet) -> impl ToTokens {
    let mut tokens = quote! {};
    for (def, address) in addresses {
        match def.kind {
            AddressKind::HostToTargetFifo | AddressKind::TargetToHostFifo => {
                let register = generate_address_definition(def, *address, None);
                tokens.append_all(quote! {
                    #register
                });
            }
            _ => {
                continue;
            }
        }
    }

    // Seeing as we can't control the input naming conventions we allow non-upper-case.
    // probably we could assume Camel Case and convert but I bet that isn't very consistent.
    quote! {
            #[allow(non_upper_case_globals)]
            #[allow(dead_code)]
            pub mod fifos {
            use ni_fpga_interface::fifos::{ ReadFifo, WriteFifo };
            #tokens
        }
    }
}

fn type_string_to_type(type_string: &str) -> impl ToTokens {
    match type_string {
        "U8" => quote! {u8},
        "U16" => quote! {u16},
        "U32" => quote! {u32},
        "U64" => quote! {u64},
        "I8" => quote! {i8},
        "I16" => quote! {i16},
        "I32" => quote! {i32},
        "I64" => quote! {i64},
        "Sgl" => quote! {f32},
        "Dbl" => quote! {f64},
        _ => panic!("Unknown type {}", type_string),
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
fn generate_address_definition(
    definition: &LocationDefinition,
    address: u32,
    array_size: Option<u32>,
) -> impl ToTokens {
    let name = format_ident!("{}", definition.name);
    let ty = type_string_to_type(&definition.datatype);
    let address = TokenStream::from_str(&format!("0x{:X}", address)).unwrap();

    match definition.kind {
        AddressKind::Control | AddressKind::Indicator => {
            quote! {
                pub const #name: Register<#ty> = Register::new(#address);
            }
        }
        AddressKind::ControlArray | AddressKind::IndicatorArray => {
            let array_size = array_size.expect("Need size to generate an array register.");
            let array_size = TokenStream::from_str(&format!("{array_size}")).unwrap();
            quote! {
                pub const #name: ArrayRegister<#ty, #array_size> = ArrayRegister::new(#address);
            }
        }
        AddressKind::ControlArraySize | AddressKind::IndicatorArraySize => {
            quote! {}
        }
        AddressKind::HostToTargetFifo => {
            quote! {
                pub const #name: WriteFifo<#ty> = WriteFifo::new(#address);
            }
        }
        AddressKind::TargetToHostFifo => {
            quote! {
                pub const #name: ReadFifo<#ty> = ReadFifo::new(#address);
            }
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
            kind: AddressKind::Control,
        };

        let address = 0x1800A;

        let tokens = generate_address_definition(&definition, address, None);

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
            kind: AddressKind::Indicator,
        };

        let address = 0x1802A;

        let tokens = generate_address_definition(&definition, address, None);

        let expected = quote! {
            pub const indicator: Register<i64> = Register::new(0x1802A);
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn test_generate_control_register_f32() {
        let definition = LocationDefinition {
            name: "control".to_string(),
            datatype: "Sgl".to_string(),
            kind: AddressKind::Control,
        };

        let address = 0x1800A;

        let tokens = generate_address_definition(&definition, address, None);

        let expected = quote! {
            pub const control: Register<f32> = Register::new(0x1800A);
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn test_generate_control_register_f64() {
        let definition = LocationDefinition {
            name: "control".to_string(),
            datatype: "Dbl".to_string(),
            kind: AddressKind::Control,
        };

        let address = 0x1800A;

        let tokens = generate_address_definition(&definition, address, None);

        let expected = quote! {
            pub const control: Register<f64> = Register::new(0x1800A);
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn test_generate_control_array_register() {
        let definition = LocationDefinition {
            name: "control".to_string(),
            datatype: "U8".to_string(),
            kind: AddressKind::ControlArray,
        };

        let address = 0x1800A;

        let tokens = generate_address_definition(&definition, address, Some(5));

        let expected = quote! {
            pub const control: ArrayRegister<u8, 5> = ArrayRegister::new(0x1800A);
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn test_generate_array_indicator_register() {
        let definition = LocationDefinition {
            name: "indicator".to_string(),
            datatype: "I64".to_string(),
            kind: AddressKind::IndicatorArray,
        };

        let address = 0x1802A;

        let tokens = generate_address_definition(&definition, address, Some(3));

        let expected = quote! {
            pub const indicator: ArrayRegister<i64, 3> = ArrayRegister::new(0x1802A);
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }

    /// Should do nothing as this is handled at a higher level.
    #[test]
    fn test_generate_control_array_size() {
        let definition = LocationDefinition {
            name: "control".to_string(),
            datatype: "U8".to_string(),
            kind: AddressKind::ControlArraySize,
        };

        let address = 0x1800A;

        let tokens = generate_address_definition(&definition, address, Some(5));

        assert!(tokens.to_token_stream().is_empty());
    }

    #[test]
    fn test_generate_array_indicator_size() {
        let definition = LocationDefinition {
            name: "indicator".to_string(),
            datatype: "I64".to_string(),
            kind: AddressKind::IndicatorArraySize,
        };

        let address = 0x1802A;

        let tokens = generate_address_definition(&definition, address, Some(3));

        assert!(tokens.to_token_stream().is_empty());
    }

    #[test]
    fn test_should_generate_a_public_module_with_registers() {
        let mut registers = AddressSet::new();
        registers.insert(
            LocationDefinition {
                name: "control".to_string(),
                datatype: "U8".to_string(),
                kind: AddressKind::Control,
            },
            0x1800A,
        );

        let tokens = generate_register_module(&registers);

        let expected = quote! {
            #[allow(non_upper_case_globals)]
            #[allow(dead_code)]
            pub mod registers {
                use ni_fpga_interface::registers::{ ArrayRegister, Register};

                pub const control: Register<u8> = Register::new(0x1800A);
            }
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn test_should_not_include_fifos_in_register_module() {
        let mut registers = AddressSet::new();
        registers.insert(
            LocationDefinition {
                name: "control".to_string(),
                datatype: "U8".to_string(),
                kind: AddressKind::Control,
            },
            0x1800A,
        );
        registers.insert(
            LocationDefinition {
                name: "to_fpga".to_string(),
                datatype: "U8".to_string(),
                kind: AddressKind::HostToTargetFifo,
            },
            0x01,
        );

        let tokens = generate_register_module(&registers);

        let expected = quote! {
            #[allow(non_upper_case_globals)]
            #[allow(dead_code)]
            pub mod registers {
                use ni_fpga_interface::registers::{ ArrayRegister, Register};

                pub const control: Register<u8> = Register::new(0x1800A);
            }
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn test_should_extract_size_from_set_for_array_register() {
        let mut registers = AddressSet::new();
        registers.insert(
            LocationDefinition {
                name: "control".to_string(),
                datatype: "U8".to_string(),
                kind: AddressKind::ControlArray,
            },
            0x1800A,
        );
        registers.insert(
            LocationDefinition {
                name: "control".to_string(),
                datatype: "U8".to_string(),
                kind: AddressKind::ControlArraySize,
            },
            5,
        );

        let tokens = generate_register_module(&registers);

        let expected = quote! {
            #[allow(non_upper_case_globals)]
            #[allow(dead_code)]
            pub mod registers {
                use ni_fpga_interface::registers::{ ArrayRegister, Register};

                pub const control: ArrayRegister<u8, 5> = ArrayRegister::new(0x1800A);
            }
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }

    #[test]
    fn test_should_generate_a_public_module_with_fifos() {
        let mut registers = AddressSet::new();
        registers.insert(
            LocationDefinition {
                name: "to_fpga".to_string(),
                datatype: "U8".to_string(),
                kind: AddressKind::HostToTargetFifo,
            },
            0x01,
        );
        registers.insert(
            LocationDefinition {
                name: "from_fpga".to_string(),
                datatype: "Sgl".to_string(),
                kind: AddressKind::TargetToHostFifo,
            },
            0x02,
        );
        registers.insert(
            LocationDefinition {
                name: "control".to_string(),
                datatype: "U8".to_string(),
                kind: AddressKind::Control,
            },
            0x1800A,
        );

        let tokens = generate_fifo_module(&registers);

        let expected = quote! {
            #[allow(non_upper_case_globals)]
            #[allow(dead_code)]
            pub mod fifos {
                use ni_fpga_interface::fifos::{ ReadFifo, WriteFifo };

                pub const to_fpga: WriteFifo<u8> = WriteFifo::new(0x1);
                pub const from_fpga: ReadFifo<f32> = ReadFifo::new(0x2);
            }
        };

        assert_eq!(tokens.to_token_stream().to_string(), expected.to_string());
    }
}
