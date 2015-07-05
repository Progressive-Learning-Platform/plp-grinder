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

use std::vec::Vec;
use tokens::*;
use lexer::*;
use parser::*;
use symbols::*;
use symbol_table::*;
use support::*;
use plp::PLPWriter;

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
                // TODO: verify this index starts past the first open brace
                min_value = identify_body_bounds(tokens, index, ("{", "}")).unwrap();
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

/// Write PLP code to evaluate the given symbol sequence, and load the result into a specific register
/// A sequence can be:
/// * a  simple variable reference,
/// * a method call,
/// * a variable accessed from another symbol (e.g. foo.bar or Foo.staticBar),
/// * a method accessed from another symbol (e.g. foo.bar() or Foo.staticBar()), or
/// * a complex chain of the above (e.g. foo.method().valueInReturnValue.value.method())
///
/// The start index should be the first symbol in the sequence
/// @return (plp_code, the first index AFTER this symbol sequence)
fn compile_symbol_sequence( tokens: &Vec<Token>,
                            start: usize,
                            current_namespace: &str,
                            temp_registers: (&str, &str),
                            var_registers: (&str, &str),
                            load_registers: (&str, &str),
                            target_register: &str,
                            symbols: &StaticSymbolTable)
                            -> (String, usize)
{
    // TODO: handle array access
    // For method calls: Always push $this to stack if possible, and pop $this from stack when done (if it was pushed)
    // Wehn accessing, push instance to stack

    let mut plp = PLPWriter::new();
    let mut index = start;
    loop
    {
        let token = &tokens[index];

        if index >= tokens.len()
        {
            // End of stream
            break;
        }
        if token.name == "identifier"
        {
            let lookahead_token = &tokens[index + 1];

            // Method call
            if lookahead_token.value == "("
            {
                // push $this or reference to calling object as an argument
                plp.push(target_register);

                // compile the method and append it directly to the compiled plp code
                let method_code = compile_method_call(tokens, index, current_namespace, temp_registers, var_registers, load_registers, symbols);
                plp.code.push_str(&*method_code);
            }
            // Variable read
            else
            {
                let symbol = symbols.lookup_variable(current_namespace, &*token.value).unwrap();
                match symbol.location
                {
                    SymbolLocation::Register(name) => {
                            plp.mov(target_register, name);
                        },
                    SymbolLocation::Memory(address) => {
                            plp.li(load_registers.0, address.label_name);
                            plp.lw(target_register, address.offset, load_registers.0);
                        },
                    SymbolLocation::InstancedMemory(offset) => {
                            plp.lw(target_register, offset, target_register);
                        },
                    SymbolLocation::Structured => {
                            // TODO: append to namespace
                        },
                };
            }
        }
        else if token.value == "."
        {
            // Access references are handled when it's children are parsed (in the if block above)
            // so skip this token
            index += 1;
            continue;
        }
        else
        {
            break;
        }

        index += 1;
    }
    // first index AFTER the sequence
    index += 1;

    // TODO: compile

    (plp.code, index)
}

/// The range should start at the method identifier and end on the token AFTER the closing parenthesis
fn compile_method_call( tokens: &Vec<Token>,
                        start: usize,
                        current_namespace: &str,
                        temp_registers: (&str, &str),
                        var_registers: (&str, &str),
                        load_registers: (&str, &str),
                        symbols: &StaticSymbolTable)
                        -> String
{
    let mut plp = PLPWriter::new();

    // Find nested method calls
    // Handle each argument one at a time, and push each to the stack

    // TODO: determine namespace from caller and current_namespace
    let namespace = "";

    let method_id = &tokens[start];
    // TODO: determine if method is static

    //let method_symbol = symbols.lookup_function();

    return plp.code;
}

fn compile_arithmetic_statement(tokens: &Vec<Token>,
                                start: usize,
                                current_namespace: &str,
                                temp_registers: (&str, &str),
                                var_registers: (&str, &str),
                                load_registers: (&str, &str),
                                target_register: &str,
                                symbols: &StaticSymbolTable)
                                -> (String, usize)
{
    // TODO: handle parenthesis and operder of operations
    let mut plp = PLPWriter::new();
    let mut index = start;

    // vector to ignore parenthesis within the scope of this sequence (e.g. do not ignore parenthesis that belong to a method call)
    let mut ignore_indices: Vec<usize> = Vec::new();

    // Handle first token
    let mut token = &tokens[start];
    if token.name.starts_with("literal")
    {
        let value = &*token.value;
        plp.li(temp_registers.0, value);

        index += 1;
    }
    else if token.name == "identifier"
    {
        // handle identifier
        let (access_code, new_index) = compile_symbol_sequence(tokens, start, current_namespace, temp_registers, var_registers, load_registers, target_register, symbols);
        plp.code.push_str(&*access_code);

        index = new_index;
    }
    else if token.value == "(" || token.value == ")"
    {
        // TODO: determine bounds and push to ignore
        panic!("Unexpected token: {}\t{}", token.name, token.value);
    }
    else
    {
        panic!("Unexpected token: {}\t{}", token.name, token.value);
    }
    token = &tokens[index];

    // loop until arithmetic sequence ends
    while token.name.starts_with("operator")
    {
        token = &tokens[index];
        if token.name.starts_with("literal")
        {
            // Load second operand into temp_registers.0 as a literal
            let value = &token.value;
            plp.li(temp_registers.0, &*value);

            // Consume the operator AND second operand tokens
            index += 2;
        }
        else if token.name == "identifier"
        {
            // Load second operand into temp_registers.0 as a variable reference
            // TODO: don't give away temp_registers.0
            let temp_target = temp_registers.0;
            let (access_code, new_index) = compile_symbol_sequence(tokens, start, current_namespace, temp_registers, var_registers, load_registers, temp_target, symbols);
            plp.code.push_str(&*access_code);

            index = new_index;
        }
        else if token.value == "(" || token.value == ")"
        {
            // TODO: determine bounds and push to ignore
            panic!("Unexpected token: {}\t{}", token.name, token.value);
        }
        else
        {
            panic!("Unexpected token: {}\t{}", token.name, token.value);
        }

        // Perform the operation on the current result (target_register) and the second operand (temp_registers.0)
        compile_arithmetic_operation(&token, (target_register, temp_registers.0), target_register);
    }

    (plp.code, index)
}

fn compile_arithmetic_operation(operator: &Token, operand_registers: (&str, &str), result_register: &str) -> String
{
    let mut plp = PLPWriter::new();

    match &*operator.value
    {
        "+" => plp.addu(result_register, operand_registers.0, operand_registers.1),
        "-" => plp.subu(result_register, operand_registers.0, operand_registers.1),
        "*" => plp.mullo(result_register, operand_registers.0, operand_registers.1),
         _  => panic!("Unsupported operator: {}: {}", operator.name, operator.value)
    };

    return plp.code;
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
