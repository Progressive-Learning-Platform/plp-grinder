use tokens::*;
use symbols::*;
use support::*;
use plp::PLPWriter;
use symbols::commons::*;
use symbols::symbol_table::*;

pub fn get_accessor_namespace(symbol: &Symbol, symbol_table: &StaticSymbolTable) -> Option<String>
{
    match symbol.symbol_class
    {
        SymbolClass::Variable(ref variable_type) => {
            let potential_matches = symbol_table.lookup_by_name(&*variable_type);
            if potential_matches.is_empty() { return None; }
            let type_symbol = potential_matches[0];

            let namespace = concatenate_namespace(&*type_symbol.namespace, &*type_symbol.name);
            return Some(namespace);
        },
        SymbolClass::Structure(_, _) => {
            panic!("get_accessor_namespace: Structure access not currently supported");
        },
        SymbolClass::Function(_, _, _, _) => { panic!("Expected Variable or Structure, found Function"); },
    };
}

/// @return: (static_memory_label, static_init_label, local_init_label)
pub fn get_class_labels(class_symbol: &Symbol) -> (String, String, String)
{
    let mut namespace_label = class_symbol.namespace.clone();
    if !namespace_label.is_empty() { namespace_label.push_str("_"); }
    namespace_label.push_str(&*class_symbol.name.clone());

    let mut static_memory_label = namespace_label.clone();
    static_memory_label.push_str("_static");

    let mut static_init_label = static_memory_label.to_string();
    static_init_label.push_str("_init");

    let mut local_init_label = namespace_label.clone();
    local_init_label.push_str("_local_init");

    (static_memory_label, static_init_label, local_init_label)
}

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
            SymbolClass::Structure(ref subtype, _) => {
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
        SymbolClass::Structure(ref subtype, _) => {
                panic!("Expected Function found {}", subtype);
            }
    }
}

/// @return ([arg1] [, arg2] {, arg3..})
pub fn get_arg_signature_of(method_symbol: &Symbol) -> String
{
    let types: &Vec<String> = match method_symbol.symbol_class
    {
        SymbolClass::Variable(_) => {
                panic!("Expected Function found Variable");
            },
        SymbolClass::Function(_, ref arg_types, _, _) => arg_types,
        SymbolClass::Structure(ref subtype, _) => {
                panic!("Expected Function found {}", subtype);
            }
    };

    let mut arg_signature = "(".to_string();
    // Handle first arg type
    if types.len() > 0
    {
        arg_signature.push_str(&*types[0]);
        for ref arg_type in types[1..].iter()
        {
            arg_signature.push_str(",");
            arg_signature.push_str(&*arg_type);
        }
    }
    arg_signature.push_str(")");

    arg_signature
}
