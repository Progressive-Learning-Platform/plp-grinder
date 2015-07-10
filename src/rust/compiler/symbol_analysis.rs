use tokens::*;
use symbols::*;
use symbols::symbol_table::*;
use support::*;
use plp::PLPWriter;

/// @return: (method_label, return_label)
pub fn get_method_labels(method_symbol: &Symbol) -> (String, String)
{
    let method_label = match method_symbol.location {
        SymbolLocation::Memory(ref address) => address.label_name.clone(),
        _ => { panic!("compile_method_body: Expected Memory address for method"); },
    };

    let mut return_label = method_label.clone();
    return_label.push_str("_return");

    (method_label, return_label)
}

/// @return (memory_label, memory_size)
pub fn get_static_allocation(method_symbol: &Symbol) -> (String, u16)
{
    match method_symbol.symbol_class {
            SymbolClass::Variable(_) => {
                    panic!("Expected Function found Variable");
                },
            SymbolClass::Function(_, _, ref label_name, var_count) => (label_name.clone(), var_count as u16),
            SymbolClass::Structure(ref subtype) => {
                    panic!("Expected Function found {}", subtype);
                }
        }
}

/// @return (memory_label, memory_size)
pub fn get_return_type_of(method_symbol: &Symbol) -> String
{
    match method_symbol.symbol_class
    {
        SymbolClass::Variable(_) => {
                panic!("Expected Function found Variable");
            },
        SymbolClass::Function(ref return_type, _, _, _) => return_type.clone(),
        SymbolClass::Structure(ref subtype) => {
                panic!("Expected Function found {}", subtype);
            }
    }
}
