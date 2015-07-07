use std::vec::Vec;
use tokens::*;
use lexer::*;
use symbol_table::*;
use support::*;
use plp::PLPWriter;

/// range should start ON the open brace for the method body, and
/// range should end ON the closing brace for the method body
pub fn compile_method_body( tokens: &Vec<Token>,
                            range: (usize, usize),
                            method_symbol: &Symbol,
                            current_namespace: &str,
                            registers: (&str, &str, &str, &str),
                            symbol_table: &StaticSymbolTable)
{
    let (start_index, end_index) = range;

    let mut plp_string: String = String::new();
    let mut index: usize = start_index;

    if tokens[start_index].value != "{" { panic!("Expected '{{' received: {}", tokens[start_index].value); }
    else { index += 1; }

    // ASSUMPTION: before calling a method:
    // * a reference of the caller or $0 (if the method is called statically) will be pushed to the stack
    // * all arguments for the method will be pushed to the stack
    // * the stack pointer $sp at the top of the argument stack will be passed to $a0

    // Methods will store their argument pointer in static memory directly above the method body

    while index < end_index
    {
        let token = &tokens[index];

        if token.value == "class"
        {
            panic!("Unexpected token: {}\t{}", token.name, token.value);
        }
        else if token.value == "{"
        {
            panic!("Nested scopes currently unsupported");
        }
        else if token.name == "type" // || token.name == "identifier"
        {
            panic!("Local variable declarations currently unsupported");
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

        index += 1;
    }
}

pub fn compile_conditional( tokens: &Vec<Token>,
                            start: usize,
                            current_namespace: &str,
                            temp_register: &str, // indirect
                            load_registers: (&str, &str),
                            target_register: &str,
                            symbols: &StaticSymbolTable)
                            -> (String, usize)
{
    let mut plp = PLPWriter::new();
    let mut index = start;
    while index < (tokens.len() - 1)
    {
        let token = &tokens[index];

    }

    // first index AFTER the sequence
    index += 1;

    (plp.code, index)
}

pub fn compile_statement(   tokens: &Vec<Token>,
                            start: usize,
                            current_namespace: &str,
                            temp_register: &str, // indirect
                            load_registers: (&str, &str),
                            target_register: &str,
                            symbols: &StaticSymbolTable)
                            -> (String, usize)
{
    let mut plp = PLPWriter::new();
    let mut index = start;
    while index < (tokens.len() - 1)
    {
        let token = &tokens[index];

        // PRESUMPTION: there is a reference on the stack, unless this is the first symbol AND the scope is static, in which case $0 will be on the stack
        if token.name == "identifier"
        {
            let lookahead_token = &tokens[index + 1];

            // Method call
            if lookahead_token.value == "("
            {
                // compile the method and append it directly to the compiled plp code
                let (method_code, return_type, new_index) = compile_method_call(tokens, index, current_namespace, temp_register, load_registers, symbols);
                plp.code.push_str(&*method_code);
                index = new_index;

                // TODO: pop previous reference from stack
                // TODO: if next token is "." then push value to stack
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
                    SymbolLocation::Memory(ref address) => {
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

                // TODO: pop previous reference from stack
                // TODO: if next token is "." then push value to stack

                index += 1;
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
    }
    // first index AFTER the sequence
    index += 1;

    (plp.code, index)
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
                                temp_register: &str,
                                load_registers: (&str, &str),
                                target_register: &str,
                                symbols: &StaticSymbolTable)
                                -> (String, usize)
{
    // TODO: handle array access

    // TODO: push $this to stack

    let mut plp = PLPWriter::new();
    let mut index = start;
    while index < (tokens.len() - 1)
    {
        let token = &tokens[index];

        // PRESUMPTION: there is a reference on the stack, unless this is the first symbol AND the scope is static, in which case $0 will be on the stack
        if token.name == "identifier"
        {
            let lookahead_token = &tokens[index + 1];

            // Method call
            if lookahead_token.value == "("
            {
                // compile the method and append it directly to the compiled plp code
                let (method_code, return_type, new_index) = compile_method_call(tokens, index, current_namespace, temp_register, load_registers, symbols);
                plp.code.push_str(&*method_code);
                index = new_index;

                // TODO: pop previous reference from stack
                // TODO: if next token is "." then push value to stack
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
                    SymbolLocation::Memory(ref address) => {
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

                // TODO: pop previous reference from stack
                // TODO: if next token is "." then push value to stack

                index += 1;
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
    }
    // first index AFTER the sequence
    index += 1;

    (plp.code, index)
}

/// The range should start at the method identifier
/// The returned end_index will be the index AFTER the closing parenthesis
/// @return (code, return_type, end_index)
pub fn compile_method_call( tokens: &Vec<Token>,
                            start: usize,
                            current_namespace: &str,
                            arg_register: &str,
                            load_registers: (&str, &str),
                            symbols: &StaticSymbolTable)
                            -> (String, String, usize)
{
    let mut plp = PLPWriter::new();

    // start at the token AFTER the open parenthesis
    let mut index = start + 2;

    // Index OF the closing parenthesis
    let end_index = identify_body_bounds(&tokens, index, ("(", ")")).unwrap();

    // TODO: Keep track of argument types, in order, to determine the method signature
    let mut argument_types: Vec<String> = Vec::new();

    while index < end_index
    {
        let token = &tokens[index];
        if token.value == ","
        {
            // Skip commas, arguments are separated by the stack divisors
            index += 1;
            continue;
        }
        else
        {
            // Load argument into arg_register
            let (code, argument_type, new_index) = compile_arithmetic_statement(tokens, index, current_namespace, "$t9", load_registers, arg_register, symbols);
            plp.code.push_str(&*code);
            index = new_index;

            // Push argument_type to argument_types
            argument_types.push(argument_type.clone());

            // Push argument to the stack
            plp.push(arg_register);
        }
    }

    // Find nested method calls
    // Handle each argument one at a time, and push each to the stack

    // TODO: determine namespace from caller and current_namespace
    let namespace = "";

    let id_token = &tokens[start];
    let method_name = &*id_token.value;

    let method_symbol = symbols.lookup_function(namespace, method_name, &argument_types).unwrap();
    // TODO: determine if method is static
    // TODO: if function is non-static, push $this to stack
    let return_type = match method_symbol.symbol_class
    {
        SymbolClass::Variable(variable_type) => {
                panic!("Expected Function found Variable");
            },
        SymbolClass::Function(return_type, argument_types) => return_type,
        SymbolClass::Structure(subtype) => {
                panic!("Expected Function found Structure");
            }
    };
    match method_symbol.location
    {
        SymbolLocation::Register(name) => {
                panic!("Found method at a Register instead of a constant Memory address");
            },
        SymbolLocation::Memory(ref address) => {
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

    //Return index AFTER the closing parenthesis
    return (plp.code, return_type.to_string(), end_index + 1);
}

/// @return (code, result_type, end_index)
pub fn compile_arithmetic_statement(tokens: &Vec<Token>,            // used
	                                start: usize,                   // used
	                                current_namespace: &str,        // indirect
	                                temp_register: &str,   			// used
	                                load_registers: (&str, &str),	// indirect
	                                target_register: &str,			// used
	                                symbols: &StaticSymbolTable)	// indirect
	                                -> (String, String, usize)
{
    // TODO: handle order of operations
    let mut plp = PLPWriter::new();
    let first_token = &tokens[start];

    if first_token.value == "("
    {
        let (code, result_type, end_index) = compile_arithmetic_statement(tokens, start + 1, current_namespace, temp_register, load_registers, temp_register, symbols);
        // Return index AFTER closing parenthesis
        return (code, result_type, end_index + 1);
    }
    else
    {
        // Evaluate first symbol and store it in target_register, then push the result to the stack
        let mut index = compile_evaluation(tokens, start, current_namespace, temp_register, load_registers, target_register, symbols, &mut plp);
    	plp.push(target_register);

        // loop until arithmetic sequence ends
    	let operator_token = &tokens[index];
        if operator_token.name.starts_with("operator")
        {
            // PRESUMPTION: The first operand is at the top of the stack

            // Evaluate the second operand and store the result in temp_register
            let (code, operand_type, new_index) = compile_arithmetic_statement(tokens, index + 1, current_namespace, temp_register, load_registers, temp_register, symbols);
    		index = new_index;
    		plp.code.push_str(&*code);

    		// Retreive the first operand from the stack and store it in target_register
    		plp.pop(target_register);

            // Perform the operation on the first (target_register) and second operand (temp_register) and store the result in target_register
            let code = compile_arithmetic_operation(&operator_token, (target_register, temp_register), target_register);
    		plp.code.push_str(&*code);

            // push the value to the stack, for the next operand
            plp.push(target_register);
        }

        // Load the final result into target_register
        plp.pop(target_register);

        // TODO: determine real type instead of "Number"
        return (plp.code, "Number".to_string(),index);
    }
}

/// Evaluates either a literal, or a sequence of symbols (variables, accessors, method calls, etc)
///
/// Writes to the specified PLPWriter code to evaluate a value triggered by the start token
/// If the token is a literal, the literal value will be loaded into the target_register
/// If the token is an identifier, it will be evaluated based on what the symbol represents
/// * If the symbol represents a method, the method will be called and the result stored in target_register
/// * If the symbol represents a variable, or a chain of accessors, the sequence will be evaluated and the result stored in target_register
///
/// This method will compile plp code directly to a PLPWriter as specified
///
/// @return
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
