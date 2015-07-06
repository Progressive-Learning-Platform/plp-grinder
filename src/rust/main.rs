extern crate regex;

mod matching;
mod support;
mod tokens;
mod lexer;
mod files;
mod parser;
mod symbols;
mod symbol_table;
mod plp;
mod compiler;

use std::vec::Vec;
use tokens::*;
use lexer::*;
use symbols::*;
use support::*;
use compiler::*;

fn main()
{
    let source_file = "sampleData/BasicArithmatic.java";
    let lex_output_file = "sampleData/output/stable/BasicArithmatic.java.lexed";
    let preprocessed_output_file = "sampleData/output/stable/BasicArithmatic.java.preprocessed";

    let mut tokens: Vec<Token> = lex_file(source_file);

    println!("\n\nFound Tokens:");
    tokens.print_to(lex_output_file, true);

    remove_meta(&mut tokens);
    println!("\n\nPerged Tokens:");
    tokens.print_to(preprocessed_output_file, true);

    let mut symbols_table = SymbolTable::new();
    parse_class(&tokens, 1, &mut symbols_table);


    if tokens[0].value != "class"
    {
        panic!("Unexpected token: {}: {}", tokens[0].value, tokens[0].name);
    }

    let (last_index, asm_string) = compile_class(&tokens, 1);
}

fn parse_class(tokens: &Vec<Token>, start_index: usize, symbols_table: &mut SymbolTable)
{
    let mut static_variables: Vec<(usize, usize)> = Vec::new();
    let mut static_methods: Vec<(usize, usize)> = Vec::new();
    let mut static_classes: Vec<(usize, usize)> = Vec::new();
    let mut non_static_variables: Vec<(usize, usize)> = Vec::new();
    let mut non_static_methods: Vec<(usize, usize)> = Vec::new();
    let mut non_static_classes: Vec<(usize, usize)> = Vec::new();

    println!("\n<------------ Parse Class --------------->");
    let mut min_value = 0;
    let mut skip_amount = 0;

    for (index, token) in tokens.iter().enumerate()
    {
        if min_value != 0
        {
            min_value -= 1;
            continue;
        }
        //Static Variable/Method/Class
        if token.name == "mod.access"
        {
            skip_amount = 3;
            if tokens[index + skip_amount].name.starts_with("operator")
            {
                println!("------Incoming Static Variable Decl!");

                let (low, high) = symbols_table.get_variable_locations(tokens, index + 1);
                symbols_table.static_variables.push((low, high));

                min_value =  low;
                min_value -= index;
            }
            else if tokens[index + 1].name.starts_with("construct")
            {
                if tokens[index + skip_amount].name.starts_with("control")
                {
                    println!("------Incoming Static Class Decl!");
                    // TODO: verify this index starts past the first open brace
                    min_value = identify_body_bounds(tokens, index, ("{", "}")).unwrap();
                    static_classes.push((index + 2, min_value));
                    min_value -= index;
                }
                else
                {
                    panic!("Unsupported or Unexpected token: {} + {}.", tokens[index + skip_amount].value, tokens[index + skip_amount].name);
                }

            }
            else if tokens[index + skip_amount].name.starts_with("control")
            {
                println!("------Incoming Static Method Decl!");
                // TODO: verify this index starts past the first open brace
                min_value = identify_body_bounds(tokens, index, ("{", "}")).unwrap();
                static_methods.push((index + 1, min_value));
                min_value -=  index;
            }
            else
            {
                panic!("Unsupported or Unexpected token: {}: {}.", tokens[index + skip_amount].value, tokens[index + skip_amount].name);
            }

        }
        //Non-Static Variable/Method/Class
        else if token.name.starts_with("construct")
        {
            skip_amount = 2;
            if tokens[index + skip_amount].name.starts_with("control")
            {
                println!("------Incoming Non-Static Class Decl!");
                let index_after_brace = index + skip_amount + 1;
                min_value = identify_body_bounds(tokens, index_after_brace, ("{", "}")).unwrap();
                non_static_classes.push((index + 1, min_value));
                min_value = 0;
            }
            else
            {
                panic!("Unsupported or Unexpected token: {} + {}.", tokens[index + skip_amount].value, tokens[index + skip_amount].name);
            }
        }
        //Non-Static Variable/Method/Class
        else if token.name == "type"
        {
            skip_amount = 2;

            if tokens[index + skip_amount].name.starts_with("control")
            {
                println!("------Incoming Non-Static Method Decl!");
                // TODO: verify this index starts past the first open brace
                min_value = identify_body_bounds(tokens, index, ("{", "}")).unwrap();
                non_static_methods.push((index, min_value));
                min_value -= index;
                //check for control
            }
            else if tokens[index + skip_amount].name.starts_with("operator")
            {
                println!("------Incoming Non-Static Variable Decl!");
                min_value =  find_next(tokens, index, ";").unwrap();
                non_static_variables.push((index, min_value));
                min_value -= index;
            }
        }
        println!("\tIndex: {} | Token -> {} : {}", index, token.value, token.name );
        //deal with parameters
    }
    println!("\n<---------------- Static Variables --------------->");
    for &(start, end) in symbols_table.static_variables.iter()
    {
        println!("Start/End {}/{}", start, end);
    }

    println!("\n<---------------- Static Classes ----------------->");
    for &(start, end) in static_classes.iter()
    {
        println!("Start/End {}/{}", start, end);
    }

    println!("\n<---------------- Static Methods ----------------->");
    for &(start, end) in static_methods.iter()
    {
        println!("Start/End {}/{}", start, end);
    }

    println!("\n<---------------- Non-Static Variables --------------->");
    for &(start, end) in non_static_variables.iter()
    {
        println!("Start/End {}/{}", start, end);
    }

    println!("\n<---------------- Non-Static Classes ----------------->");
    for &(start, end) in non_static_classes.iter()
    {
        println!("Start/End {}/{}", start, end);
    }

    println!("\n<---------------- Non-Static Methods ----------------->");
    for &(start, end) in non_static_methods.iter()
    {
        println!("Start/End {}/{}", start, end);
    }
    println!("\n");
}

/// Removes all meta tokens from the give Vector
/// Meta tokens include:
/// * comments
/// * imports
/// * permission modifiers (public, private, protected)
///
/// Note that in future versions, imports will not be removed.
fn remove_meta(tokens: &mut Vec<Token>)
{
    // Indecies of tokens vector to be removed
    let mut invalid_indecies: Vec<usize> = Vec::new();

    let mut min_index: usize = 0;
    for (index, token) in tokens.iter().enumerate()
    {
        // Allow forward skipping (by setting min_index)
        if index < min_index
        {
            invalid_indecies.push(index);
        }
        // Remove imports
        else if token.name == "special.import"
        {
            invalid_indecies.push(index);
            min_index = index + 2; // skip the next token (semi-colon)
        }
        // Remove comments
        else if token.name.starts_with("comment")
        {
            invalid_indecies.push(index);
        }
        // Remove all permission modifiers (public, private, protected)
        else if token.name == "mod.permission"
        {
            invalid_indecies.push(index);
        }
    }

    // count := how many indecies have already been removed
    // index := the index in the original tokens vector that should be removed
    for (count, index) in invalid_indecies.iter().enumerate()
    {
        // "index" refers to the index before any others were removed. Therefore, it must be offset by the number of removed tokens
        tokens.remove(index - count);
    }
}
