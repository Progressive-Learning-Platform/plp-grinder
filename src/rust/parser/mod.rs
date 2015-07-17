use tokens::*;
use files::dump;

impl <'a> Printable for Vec<(String, String, String, String)>
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
        for _ in (0..150)
        {
            spaces.push_str(" ");
        }

        // Spacing definitions
        let name_column_width = 30;
        let namespace_width = 40;
        let is_static_width = 20;


        let mut lexed_token_string: String = String::new();

        for symbol_item in self.iter()
        {
            let name_pad_length = name_column_width - symbol_item.0.len();
            let namespace_pad_length = namespace_width - symbol_item.1.len();
            let is_static_pad_length = is_static_width - symbol_item.2.len();

            // Symbol name
            lexed_token_string.push_str(&*symbol_item.0.clone());
            if name_pad_length > 0 { lexed_token_string.push_str(&spaces[..name_pad_length]); }
            else { lexed_token_string.push_str(" "); }

            // Symbol namespace
            lexed_token_string.push_str(&*symbol_item.1.clone());
            if namespace_pad_length > 0 { lexed_token_string.push_str(&spaces[..namespace_pad_length]); }
            else { lexed_token_string.push_str(" "); }

            // Symbol is_static
            lexed_token_string.push_str(&*symbol_item.2.clone());
            if is_static_pad_length > 0 { lexed_token_string.push_str(&spaces[..is_static_pad_length]); }
            else { lexed_token_string.push_str(" "); }

            // Symbol location and class
            lexed_token_string.push_str(&*symbol_item.3.clone());
            lexed_token_string.push_str("\n");

            if console_out { println!("{}", lexed_token_string); }
        }

        lexed_token_string
    }
}
