use symbol_table::*;
use std::collections::HashMap;

//TODO change String to Symbol
pub struct MemberBlock (pub usize, pub usize, pub String);

pub struct ClassStructure
{
    //TODO add constructor vector
    pub static_variables: Vec<MemberBlock>,
    pub static_methods: Vec<MemberBlock>,
    pub static_classes: Vec<MemberBlock>,

    pub non_static_variables: Vec<MemberBlock>,
    pub non_static_methods: Vec<MemberBlock>,
    pub non_static_classes: Vec<MemberBlock>,
}

impl ClassStructure
{
    pub fn new() -> ClassStructure
    {
        ClassStructure
        {
            static_variables: Vec::new(),
            static_methods: Vec::new(),
            static_classes: Vec::new(),

            non_static_variables: Vec::new(),
            non_static_methods: Vec::new(),
            non_static_classes: Vec::new(),
        }
    }
}

pub struct SymbolTable<'a>
{
    children_scopes: Vec<Symbol<'a>>,
}

impl<'a> SymbolTable<'a>
{
    pub fn new() -> SymbolTable<'a>
    {
        SymbolTable
        {
            children_scopes: Vec::new(),
        }
    }
}

impl<'a> StaticSymbolTable<'a> for SymbolTable<'a>
{
    /// Return all symbols in this table with the specified name (in any namespace)
    fn lookup_by_name(&self, name: &str) -> Vec<&Symbol<'a>>
    {
        let mut symbols: Vec<&Symbol<'a>> = Vec::new();
        for symbol in self.children_scopes.iter()
        {
            if symbol.name == name
            {
                symbols.push(symbol.clone());
            }
        }
        symbols
    }

    /// Return all symbols in this table with the specified namespace
	fn lookup_by_namespace(&self, namespace: &str) -> Vec<&Symbol<'a>>
    {
        let mut symbols: Vec<&Symbol<'a>> = Vec::new();
        for symbol in self.children_scopes.iter()
        {
            if symbol.namespace == namespace
            {
                symbols.push(symbol.clone());
            }
        }
        symbols
    }

    /// Lookup a variable by its name and namespace. Duplicate symbols are not allowed, so the result will be unique
    /// @return the specified symbol or None if the specified symbol is not in this namespace
	fn lookup_variable(&self, namespace: &str, name: &str) -> Option<&Symbol<'a>>
    {
        let mut namespaces: Vec<&str> = namespace.split_terminator('.').collect();
        let mut length;
        let mut current_namespace;

        loop
        {
            current_namespace = namespaces.connect(".");

            if namespaces.is_empty()
            {
                break;
            }
            else
            {
                for symbol in self.lookup_by_namespace(&*current_namespace).iter()
                {
                    if symbol.namespace == current_namespace
                    {
                        if symbol.name == name
                        {
                            match symbol.symbol_class
                            {
                                SymbolClass::Variable(variable_type) => return Some((symbol).clone()),
                                _ => continue,
                            };
                        }
                    }
                }
                length = namespaces.len();
                namespaces.remove(length - 1);
            }
        }
        None
    }

    /// Lookup a function by its name and namespace. Functions with the same signature are not allowed, so the result will be unique
    /// If no result is found in the direct namespace, the parent namespaces will be searched
    /// @return the specified symbol or None if the specified symbol is not in this namespace or a parent namespace
	fn lookup_function(&self, namespace: &str, name: &str, argument_types: &Vec<String>) -> Option<&Symbol<'a>>
    {
        let mut namespaces: Vec<&str> = namespace.split_terminator('.').collect();
        let mut length;
        let mut current_namespace;

        loop
        {
            current_namespace = namespaces.connect(".");

            if namespaces.is_empty()
            {
                break;
            }
            else
            {
                for symbol in self.lookup_by_namespace(&*current_namespace).iter()
                {
                    if symbol.namespace == current_namespace
                    {
                        if symbol.name == name
                        {
                            match symbol.symbol_class
                            {
                                SymbolClass::Function(return_type, arguments) => return Some((symbol).clone()),
                                _ => continue,
                            };
                        }
                    }
                }
                length = namespaces.len();
                namespaces.remove(length - 1);
            }
        }
        None
    }

    /// Lookup a structure (class, enum) by its name and namespace.
    /// Duplicate classes in the same namespace are not allowed, so the result will be unique
    /// If no result is found in the direct namespace, the parent namespaces will be searched
    /// @return the specified symbol or None if the specified symbol is not in this namespace or a parent namespace
	fn lookup_structure(&self, namespace: &str, name: &str) -> Option<(&Symbol<'a>)>
    {
        let mut namespaces: Vec<&str> = namespace.split_terminator('.').collect();
        let mut length;
        let mut current_namespace;

        loop
        {
            current_namespace = namespaces.connect(".");

            if namespaces.is_empty()
            {
                break;
            }
            else
            {
                for symbol in self.lookup_by_namespace(&*current_namespace).iter()
                {
                    if symbol.namespace == current_namespace
                    {
                        if symbol.name == name
                        {
                            match symbol.symbol_class
                            {
                                SymbolClass::Structure(sub_type) => return Some((symbol).clone()),
                                _ => continue,
                            };
                        }
                    }
                }
                length = namespaces.len();
                namespaces.remove(length - 1);
            }
        }
        None
    }

    /// Adds a symbol to this table and allocates it's location
    /// Returns true if the symbol could be added; false otherwise
    /// Duplicate symbols are not allowed
    /// TODO: support overloaded methods
	fn add(&mut self, class: SymbolClass<'a>, namespace: &'a str, name: &'a str) -> bool
    {
        //TODO add return false
        let mut location: SymbolLocation = match class
        {
            //TODO replace with storing logic
            SymbolClass::Structure(sub_type) => SymbolLocation::Structured,
            SymbolClass::Variable(variable_type) => SymbolLocation::Structured,
            SymbolClass::Function(return_type, arguments) => SymbolLocation::Structured,
        };

        let mut symbol: Symbol =  Symbol
        {
            name: name,
            namespace: namespace,
            is_static: false,
            symbol_class: class,
            location: location,
        };

        self.children_scopes.push(symbol);
        return true;
    }
}
