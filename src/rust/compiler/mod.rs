use std::vec::Vec;
use tokens::*;
use symbols::*;
use support::*;
use plp::PLPWriter;

/// range should start ON the open brace for the method body, and
/// range should end AFTER the closing brace for the method body
pub fn compile_method_body( tokens: &Vec<Token>,
                            range: (usize, usize),
                            method_symbol: &Symbol,
                            current_namespace: &str,
                            registers: (&str, &str, &str, &str, &str),
                            symbol_table: &StaticSymbolTable) -> String
{
    let (start_index, end_index) = range;

    let mut plp = PLPWriter::new();
    let mut index: usize = start_index;

    if tokens[start_index].value != "{" { panic!("Expected '{{' received: {}", tokens[start_index].value); }
    else { index += 1; }

    let method_name = match method_symbol.location {
        SymbolLocation::Memory(ref address) => address.label_name.clone(),
        _ => { panic!("compile_method_body: Expected Memory address for method"); },
    };
    let mut return_label = method_name.clone();
    return_label.push_str("_return");

    let (memory_label, memory_size) = match method_symbol.symbol_class {
            SymbolClass::Variable(_) => {
                    panic!("Expected Function found Variable");
                },
            SymbolClass::Function(_, _, ref label_name, var_count) => (label_name, var_count as u16),
            SymbolClass::Structure(ref subtype) => {
                    panic!("Expected Function found {}", subtype);
                }
        };
    plp.label(memory_label);
    plp.space(memory_size);

    plp.label(&*method_name);
    compile_save_method_state(method_symbol, (registers.0, registers.1), &mut plp);

    // ASSUMPTION: before calling a method:
    // * a reference of the caller or $0 (if the method is called statically) will be loaded to call_buffer
    // * all arguments for the method will be pushed to the stack
    // * the stack pointer $sp at the top of the argument stack will be passed to $a0

    // ASSUMPTION: Methods will store their argument pointer in static memory directly above the method body

    let mut realz_namespace = String::new();
    realz_namespace.push_str(current_namespace);
    realz_namespace.push_str("_");
    realz_namespace.push_str(&*method_symbol.name);

    println!("compile_method_body: Start: {} End: {}", start_index, end_index);
    while index < end_index - 1
    {
        let token = &tokens[index];
        println!("compile_method_body: compiling token | {} | {}: {}", index, token.value, token.name);

        if token.value == "return"
        {
            println!("compile_method_body: return token found at {}", index);

            let (code, result_type, end_index) = compile_arithmetic_statement(  tokens,
                                                                                index + 1,
                                                                                &*realz_namespace,
                                                                                registers.0,
                                                                                (registers.1, registers.2),
                                                                                "$v0",
                                                                                symbol_table);
            plp.code.push_str(&*code);
            // TODO: validate return type
            plp.j(&*return_label);
            index = end_index;
            println!("compile_method_body: new index is {}", index);
        }
        else
        {
            println!("compile_method_body: statement found at {}", index);
            let (code, end_index) = compile_statement(tokens, index, method_symbol, &*realz_namespace, registers, symbol_table);
            plp.code.push_str(&*code);
            index = end_index;
            println!("compile_method_body: new index is {}", index);
        }
    }

    println!("\n");
    plp.label(&*return_label);
    compile_restore_method_state(method_symbol, (registers.0, registers.1), &mut plp);
    plp.ret();

    plp.code
}

