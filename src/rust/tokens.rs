use files::dump;

pub struct Token<'a>
{
    pub name: &'a str,
    pub range: (usize, usize),
    pub value: String,
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
        let mut lexed_token_string: String = String::new();

        for token in self.iter()
        {
            lexed_token_string.push_str(token.name);
            lexed_token_string.push_str("\t");
            lexed_token_string.push_str(&*token.value);
            lexed_token_string.push_str("\n");

            if console_out { println!("\t{}\t{:?}\t{}", token.name, token.range, token.value); }
        }

        lexed_token_string
    }
}
