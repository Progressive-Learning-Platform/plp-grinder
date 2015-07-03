extern crate regex;

mod matching;
mod support;
mod tokens;
mod lexer;
mod files;
mod parser;

use std::vec::Vec;
use tokens::*;
use lexer::*;
use parser::*;

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

    if tokens[0].value != "class"
    {
        panic!("Unexpected token: {}: {}", tokens[0].value, tokens[0].name);
    }

    let (last_index, asm_string) = compile_class(&tokens, 1);
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
        // Remove all modifiers (public, private, volatile, transient, static, etc.)
        else if token.name.starts_with("mod")
        {
            if token.name.ends_with("access")
            {
                continue;
            }
            invalid_indecies.push(index);
        }
        // Remove package declarations
        else if token.name == "special.package"
        {
            if token.name.ends_with("package")
            {
                continue;
            }
            invalid_indecies.push(index);
            min_index = find_next_semicolon(tokens, index) + 1;
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
        }

        let line = compile_arithmetic_operation(operator, temp_registers, temp_registers.0);
        compiled_code.push_str(&*line);
        index += 2;
    }

    (compiled_code, index + 1)
}

fn compile_arithmetic_operation(operator: &Token, operands: (&str, &str), result_register: &str) -> String
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
    compiled_code.push_str(operands.0);
    compiled_code.push_str(", ");
    compiled_code.push_str(operands.1);
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
