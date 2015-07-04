extern crate regex;

mod matching;
mod support;
mod tokens;
mod lexer;
mod files;
mod parser;
mod symbols;
mod symbol_table;

use std::vec::Vec;
use tokens::*;
use lexer::*;
use parser::*;
use symbols::*;
use symbol_table::*;

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
                    min_value = find_outer_ending_brace(tokens, index);
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
                min_value = find_outer_ending_brace(tokens, index);
                static_methods.push((index + 1, min_value));
                min_value -=  index;
            }
            else
            {
                panic!("Unsupported or Unexpected token: {} + {}.", tokens[index + skip_amount].value, tokens[index + skip_amount].name);
            }

        }
        //Non-Static Variable/Method/Class
        else if token.name.starts_with("construct")
        {
            skip_amount = 2;
            if tokens[index + skip_amount].name.starts_with("control")
            {
                println!("------Incoming Non-Static Class Decl!");
                min_value = find_outer_ending_brace(tokens, index);
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
                min_value = find_outer_ending_brace(tokens, index);
                non_static_methods.push((index, min_value));
                min_value -= index;
                //check for control
            }
            else if tokens[index + skip_amount].name.starts_with("operator")
            {
                println!("------Incoming Non-Static Variable Decl!");
                min_value =  find_next_semicolon(tokens, index);
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

fn compile_class(tokens: &Vec<Token>, start_index: usize) -> (usize, String)
{
    let invalid_types = get_invalid_token_types();
    let invalid_values = get_invalid_token_values();

    let mut plp_string: String = String::new();
    let mut current_index: usize = start_index;

    if tokens[current_index].value != "{" { panic!("Expected '{{' received: {}", tokens[current_index].value); }
    else { current_index += 1; }

    for (index, token) in tokens.iter().enumerate()
    {
        // Handle forward skipping
        if index < current_index { continue; }
        else { current_index = index; }

        // TODO: encapsulate into token_rules.validate(token)
        // Panic! if token type is invalid
        if invalid_types.contains(&token.name)
        {
            panic!("Unsupported token type: {}", token.name);
        }
        // Panic! if token value is invalid
        else if invalid_values.contains(&&*token.value)
        {
            panic!("Unsupported token value: {}", token.value);
        }

        if token.value == "class"
        {
            // parse class body
            let (end_index, compiled_class) = compile_class(&tokens, index + 1);
            plp_string.push_str(&*compiled_class);

            current_index = end_index;
        }
        else if token.name == "type" // || token.name == "identifier"
        {
            // look ahead
            // parse variable declaration
            // OR parse method declaration
        }
        else if token.value == "if"
        {
            // parse if body
            // Unsupported for now
        }
        else if token.name == "construct.conditional"
        {
            // parse conditional
            // Unsupported for now
        }
        else
        {
            panic!("Unexpected token: {}\t{}", token.name, token.value);
        }
    }

    (current_index + 1, plp_string)
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

fn compile_arithmetic_statement(tokens: &Vec<Token>, start: usize, temp_registers: (&str, &str), var_registers: (&str, &str), load_registers: (&str, &str)) -> (String, usize)
{
    let mut compiled_code = String::new();
    if tokens[start].value.starts_with("literal")
    {
        compiled_code.push_str("li ");
        compiled_code.push_str(temp_registers.0);
        compiled_code.push_str(", ");
        compiled_code.push_str(&*tokens[start].value);
        compiled_code.push_str("\n");
    }

    let mut index = start;
    while tokens[index].value != ";"
    {
        while tokens[index].name == "control" // ignore parenthesis
        {
            index += 1;
        }
        if !tokens[index].name.starts_with("operator")
        {
            let current_token = &tokens[index];
            panic!("Unexpected token while parsing arithmetic statement: {}: {}", current_token.value, current_token.name);
        }

        let operator = &tokens[index];
        while tokens[index].name == "control" // ignore parenthesis
        {
            index += 1;
        }
        let operand = &tokens[index + 1];
        if operand.name.starts_with("literal")
        {
            // TODO: use immediate operators
            compiled_code.push_str("li ");
            compiled_code.push_str(temp_registers.1);
            compiled_code.push_str(", ");
            compiled_code.push_str(&*operand.value);
            compiled_code.push_str("\n");
        }
        else if operand.name.starts_with("identifier")
        {
            // TODO: lookup memory location from symbols table


            // TODO: parse method calls
        }

        let line = compile_arithmetic_operation(operator, temp_registers, temp_registers.0);
        compiled_code.push_str(&*line);
        index += 2;
    }

    (compiled_code, index + 1)
}

fn compile_arithmetic_operation(operator: &Token, operand_registers: (&str, &str), result_register: &str) -> String
{
    let mut compiled_code = String::new();

    if operator.name != "operator.binary"
    {
        panic!("Unsupported operator: {}: {}", operator.name, operator.value);
    }
    else if operator.value == "+"
    {
        compiled_code.push_str("addu ");
    }
    else if operator.value == "-"
    {
        compiled_code.push_str("subu ");
    }
    else if operator.value == "*"
    {
        compiled_code.push_str("mullo ");
    }
    else
    {
        panic!("Unsupported operator: {}: {}", operator.name, operator.value);
    }

    compiled_code.push_str(result_register);
    compiled_code.push_str(", ");
    compiled_code.push_str(operand_registers.0);
    compiled_code.push_str(", ");
    compiled_code.push_str(operand_registers.1);
    compiled_code.push_str("\n");

    compiled_code
}

fn find_next_semicolon(tokens: &Vec<Token>, start: usize) -> usize
{
    for (index, token) in tokens[start..].iter().enumerate()
    {
        if token.value == ";" { return index + start; }
    }

    return 0;
}

fn find_outer_ending_brace(tokens: &Vec<Token>, start: usize) -> usize
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