pub fn compile_save_method_state(   method_symbol: &Symbol,
                                    registers: (&str, &str),
                                    plp: &mut PLPWriter)
{
    // Save current method state to the stack
    // *Determine size and location of static memory
    let (var_count, label_name) = match method_symbol.symbol_class {
            SymbolClass::Variable(_) => {
                    panic!("Expected Function found Variable");
                },
            SymbolClass::Function(_, _, ref label_name, var_count) => (var_count as u16, label_name),
            SymbolClass::Structure(ref subtype) => {
                    panic!("Expected Function found {}", subtype);
                }
        };
    // *Push static memory
    plp.li(registers.0, &*label_name.clone());
    for var_index in 0..var_count
    {
        let offset = 4 * var_index;
        plp.lw(registers.1, offset, registers.0);
        plp.push(registers.1);
    }
    // *Push arg_stack pointer
    plp.li(registers.0, "arg_stack");
    plp.lw(registers.1, 0, registers.0);
    plp.push(registers.1);
    // *Load $a0 as the new arg_stack pointer
    plp.sw("$a0", 0, registers.0);

    // *Push caller
    plp.li(registers.0, "caller");
    plp.lw(registers.1, 0, registers.0);
    plp.push(registers.1);
    // *Make call_buffer the current caller
    plp.li(registers.1, "call_buffer");
    plp.lw(registers.1, 0, registers.1);
    plp.sw(registers.1, 0, registers.0);
}

pub fn compile_restore_method_state(method_symbol: &Symbol,
                                    registers: (&str, &str),
                                    plp: &mut PLPWriter)
{
    // Save current method state to the stack
    // *Determine size and location of static memory
    let (var_count, label_name) = match method_symbol.symbol_class {
            SymbolClass::Variable(_) => {
                    panic!("Expected Function found Variable");
                },
            SymbolClass::Function(_, _, ref label_name, var_count) => (var_count as u16, label_name),
            SymbolClass::Structure(ref subtype) => {
                    panic!("Expected Function found {}", subtype);
                }
        };

    // *Restore caller
    plp.li(registers.0, "caller");
    plp.pop(registers.1);
    plp.sw(registers.1, 0, registers.0);

    // *Restore arg_stack pointer (discard old value)
    plp.li(registers.0, "arg_stack");
    plp.pop(registers.1);
    plp.sw(registers.1, 0, registers.0);

    // *Restore static memory
    plp.li(registers.0, &*label_name.clone());
    for var_index in (0..var_count).rev()
    {
        let offset = 4 * var_index;
        plp.pop(registers.1);
        plp.sw(registers.1, offset, registers.0);
    }
}

// TODO: enable
#[allow(dead_code)]
pub fn compile_conditional( tokens: &Vec<Token>,
                            start: usize,
                            base_label_name: &str,
                            current_namespace: &str,
                            temp_register: &str, // indirect
                            load_registers: (&str, &str),
                            target_register: &str,
                            symbols: &StaticSymbolTable)
                            -> (String, usize)
{
    let mut plp = PLPWriter::new();
    let mut index = start;
    let mut token = &tokens[index];

    if token.value != "if"
    {
        panic!("compile_conditional: Expected 'if' found {}", token.value);
    }

    let (code, result_type, end_index) = compile_arithmetic_statement(tokens, index, current_namespace, temp_register, load_registers, target_register, symbols);
    if result_type != "boolean"
    {
        panic!("compile_conditional: Expected evaluation of boolean, found evaluation of {}", result_type);
    }

    // first index AFTER the sequence
    index += 1;

    (plp.code, index)
}

