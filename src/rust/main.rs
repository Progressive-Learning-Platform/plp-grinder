extern crate regex;

mod matching;
mod support;
mod tokens;
mod lexer;
mod files;
mod symbol_table;
mod plp;
mod compiler;

use std::vec::Vec;
use tokens::*;
use symbol_table::symbol_table::*;
use lexer::*;
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

    let mut symbols_table: SymbolTable = SymbolTable::new();
    let class_structure = parse_class(&tokens, 1, &mut symbols_table);


    if tokens[0].value != "class"
    {
        panic!("Unexpected token: {}: {}", tokens[0].value, tokens[0].name);
    }

    let (last_index, asm_string) = compile_class(&tokens, 1);
}

fn parse_class(tokens: &Vec<Token>, start_index: usize, symbols_table: &mut SymbolTable) -> ClassStructure
{
    let mut class_structure: ClassStructure = ClassStructure::new();

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
                //TODO account for final
                println!("------Incoming Static Variable Decl!");

                let low = index + 1;
                let high = find_next(tokens, low, ";").unwrap() + 1;

                class_structure.static_variables.push(MemberBlock (low, high, tokens[index + 2].value.clone()));

                min_value =  low;
                min_value -= index;
            }
            else if tokens[index + skip_amount].name == "identifier"
            {
                //TODO account for final
                println!("------Incoming Static Variable Decl!");

                let low = index + 2;
                let high = find_next(tokens, low, ";").unwrap() + 1;
                class_structure.static_variables.push(MemberBlock (low, high, tokens[index + 3].value.clone()));

                min_value =  low;
                min_value -= index;
            }
            else if tokens[index + 1].name.starts_with("construct")
            {
                if tokens[index + skip_amount].name.starts_with("control")
                {
                    //TODO account for final
                    println!("------Incoming Static Class Decl!");
                    let starting_point = find_next(tokens, index, "{").unwrap() + 1;
                    min_value = identify_body_bounds(tokens, starting_point, ("{", "}")).unwrap() + 1;
                    class_structure.static_classes.push(MemberBlock (starting_point - 1, min_value, tokens[index + 2].value.clone()));
                    min_value -= index + 1;
                }
                else
                {
                    panic!("Unsupported or Unexpected token: {} + {}.", tokens[index + skip_amount].value, tokens[index + skip_amount].name);
                }
            }
            else if tokens[index + skip_amount].name.starts_with("control")
            {
                //TODO account for final
                println!("------Incoming Static Method Decl!");
                let starting_point = find_next(tokens, index, "{").unwrap() + 1;
                min_value = identify_body_bounds(tokens, starting_point, ("{", "}")).unwrap() + 1;
                class_structure.static_methods.push(MemberBlock (starting_point - 1, min_value, tokens[index + 2].value.clone()));
                min_value -=  index + 1;
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
                //TODO account for final
                println!("------Incoming Non-Static Class Decl!");
                let index_after_brace = index + skip_amount + 1;
                min_value = identify_body_bounds(tokens, index_after_brace, ("{", "}")).unwrap() + 1;
                class_structure.non_static_classes.push(MemberBlock (index_after_brace - 1, min_value, tokens[index + 1].value.clone()));
                //TODO parse_class(tokens, index, symbols_table);
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
                //TODO account for final
                println!("------Incoming Non-Static Method Decl!");
                let starting_point = find_next(tokens, index, "{").unwrap() + 1;
                min_value = identify_body_bounds(tokens, starting_point, ("{", "}")).unwrap() + 1;
                class_structure.non_static_methods.push(MemberBlock (starting_point - 1, min_value, tokens[index + 1].value.clone()));
                min_value -= index + 1;
                //check for control
            }
            else if tokens[index + skip_amount].name.starts_with("operator")
            {
                //TODO account for final
                println!("------Incoming Non-Static Variable Decl!");
                min_value =  find_next(tokens, index, ";").unwrap() + 1;
                class_structure.non_static_variables.push(MemberBlock (index, min_value, tokens[index + 1].value.clone()));
                min_value -= index + 1;
            }
        }
        println!("\tIndex: {} | Token -> {} : {}", index, token.value, token.name );
        //deal with parameters
    }
    println!("\n<---------------- Static Variables --------------->");
    for member_block in class_structure.static_variables.iter()
    {
        println!("Start/End {}/{}: {}", member_block.0, member_block.1, member_block.2);
    }

    println!("\n<---------------- Static Classes ----------------->");
    for member_block in class_structure.static_classes.iter()
    {
        println!("Start/End {}/{}: {}", member_block.0, member_block.1, member_block.2);
    }

    println!("\n<---------------- Static Methods ----------------->");
    for member_block in class_structure.static_methods.iter()
    {
        println!("Start/End {}/{}: {}", member_block.0, member_block.1, member_block.2);
    }

    println!("\n<---------------- Non-Static Variables --------------->");
    for member_block in class_structure.non_static_variables.iter()
    {
        println!("Start/End {}/{}: {}", member_block.0, member_block.1, member_block.2);
    }

    println!("\n<---------------- Non-Static Classes ----------------->");
    for member_block in class_structure.non_static_classes.iter()
    {
        println!("Start/End {}/{}: {}", member_block.0, member_block.1, member_block.2);
    }

    println!("\n<---------------- Non-Static Methods ----------------->");
    for member_block in class_structure.non_static_methods.iter()
    {
        println!("Start/End {}/{}: {}", member_block.0, member_block.1, member_block.2);
    }
    println!("\n");

    class_structure
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
