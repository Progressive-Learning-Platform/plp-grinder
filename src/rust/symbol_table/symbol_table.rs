use symbol_table::*;

pub struct MemberBlock (pub usize, pub usize, pub String);

pub struct ClassStructure
{
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
    pub signature: Symbol<'a>,

    pub local_variables: Vec<Symbol<'a>>,
    pub local_functions: Vec<Symbol<'a>>,
    pub local_structures: Vec<Symbol<'a>>,

    pub static_variables: Vec<Symbol<'a>>,
    pub static_functions: Vec<Symbol<'a>>,
    pub static_structures: Vec<Symbol<'a>>,
}

impl<'a> SymbolTable<'a>
{
    pub fn new() -> SymbolTable<'a>
    {
        SymbolTable
        {
            signature: Symbol {namespace: "", name: "", symbol_class: SymbolClass::Structure(""), location: SymbolLocation::Structured},

            static_variables: Vec::new(),
            static_functions: Vec::new(),
            static_structures: Vec::new(),

            local_variables: Vec::new(),
            local_functions: Vec::new(),
            local_structures: Vec::new(),
        }
    }
}

impl<'a> StaticSymbolTable<'a> for SymbolTable<'a>
{
    fn lookup_by_name(&self, name: &str) -> Vec<Symbol<'a>>
    {
        let temp: Vec<Symbol<'a>> = Vec::new();
        temp
    }

	fn lookup_by_namespace(&self, namespace: &str) -> Vec<Symbol<'a>>
    {
        let temp: Vec<Symbol<'a>> = Vec::new();
        temp
    }

	fn lookup_variable(&self, namespace: &str, name: &str) -> Option<Symbol<'a>>
    {
        for symbol in self.local_variables.iter()
        {

        }
        None
    }

	fn lookup_function(&self, namespace: &str, name: &str, argument_types: &Vec<String>) -> Option<Symbol<'a>>
    {
        None
    }

	fn lookup_structure(&self, namespace: &str, name: &str) -> Option<(Symbol<'a>)>
    {
        None
    }

	fn add(&self, class: SymbolClass<'a>, namespace: &'a str, name: &'a str) -> bool
    {
        false
    }
}
