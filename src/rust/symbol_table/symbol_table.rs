use symbol_table::*;

pub struct ClassStructure
{
    pub static_variables: Vec<(usize, usize)>,
    pub static_methods: Vec<(usize, usize)>,
    pub static_classes: Vec<(usize, usize)>,

    pub non_static_variables: Vec<(usize, usize)>,
    pub non_static_methods: Vec<(usize, usize)>,
    pub non_static_classes: Vec<(usize, usize)>,
}

pub struct SymbolTable<'a>
{
    pub signature: &'a str,
    pub return_type: &'a str,
    pub package_path: &'a str,
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

/*
impl<'a> StaticSymbolTable<'a> for SymbolTable<'a>
{
    fn lookup_by_name(&self, name: &str) -> Vec<Symbol<'a>>
    {
        Vec::new()
    }

	fn lookup_by_namespace(&self, namespace: &str) -> Vec<Symbol<'a>>
    {
        Vec::new()
    }

	fn lookup_variable(&self, namespace: &str, name: &str) -> Option<Symbol<'a>>
    {

    }

	fn lookup_function(&self, namespace: &str, name: &str, argument_types: &Vec<&str>) -> Option<Symbol<'a>>
    {

    }

	fn lookup_structure(&self, namespace: &str, name: &str) -> Option<(Symbol<'a>)>
    {
        Some(Symbol {"", ""})
    }

	fn add(&self, class: SymbolClass<'a>, namespace: &'a str, name: &'a str) -> bool
    {
        true
    }
}
*/