/// A statement includes any executable statement inside an executable body.
///
/// Specifically, this includes:
/// * method calls
/// * variable assignments
/// * symbol sequences (e.g. accessed method calls and accessed variables)
/// * conditional statements
/// * loops
///
/// This does not support:
/// * blocks, except those of conditionals or loops
/// * method declarations
/// * class declarations
///
/// This explicitly ignores:
/// * variable declarations
///
/// range should start ON the open brace for the method body, and
/// range should end ON the closing brace for the method body
/// returns the index AFTER the end of this statement (e.g. after a semi-colon or end brace)
/// @return (code, end_index)
pub fn compile_statement(   tokens: &Vec<Token>,
                            start_index: usize,
                            method_symbol: &Symbol,
                            current_namespace: &str,
                            registers: (&str, &str, &str, &str, &str),
                            symbol_table: &StaticSymbolTable) -> (String, usize)
{
    let mut plp = PLPWriter::new();
    let mut index: usize = start_index;
    let target_register = registers.3;
    let address_register = registers.4;

    // ASSUMPTION: before calling a method:
    // * a reference of the caller or $0 (if the method is called statically) will be loaded to call_buffer
    // * all arguments for the method will be pushed to the stack
    // * the stack pointer $sp at the top of the argument stack will be passed to $a0

    // ASSUMPTION: Methods will store their argument pointer in static memory directly above the method body

    while index < tokens.len()
    {
        let token = &tokens[index];
        println!("compile_statement: processing token at {} | {}: {}", index, token.value, token.name);
        if token.value == "{"
        {
            panic!("compile_statement: Nested scopes currently unsupported");
        }
        else if token.value == ";"
        {
            println!("compile_statement: found semi-colon; breaking");
            // Index AFTER the last token in this statement
            index += 1;
            break;
        }
        else if token.name == "type" // || token.name == "identifier"
        {
            // IGNORE
            println!("compile_statement:ignoring token at {}", index);
            index += 1;
        }
        else if token.name.starts_with("literal") // || token.name == "identifier"
        {
            panic!("compile_statement: Literal on left hand side");
        }
        // TODO: support structures below
        else if token.name == "construct.conditional"
        {
            panic!("compile_statement: Conditionals currently unsupported");
        }
        else if token.name == "construct.handles"
        {
            panic!("compile_statement: Exception handles currently unsupported");
        }
        else if token.name == "construct.switch"
        {
            panic!("compile_statement: Switch statements currently unsupported");
        }
        else if token.name == "construct.loop"
        {
            panic!("compile_statement: Loops currently unsupported");
        }
        else if token.name == "construct.type"
        {
            panic!("compile_statement: Cannot declare class inside execution body.\n\tUnexpected token: {}: {}", token.value, token.name);
        }
        else if token.name == "identifier"
        {
            println!("compile_statement: found identifier {} | {}: {}", index, token.value, token.name);
            // TODO: determine memory location of nested access
            let (code, new_index) = compile_symbol_sequence(tokens,
                                                            index,
                                                            current_namespace,
                                                            registers.0,
                                                            (registers.1, registers.2),
                                                            target_register,
                                                            Some(address_register),
                                                            symbol_table);
            plp.code.push_str(&*code);
            index = new_index;
            println!("compile_statement: new index is {}", index);
        }
        else if token.value == "="
        {
            println!("compile_statement: found assignment {} | {}: {}", index, token.value, token.name);
            plp.push(address_register);
            let (code, result_type, new_index) = compile_arithmetic_statement(  tokens,
                                                                                index + 1,
                                                                                current_namespace,
                                                                                registers.0,
                                                                                (registers.1, registers.2),
                                                                                target_register,
                                                                                symbol_table);
            plp.code.push_str(&*code);
            plp.pop(address_register);
            plp.sw(target_register, 0, address_register);
            index = new_index;
            println!("compile_statement: new index is {}", index);
        }
        else if token.value == "+="
        {
            plp.push(address_register);
            let (code, result_type, new_index) = compile_arithmetic_statement(  tokens,
                                                                                index + 1,
                                                                                current_namespace,
                                                                                registers.0,
                                                                                (registers.1, registers.2),
                                                                                target_register,
                                                                                symbol_table);
            plp.code.push_str(&*code);
            plp.pop(address_register);
            plp.lw(registers.0, 0, address_register);
            plp.addu(target_register, target_register, registers.0);
            plp.sw(target_register, 0, address_register);

            index = new_index;
        }
        else if token.value == "-="
        {
            panic!("compile_statement: Unsupported operator: {}\t{}", token.name, token.value);
        }
        else if token.value == "*="
        {
            panic!("compile_statement: Unsupported operator: {}\t{}", token.name, token.value);
        }
        else
        {
            panic!("compile_statement: Unexpected token: {}\t{}", token.name, token.value);
        }
    }

    return (plp.code, index);
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
                                address_register: Option<&str>,
                                symbols: &StaticSymbolTable)
                                -> (String, usize)
{
    // TODO: handle array access

    let mut plp = PLPWriter::new();
    let mut index = start;
    let mut valid_address = false;

    // Save call buffer
    plp.li(load_registers.0, "call_buffer");
    plp.lw(load_registers.0, 0, load_registers.0);
    plp.push(load_registers.0);

    while index < (tokens.len() - 1)
    {
        let token = &tokens[index];
        println!("\tcompile_symbol_sequence: processing token at {} | {}: {}", index, token.value, token.name);

        // PRESUMPTION: there is a reference on the stack, unless this is the first symbol AND the scope is static, in which case $0 will be on the stack
        if token.name == "identifier"
        {
            println!("\tcompile_symbol_sequence: identifier found at {}", index);
            let lookahead_token = &tokens[index + 1];

            // Method call
            if lookahead_token.value == "("
            {
                println!("\tcompile_symbol_sequence: identifier represents method call");
                // compile the method and append it directly to the compiled plp code
                let (method_code, return_type, new_index) = compile_method_call(tokens, index, current_namespace, temp_register, load_registers, symbols);
                plp.code.push_str(&*method_code);
                plp.mov(target_register, "$v0");
                index = new_index;
                valid_address = false;

                // TODO: panic if a method call is the last symbol, and address_register is Some(_)
                // TODO: pop previous reference from stack
                // TODO: if next token is "." then push value to stack
            }
            // Variable read
            else
            {
                println!("\tcompile_symbol_sequence: identifier represents variable read");
                println!("\tcompile_symbol_sequence: symbol lookup: {} : {}", current_namespace, &*token.value);
                let symbol = symbols.lookup_variable(current_namespace, &*token.value).unwrap();
                valid_address = false;
                match symbol.location
                {
                    SymbolLocation::Register(ref name) => {
                            plp.mov(target_register, name);
                            println!("\tcompile_symbol_sequence: found {}: Register", &*token.value);
                        },
                    SymbolLocation::Memory(ref address) => {
                            plp.li(load_registers.0, &*address.label_name);
                            plp.lw(target_register, address.offset, load_registers.0);
                            println!("\tcompile_symbol_sequence: found {}: Memory Address", &*token.value);

                            match address_register
                            {
                                Some(register_name) =>
                                {
                                    // Load address into address_register
                                    plp.li(load_registers.1, &*address.offset.to_string());
                                    plp.addu(register_name, load_registers.0, load_registers.1);
                                    valid_address = true;
                                },
                                None    =>
                                {
                                    /* DO NOTHING */
                                },
                            }
                        },
                    SymbolLocation::InstancedMemory(offset) => {
                            // Use base address from call_buffer
                            plp.li(load_registers.0, "call_buffer");
                            plp.lw(load_registers.0, offset, load_registers.0);

                            plp.lw(target_register, offset, load_registers.0);
                            println!("\tcompile_symbol_sequence: found {}: InstancedMemory", &*token.value);
                        },
                    SymbolLocation::MethodArgument(offset) => {
                            //TODO: account for method argument
                            println!("\tcompile_symbol_sequence: found {}: MethodArgument", &*token.value);
                            panic!("compile_symbol_sequence: method arguments currently unsupported!");
                        },
                    SymbolLocation::Structured => {
                            // TODO: append to namespace
                            println!("\tcompile_symbol_sequence: found {}: Strcutured", &*token.value);
                        },
                };

                // Load result into call buffer, for next token
                plp.li(load_registers.0, "call_buffer");
                plp.sw(target_register, 0, load_registers.0);

                index += 1;
            }

            println!("\tcompile_symbol_sequence: new index: {}", index);
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

    // Restore previous call_buffer
    plp.li(load_registers.1, "call_buffer");
    plp.pop(load_registers.0);
    plp.sw(load_registers.0, 0, load_registers.1);

    if !valid_address
    {
        match address_register
        {
            Some(_) => { panic!("Cannot store address of register or method call"); },
            None    => { /* DO NOTHING */ },
        }
    }

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
        println!("\t\tcompile_method_call: processing token at {} | {}: {}", index, token.value, token.name);
        if token.value == ","
        {
            // Skip commas, arguments are separated by the stack divisors
            index += 1;
            continue;
        }
        else
        {
            // Load argument into arg_register
            println!("\t\tcompile_method_call: outsourcing to compile_arithmetic_statement");
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
    let namespace = current_namespace;

    let id_token = &tokens[start];
    let method_name = &*id_token.value;

    println!("\t\tcompile_method_call: lookup method symbol {} | {} | {}", namespace, method_name, argument_types.len());
    let method_symbol = symbols.lookup_function(namespace, method_name, &argument_types).unwrap();
    // TODO: determine if method is static
    // TODO: if function is non-static, push $this to stack
    let return_type = match method_symbol.symbol_class
    {
        SymbolClass::Variable(ref variable_type) => {
                panic!("Expected Function found Variable");
            },
        SymbolClass::Function(ref return_type, _, _, _) => return_type,
        SymbolClass::Structure(ref subtype) => {
                panic!("Expected Function found {}", subtype);
            }
    };
    match method_symbol.location
    {
        SymbolLocation::Register(_) => {
                panic!("Found method at a Register instead of a constant Memory address");
            },
        SymbolLocation::Memory(ref address) => {
                plp.call(&*address.label_name);
            },
        SymbolLocation::InstancedMemory(_) => {
                panic!("Found method at InstancedMemory instead of a constant Memory address");
            },
        SymbolLocation::MethodArgument(offset) => {
                //TODO: account for method argument
                panic!("compile_method_call: method arguments currently unsupported!");
            },
        SymbolLocation::Structured => {
                // TODO: call constructor
                panic!("Constructors currently unsupported");
            },
    };

    //Return index AFTER the closing parenthesis
    return (plp.code, return_type.to_string(), end_index + 1);
}

/// Compiles one or more symbol sequences linked by zero or more operators.
/// @return (code, result_type, end_index)
pub fn compile_arithmetic_statement(tokens: &Vec<Token>,            // used
	                                start: usize,                   // used
	                                current_namespace: &str,        // indirect
	                                temp_register: &str,   			// used
	                                load_registers: (&str, &str),	// indirect
	                                target_register: &str,			// used
	                                symbols: &StaticSymbolTable,	// indirect
                                    )
	                                -> (String, String, usize)
{
    // TODO: handle order of operations
    let mut plp = PLPWriter::new();
    let mut index = start;
    let token = &tokens[index];

    println!("\tcompile_arithmetic_statement: Received {} | {}: {}", index, token.value, token.name);

    // Evaluate first symbol and store it in target_register, then push the result to the stack

    if token.value == "("
    {
        // TODO: verify result type
        // Begin evaluation AFTER the parenthesis
        let (code, result_type, end_index) = compile_arithmetic_statement(tokens, index + 1, current_namespace, temp_register, load_registers, target_register, symbols);
        plp.code.push_str(&*code);
        plp.push(target_register);
        // Continue parsing AFTER closing parenthesis
        index = end_index + 1;
    }
    else
    {
        index = compile_evaluation(tokens, index, current_namespace, temp_register, load_registers, target_register, symbols, &mut plp);
    	plp.push(target_register);
    }

    // Recurse until arithmetic sequence ends (e.g. sees a non-oporator pattern)
	let operator_token = &tokens[index];
    if operator_token.name.starts_with("operator")
    {
        // PRESUMPTION: The first operand is at the top of the stack

        // Evaluate the second operand and store the result in target_register
        let (code, operand_type, new_index) = compile_arithmetic_statement(tokens, index + 1, current_namespace, temp_register, load_registers, target_register, symbols);
		index = new_index;
		plp.code.push_str(&*code);

		// Retreive the first operand from the stack and store it in temp_register
		plp.pop(temp_register);

        // Perform the operation on the first (target_register) and second operand (temp_register) and store the result in target_register
        let code = compile_arithmetic_operation(&operator_token, (temp_register, target_register), target_register);
		plp.code.push_str(&*code);

        // push the value to the stack, for the next operand
        plp.push(target_register);
    }

    // Load the final result into target_register
    plp.pop(target_register);

    // TODO: determine real type instead of "Number"
    return (plp.code, "Number".to_string(),index);
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
    let token = &tokens[start];
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
        let (access_code, new_index) = compile_symbol_sequence(tokens, start, current_namespace, temp_register, load_registers, target_register, None, symbols);
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
        panic!("Unexpected token at {}: {}\t{}", end_index, token.value, token.name);
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
