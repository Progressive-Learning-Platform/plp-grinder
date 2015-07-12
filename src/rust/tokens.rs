use files::dump;

pub struct Token<'a>
{
    pub name: &'a str,
    pub range: (usize, usize),
    pub value: String,
    pub line_number: usize,
}

impl <'a> Token<'a>
{
    #[allow(dead_code)]
    pub fn clone(&self) -> Token<'a>
    {
        Token
        {
            name: self.name.clone(),
            range: self.range.clone(),
            value: self.value.clone(),
            line_number: self.line_number.clone(),
        }
    }
}

pub trait Printable
{
    fn print_to(&self, file_path: &str, console_out: bool);

    fn format_for_print(&self, console_out: bool) -> String;
}

impl <'a> Printable for Vec<Token<'a>>
{
    fn print_to(&self, file_path: &str, console_out: bool)
    {
        let lexed_token_string = self.format_for_print(console_out);
        if file_path.len() > 0 { dump(file_path, lexed_token_string); }
    }

    fn format_for_print(&self, console_out: bool) -> String
    {
        // Create a String of 100 consecutive spaces
        let mut spaces = String::new();
        for _ in (0..100)
        {
            spaces.push_str(" ");
        }

        // Spacing definitions
        let name_column_width = 30;
        let line_number_column_width = 20;

        let mut lexed_token_string: String = String::new();

        for token in self.iter()
        {
            let name_pad_length = name_column_width - token.name.len();
            let line_pad_length = line_number_column_width - token.line_number.to_string().len() - 3;

            // Token name
            lexed_token_string.push_str(token.name);
            if name_pad_length > 0 { lexed_token_string.push_str(&spaces[..name_pad_length]); }
            else { lexed_token_string.push_str(" "); }

            // Line number
            lexed_token_string.push_str("ln:");
            lexed_token_string.push_str(&*token.line_number.to_string());
            if line_pad_length > 0 { lexed_token_string.push_str(&spaces[..line_pad_length]); }
            else { lexed_token_string.push_str(" "); }

            // Token value
            lexed_token_string.push_str(&*token.value);
            lexed_token_string.push_str("\n");

            if console_out { println!("\t{}\t{:?}\t{}", token.name, token.range, token.value); }
        }

        lexed_token_string
    }
}
