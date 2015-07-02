extern crate regex;

mod matching;
mod support;
mod tokens;
mod lexer;
mod files;

use std::vec::Vec;
use tokens::*;
use lexer::*;

fn main()
{
    let source_file = "sampleData/BasicArithmatic.java";
    let lex_output_file = "sampleData/output/stable/BasicArithmatic.java.lexed";

    let mut tokens: Vec<Token> = lex_file(source_file);

    println!("\n\nFound Tokens:");
    tokens.print_to(lex_output_file, true);
}
