use std::vec::Vec;
use tokens::*;
use lexer::*;
use parser::*;
use symbol_table::*;
use support::*;
use plp::PLPWriter;

pub fn compile_class(tokens: &Vec<Token>, start_index: usize) -> (usize, String)
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
pub fn compile_symbol_sequence( tokens: &Vec<Token>,
                            start: usize,
                            current_namespace: &str,
                            temp_register: &str, // indirect
                            load_registers: (&str, &str),
                            target_register: &str,
                            symbols: &StaticSymbolTable)
                            -> (String, usize)
{
    // TODO: handle array access

    let mut plp = PLPWriter::new();
    let mut index = start;
    while index < (tokens.len() - 1)
    {
        let token = &tokens[index];

        if token.name == "identifier"
        {
            let lookahead_token = &tokens[index + 1];

            // Method call
            if lookahead_token.value == "("
            {
                // compile the method and append it directly to the compiled plp code
                let method_code = compile_method_call(tokens, index, current_namespace, temp_register, load_registers, symbols);
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
pub fn compile_method_call( tokens: &Vec<Token>,
                        start: usize,
                        current_namespace: &str,
                        arg_register: &str,
                        load_registers: (&str, &str),
                        symbols: &StaticSymbolTable)
                        -> String
{
    let mut plp = PLPWriter::new();

    // start at the token AFTER the open parenthesis
    let mut index = start + 2;

    // Index OF the closing parenthesis
    let end_index = identify_body_bounds(&tokens, index, ("(", ")")).unwrap();

    // TODO: Keep track of argument types, in order, to determine the method signature
    let mut argument_types: Vec<&str> = Vec::new();

    while index < end_index
    {
        let token = &tokens[index];
        if token.value == "("
        {
            panic!("Parenthesis surrounding expression currently unsupported");
        }
        else if token.value == ","
        {
            // Skip commas, arguments are separated by the stack divisors
            index += 1;
            continue;
        }
        else
        {
            // Load argument into arg_register
            let (code, new_index) = compile_arithmetic_statement(tokens, index, current_namespace, "$t9", load_registers, arg_register, symbols);
            plp.code.push_str(&*code);
            index = new_index;

            // Push argument to the stack
            plp.push(arg_register);

            // TODO: push argument_type to argument_types
        }
    }

    // Find nested method calls
    // Handle each argument one at a time, and push each to the stack

    // TODO: determine namespace from caller and current_namespace
    let namespace = "";

    let id_token = &tokens[start];
    let method_name = &*id_token.value;
    // TODO: determine if method is static
    // TODO: if function is non-static, push $this to stack

    let method_symbol = symbols.lookup_function(namespace, method_name, &argument_types).unwrap();
    match method_symbol.location
    {
        SymbolLocation::Register(name) => {
                panic!("Found method at a Register instead of a constant Memory address");
            },
        SymbolLocation::Memory(address) => {
                plp.call(address.label_name);
            },
        SymbolLocation::InstancedMemory(offset) => {
                panic!("Found method at InstancedMemory instead of a constant Memory address");
            },
        SymbolLocation::Structured => {
                // TODO: call constructor
                panic!("Constructors currently unsupported");
            },
    };

    return plp.code;
}

/// Future revision: @return (code, end_index, result_type)
pub fn compile_arithmetic_statement(tokens: &Vec<Token>,            // used
	                                start: usize,                   // used
	                                current_namespace: &str,        // indirect
	                                temp_register: &str,   			// used
	                                load_registers: (&str, &str),	// indirect
	                                target_register: &str,			// used
	                                symbols: &StaticSymbolTable)
	                                -> (String, usize)
{
    // TODO: handle parenthesis and order of operations
    let mut plp = PLPWriter::new();

    // Evaluate first symbol and store it in target_register, then push the result to the stack
    let mut index = compile_evaluation(tokens, start, current_namespace, temp_register, load_registers, target_register, symbols, &mut plp);
	plp.push(target_register);

    // loop until arithmetic sequence ends
	let operator_token = &tokens[index];
    if operator_token.name.starts_with("operator")
    {
        // PRESUMPTION: The first operand is at the top of the stack

        // Evaluate the second operand and store the result in temp_register
        let (code, new_index) = compile_arithmetic_statement(tokens, index + 1, current_namespace, temp_register, load_registers, temp_register, symbols);
		index = new_index;
		plp.code.push_str(&*code);

		// Retreive the first operand from the stack and store it in target_register
		plp.pop(target_register);

        // Perform the operation on the first (target_register) and second operand (temp_register) and store the result in target_register
        let code = compile_arithmetic_operation(&operator_token, (target_register, temp_register), target_register);
		plp.code.push_str(&*code);
    }

    (plp.code, index)
}

/// Writes plp code to evaluate a value triggered by the start token
/// If the token is a literal, the literal value will be loaded into the target_register
/// If the token is an identifier, it will be evaluated based on what the symbol represents
/// * If the symbol represents a method, the method will be called and the result stored in target_register
/// * If the symbol represents a variable, or a chain of accessors, the sequence will be evaluated and the result stored in target_register
///
/// This method will compile plp code directly to a PLPWriter as specified
pub fn compile_evaluation(  tokens: &Vec<Token>,            // used
                        start: usize,                   // used
                        current_namespace: &str,        // indirect-------------
                        temp_register: &str,            // used
                        load_registers: (&str, &str),   // indirect-------------
                        target_register: &str,          // used
                        symbols: &StaticSymbolTable,    // indirect-------------
                        plp: &mut PLPWriter)            // used
                        -> usize
{
    let mut token = &tokens[start];
    let mut end_index = start;

    if token.name.starts_with("literal")
    {
        let value = &*token.value;
        plp.li(target_register, value);

        end_index += 1;
    }
    else if token.name == "identifier"
    {
        // handle identifier
        let (access_code, new_index) = compile_symbol_sequence(tokens, start, current_namespace, temp_register, load_registers, target_register, symbols);
        plp.code.push_str(&*access_code);

        end_index = new_index;
    }
    else if token.value == "("
    {
        // TODO: find end bounds and evaluate parenthetical expression
        panic!("Unexpected token: {}\t{}", token.name, token.value);
    }
    else
    {
        panic!("Unexpected token: {}\t{}", token.name, token.value);
    }

    end_index
}

/// Writes plp code to perform a binary operation on two registers, and store the result in a third register
/// The specified registers need not be unique, and may all be the same if desired
/// This method performs only a single operation, and does not check for register validity
/// The register arguments are assumed to be prefaced with '$'
pub fn compile_arithmetic_operation(operator: &Token, operand_registers: (&str, &str), result_register: &str) -> String
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
