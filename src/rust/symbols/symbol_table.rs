use symbols::*;
use support::*;

//TODO change String to Symbol
///start, end, name, namespace
pub struct MemberBlock (pub usize, pub usize, pub String, pub String);

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

pub struct SymbolTable
{
    pub children_scopes: Vec<Symbol>,
}

impl<'a> SymbolTable
{
    pub fn new() -> SymbolTable
    {
        SymbolTable
        {
            children_scopes: Vec::new(),
        }
    }
}

impl StaticSymbolTable for SymbolTable
{
    /// Return all symbols in this table with the specified name (in any namespace)
    fn lookup_by_name(&self, name: &str) -> Vec<&Symbol>
    {
        let mut symbols: Vec<&Symbol> = Vec::new();
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
	fn lookup_by_namespace(&self, namespace: &str) -> Vec<&Symbol>
    {
        let mut symbols: Vec<&Symbol> = Vec::new();
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
	fn lookup_variable(&self, namespace: &str, name: &str) -> Option<&Symbol>
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
                                SymbolClass::Variable(ref variable_type) => return Some((symbol).clone()),
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
	fn lookup_function(&self, namespace: &str, name: &str, argument_types: &Vec<String>) -> Option<&Symbol>
    {
        //TODO use argument_types
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
                                SymbolClass::Function(ref return_type, ref arguments, ref static_label, static_length) => return Some((symbol).clone()),
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
	fn lookup_structure(&self, namespace: &str, name: &str) -> Option<(&Symbol)>
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
                                SymbolClass::Structure(ref sub_type) => return Some((symbol).clone()),
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
	fn add(&mut self, class: SymbolClass, namespace: String, name: String, is_static: bool, in_method: bool, is_parameter: bool, local_variable_count: u16, static_variable_count: u16, parameter_offset: u16) -> bool
    {
        let mut static_label = namespace.replace(".", "_").clone();
        static_label.push_str("_static");
        let mut method_namespace = namespace.replace(".", "_").clone();
        method_namespace.push_str("_");
        method_namespace.push_str(&*name);

        //TODO add return false
        let mut location: SymbolLocation = match class
        {
            //TODO replace with storing logic
            SymbolClass::Structure(ref sub_type) => SymbolLocation::Structured,
            SymbolClass::Variable(ref variable_type) => match in_method
                {
                    true => match is_parameter
                        {
                            true => SymbolLocation::MethodArgument(parameter_offset),
                            false =>SymbolLocation::Memory(MemoryAddress {label_name: static_label, offset: static_variable_count * 4}),
                        },
                    false => match is_static
                        {
                            true => SymbolLocation::Memory(MemoryAddress {label_name: static_label, offset: static_variable_count * 4}),
                            false => SymbolLocation::InstancedMemory(local_variable_count * 4),
                        },
                },
            SymbolClass::Function(ref return_type, ref arguments, ref static_label, static_length) => SymbolLocation::Memory(MemoryAddress {label_name: method_namespace, offset: 0}),
        };

        let mut symbol: Symbol =  Symbol
        {
            name: name,
            namespace: namespace,
            is_static: is_static,
            symbol_class: class,
            location: location,
        };

        self.children_scopes.push(symbol);
        return true;
    }
}
