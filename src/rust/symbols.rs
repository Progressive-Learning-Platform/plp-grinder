use tokens::*;

pub struct SymbolTable<'a>
{
    pub signature: &'a str,
    pub return_type: &'a str,
    pub package_path: &'a str,
    pub static_variables: Vec<(usize, usize)>,
    pub static_methods: Vec<(usize, usize)>,
    pub static_classes: Vec<(usize, usize)>,
    pub non_static_variables: Vec<(usize, usize)>,
    pub non_static_methods: Vec<(usize, usize)>,
    pub non_static_classes: Vec<(usize, usize)>,
}

impl<'a> SymbolTable<'a>
{
    pub fn new() -> SymbolTable<'a>
    {
        SymbolTable
        {
            signature: "",
            return_type: "",
            package_path: "",
            static_variables: Vec::new(),
            static_methods: Vec::new(),
            static_classes: Vec::new(),
            non_static_variables: Vec::new(),
            non_static_methods: Vec::new(),
            non_static_classes: Vec::new()
        }
    }

    /// Gets the starting index for a class or method.
    /// Starting point returned is the same on given.
    pub fn get_body_locations(&self, tokens: &Vec<Token>, start: usize) -> (usize, usize)
    {
        let current_index = start;
        let final_index = self.find_outer_ending_brace(tokens, start);
        (current_index, final_index)
    }

    pub fn get_variable_locations(&self, tokens: &Vec<Token>, start: usize) -> (usize, usize)
    {
        let current_index = start;
        let final_index = self.find_next_semicolon(tokens, start);
        (current_index, final_index)
    }

    fn find_next_semicolon(&self, tokens: &Vec<Token>, start: usize) -> usize
    {
        for (index, token) in tokens[start..].iter().enumerate()
        {
            if token.value == ";" { return index + start; }
        }

        return 0;
    }

    fn find_outer_ending_brace(&self, tokens: &Vec<Token>, start: usize) -> usize
    {
        let mut open_braces = 0;

        for(index, token) in tokens[start..].iter().enumerate()
        {
            if token.value == "{"
            {
                open_braces += 1;
            }
            else if token.value == "}"
            {
                open_braces -= 1;
                if open_braces == 0
                {
                    return index + start;
                }
            }
        }
        return 0;
    }
}
