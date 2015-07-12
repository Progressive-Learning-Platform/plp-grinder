mod symbol_analysis;

use std::vec::Vec;
use compiler::symbol_analysis::*;
use tokens::*;
use symbols::*;
use symbols::symbol_table::*;
use support::*;
use plp::PLPWriter;

/// ASSUMPTION: before calling a method:
/// * a reference of the caller or $0 (if the method is called statically) will be loaded to call_buffer
/// * all arguments for the method will be pushed to the stack
/// * the stack pointer $sp at the top of the argument stack will be passed to $a0
///
/// range should start ON the open brace for the method body, and
/// range should end AFTER the closing brace for the method body
pub fn compile_method_body( tokens: &Vec<Token>,
                            range: (usize, usize),
                            method_symbol: &Symbol,
                            current_namespace: &str,
                            registers: (&str, &str, &str, &str, &str),
                            symbol_table: &StaticSymbolTable,
                            plp: &mut PLPWriter)
{
    let (start_index, end_index) = range;
    let mut index: usize = start_index;

    // Validate start token
    if tokens[start_index].value != "{" { panic!("Expected '{{' received: {}", tokens[start_index].value); }
    else { index += 1; }

    // Get method information
    let (method_label, return_label) = get_method_labels(method_symbol);
    let (memory_label, memory_size) = get_static_allocation(method_symbol);
    let expected_return_type = get_return_type_of(method_symbol);

    // Space methods with a newline
    plp.println();

    // Annotate declaration
    let mut annotation = "Method declaration: ".to_string();
    annotation.push_str(&*method_symbol.name);
    annotation.push_str(&*get_arg_signature_of(method_symbol));
    annotation.push_str(" in namespace ");
    annotation.push_str(&*method_symbol.namespace);
    plp.annotate(&*annotation);

    // Compile method headers (save method state and setup method body)
    plp.label(&*memory_label);
    plp.indent_level += 1;
    plp.space(memory_size);
    plp.indent_level -= 1;

    plp.label(&*method_label);
    plp.indent_level += 1;
    compile_save_method_state(method_symbol, (registers.0, registers.1), plp);

    // Get namespace of method block (the method's namespace + the method's name)
    let mut inner_namespace = String::new();
    inner_namespace.push_str(current_namespace);
    inner_namespace.push_str("_");
    inner_namespace.push_str(&*method_symbol.name);

    // Compile method body
    println!("compile_method_body: Start: {} End: {}", start_index, end_index);
    plp.annotate("Start of method body");
    compile_body(tokens, &*expected_return_type, &*inner_namespace, &*return_label, None, None, index, &*inner_namespace, registers, symbol_table, plp);
    plp.annotate("End of method body");

    // Compile method footers (restore method state, cleanup stack, and return)
    println!("Method compiled: {}\n", inner_namespace);
    plp.println();

    plp.annotate("Start of method return");
    plp.label(&*return_label);
    compile_restore_method_state(method_symbol, (registers.0, registers.1), plp);
    plp.ret();
    plp.annotate("End of method return");

    plp.indent_level -= 1;
    let mut annotation = "End of method declaration: ".to_string();
    annotation.push_str(&*method_symbol.name);
    annotation.push_str(&*get_arg_signature_of(method_symbol));
    annotation.push_str(" in namespace ");
    annotation.push_str(&*method_symbol.namespace);
    plp.annotate(&*annotation);
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

    // Annotate stack frame
    plp.annotate("Push local memory of this method to the stack, and restore it at the end of the method call");

    // *Push static memory
    plp.li(registers.0, &*label_name.clone());
    for var_index in 0..var_count
    {
        let offset = 4 * var_index;
        plp.lw(registers.1, offset, registers.0);
        plp.push(registers.1);
    }
    plp.annotate_newline();

    // *Push arg_stack pointer
    plp.annotate("Save the location of the previous argument stack");
    plp.li(registers.0, "arg_stack");
    plp.lw(registers.1, 0, registers.0);
    plp.push(registers.1);
    plp.annotate_newline();

    // *Load $a0 as the new arg_stack pointer
    plp.annotate("The pointer to the agument stack for this method call is stored in $a0...");
    plp.annotate("...Load it as the current argument stack");
    plp.sw("$a0", 0, registers.0);
    plp.annotate_newline();

    // *Push caller
    plp.annotate("Save the previous caller reference to the stack");
    plp.li(registers.0, "caller");
    plp.lw(registers.1, 0, registers.0);
    plp.push(registers.1);
    plp.annotate_newline();

    // *Make call_buffer the current caller
    plp.annotate("The caller of this method call is stored in the call_buffer...");
    plp.annotate("...Load it as the current caller");
    plp.li(registers.1, "call_buffer");
    plp.lw(registers.1, 0, registers.1);
    plp.sw(registers.1, 0, registers.0);
    plp.annotate_newline();
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
    plp.annotate("Restore the previous caller");
    plp.li(registers.0, "caller");
    plp.pop(registers.1);
    plp.sw(registers.1, 0, registers.0);
    plp.annotate_newline();

    // *Restore arg_stack pointer (discard old value)
    plp.annotate("Restore the pointer to the previous argument stack");
    plp.li(registers.0, "arg_stack");
    plp.pop(registers.1);
    plp.sw(registers.1, 0, registers.0);
    plp.annotate_newline();

    // *Restore static memory
    plp.annotate("Restore the static memory of the previous call to this method");
    plp.li(registers.0, &*label_name.clone());
    for var_index in (0..var_count).rev()
    {
        let offset = 4 * var_index;
        plp.pop(registers.1);
        plp.sw(registers.1, offset, registers.0);
    }
    plp.annotate_newline();
}

/// This method does not write annotations at the start or end of the body.
/// The calling method should handle these annotations based on the type of body it is
/// start_index is assumed to be AFTER the open brace
/// @return index AFTER the closing brace
pub fn compile_body(tokens: &Vec<Token>,
                    expected_return_type: &str,
                    body_name: &str,
                    return_label: &str,
                    break_label: Option<&str>,
                    continue_label: Option<&str>,
                    start_index: usize,
                    namespace: &str,
                    registers: (&str, &str, &str, &str, &str),
                    symbol_table: &StaticSymbolTable,
                    plp: &mut PLPWriter) -> usize
{
    let mut index = start_index;
    let mut nested_loop_count = 0;
    let mut nested_conditional_count = 0;
    let mut nested_switch_count = 0;

    while index < tokens.len()
    {
        let token = &tokens[index];
        println!("compile_body: compiling token | {} | {}: {}", index, token.value, token.name);

        if token.value == "}"
        {
            // go to index AFTER ending brace
            index += 1;
            break;
        }
        else if token.value == "break"
        {
            panic!("Unsupported token at {} | {}: {}", index, token.value, token.name);
        }
        else if token.value == "continue"
        {
            panic!("Unsupported token at {} | {}: {}", index, token.value, token.name);
        }
        else if token.value == "return"
        {
            println!("compile_body: return token found at {}", index);

            let (result_type, end_index) = compile_arithmetic_statement(tokens, index + 1, &*namespace, registers.0, (registers.1, registers.2), "$v0", symbol_table, plp);
            if result_type != expected_return_type
            {
                //panic!("Expected return type ({}) but found ({})", expected_return_type, result_type);
            }

            plp.j(&*return_label);
            index = end_index;
            println!("compile_body: new index is {}", index);
        }
        else if token.name == "construct.conditional"
        {
            plp.annotate("Start of conditional chain");

            println!("compile_body: conditional found at {}", index);
            let mut chain_name = body_name.to_string();
            chain_name.push_str("_conditional");
            chain_name.push_str(&*nested_conditional_count.to_string());
            index = compile_conditional(tokens,
                                        expected_return_type,
                                        return_label, break_label,
                                        continue_label, &*chain_name,
                                        0,
                                        index,
                                        namespace,
                                        registers,
                                        symbol_table,
                                        plp);
            println!("compile_body: new index is {}", index);

            plp.annotate("End of conditional chain");
            plp.annotate_newline();

            nested_conditional_count += 1;
        }
        else if token.name == "construct.handles"
        {
            panic!("compile_statement: Exception handles currently unsupported");
        }
        else if token.name == "construct.switch"
        {
            nested_switch_count += 1;
            panic!("compile_statement: Switch statements currently unsupported");
        }
        else if token.name == "construct.loop"
        {
            let mut annotation = "Start of ".to_string();
            annotation.push_str(&*token.value);
            annotation.push_str(" loop");
            plp.annotate(&*annotation);

            println!("compile_body: loop found at {}", index);
            let mut loop_name = body_name.to_string();
            loop_name.push_str("_loop");
            loop_name.push_str(&*nested_loop_count.to_string());
            index = compile_loop(tokens, expected_return_type, return_label, &*loop_name, index, namespace, registers, symbol_table, plp);
            println!("compile_body: new index is {}", index);

            let mut annotation = "End of ".to_string();
            annotation.push_str(&*token.value);
            annotation.push_str(" loop");
            plp.annotate(&*annotation);
            plp.annotate_newline();

            nested_loop_count += 1;
        }
        else if token.name == "construct.type"
        {
            panic!("compile_statement: Cannot declare class inside execution body.\n\tUnexpected token: {}: {}", token.value, token.name);
        }
        else
        {
            println!("compile_body: statement found at {}", index);
            index = compile_statement(tokens, index, &*namespace, registers, symbol_table, plp);
            println!("compile_body: new index is {}", index);
        }
    }

    return index;
}

/// start_index should be the index of the loop token
/// @return: index AFTER the close brace or closing symbol (e.g. after the semi-colon)
pub fn compile_loop(tokens: &Vec<Token>,
                    expected_return_type: &str,
                    return_label: &str,
                    loop_name: &str,
                    start_index: usize,
                    namespace: &str,
                    registers: (&str, &str, &str, &str, &str),
                    symbol_table: &StaticSymbolTable,
                    plp: &mut PLPWriter) -> usize
{
    let mut index = start_index;
    let mut token = &tokens[index];

    let continue_label = loop_name.clone();
    let mut break_label = loop_name.to_string();
    break_label.push_str("_break");

    let result_register = registers.3;
    let mut body_name = loop_name.to_string();
    body_name.push_str("_nested");

    if token.value == "do"
    {
        panic!("Do/while loop is currently unsupported. Stopped on token {} | {}: {}", index, token.value, token.name);
    }
    else if token.value == "while"
    {
        // Continue at condition evaluation
        plp.label(continue_label);

        // Evaluate condition
        plp.annotate("Evaluate condition for while loop");
        let (result_type, end_index) = compile_arithmetic_statement(tokens, index + 2, namespace, registers.0, (registers.1, registers.2), result_register, symbol_table, plp);
        index = end_index;
        token = &tokens[index];
        plp.annotate("If while condition is true (i.e. not 0), then perform the body");
        plp.annotate("Else, jump to the break lable, and stop looping");
        plp.beq(result_register, "$0", &*break_label);

        if tokens[index].value != "{"
        {
            println!("Unwrapped bodies are not currently supported");
            panic!("compile_loop: Expected {{ found {} at {}", token.value, index);
        }

        plp.annotate("Start of body of while loop");
        index = compile_body(tokens, expected_return_type, &*body_name, return_label, Some(&*break_label), Some(continue_label), index + 1, namespace, registers, symbol_table, plp);
        plp.annotate("At the end of each iteration of the loop, go back to check the condition again, and continue to loop if it is true");
        plp.j(continue_label);
        plp.annotate("End of body of while loop");
        plp.label(&*break_label);
        plp.annotate_newline();
    }
    else if token.value == "for"
    {
        let mut continue_label = loop_name.to_string();
        continue_label.push_str("_continue");

        let body_label = loop_name.clone();

        // Init statement
        plp.annotate("Initial statement of for loop");
        index = compile_statement(tokens, index + 2, namespace, registers, symbol_table, plp);
        plp.annotate_newline();

        // Evaluate condition
        plp.label(body_label);
        plp.annotate("Evaluate condition of for loop");
        let (result_type, end_index) = compile_arithmetic_statement(tokens,
                                                                    index,
                                                                    namespace,
                                                                    registers.0,
                                                                    (registers.1, registers.2),
                                                                    result_register,
                                                                    symbol_table,
                                                                    plp);
        index = end_index;
        plp.annotate("If for condition is true (i.e. not 0), then perform the body");
        plp.annotate("Else, jump to the break lable, and stop looping");
        plp.beq(result_register, "$0", &*break_label);

        // Increment statement
        let mut increment_statement = plp.copy();
        index = compile_statement(tokens, index, namespace, registers, symbol_table, &mut increment_statement);
        token = &tokens[index];

        if tokens[index].value != "{"
        {
            println!("Unwrapped bodies are not currently supported");
            panic!("compile_loop: Expected {{ found {} at {}", token.value, index);
        }

        plp.annotate("Start of body of for loop");
        index = compile_body(tokens, expected_return_type, &*body_name, return_label, Some(&*break_label), Some(&*continue_label), index + 1, namespace, registers, symbol_table, plp);

        // Continue at increment statement
        plp.label(&*continue_label);
        plp.annotate("Increment statement for for loop");
        plp.code.push_str(&*increment_statement.code);
        plp.j(body_label);

        plp.annotate("End of body of for loop");
        plp.label(&*break_label);
        plp.annotate_newline();
    }
    else
    {
        panic!("compile_loop: Unexpected token found at {} | {}: {}", index, token.value, token.name);
    }

    index
}

/// @return: index AFTER the close brace
pub fn compile_conditional( tokens: &Vec<Token>,
                            expected_return_type: &str,
                            return_label: &str,
                            break_label: Option<&str>,
                            continue_label: Option<&str>,
                            chain_name: &str,
                            else_block_index: u16,
                            start_index: usize,
                            namespace: &str,
                            registers: (&str, &str, &str, &str, &str),
                            symbol_table: &StaticSymbolTable,
                            plp: &mut PLPWriter) -> usize
{
    let mut index = start_index;
    let mut token = &tokens[index];

    let mut body_name = chain_name.to_string();
    body_name.push_str(&*else_block_index.to_string());
    body_name.push_str("_nested");

    let mut chain_end_label = chain_name.to_string();
    chain_end_label.push_str("_end");

    let mut else_label = chain_name.to_string();
    else_label.push_str("_else");
    else_label.push_str(&*else_block_index.to_string());

    let else_block_index = else_block_index + 1;

    if token.value != "if" { panic!("compile_conditional: Expected 'if' found {}", token.value); }
    // Continue
    index += 1;
    token = &tokens[index];

    // Evaluate condition
    plp.annotate("Evaluate if condition");
    let result_register = registers.3;
    let (result_type, end_index) = compile_arithmetic_statement(tokens,
                                                                index,
                                                                &*namespace,
                                                                registers.0,
                                                                (registers.1, registers.2),
                                                                result_register,
                                                                symbol_table,
                                                                plp);
    index = end_index;
    token = &tokens[index];
    plp.beq(result_register, "$0", &*else_label);

    if token.value != "{"
    {
        println!("Unwrapped bodies are not currently supported");
        panic!("compile_conditional: Expected {{ found {} at {}", token.value, index);
    }

    // Index AFTER the closing brace
    plp.annotate("Start if body");
    index = compile_body(tokens, expected_return_type, &*body_name, return_label, break_label, continue_label, index + 1, namespace, registers, symbol_table, plp);
    plp.j(&*chain_end_label);
    plp.label(&*else_label);

    // Handle "else if" and "else"
    token = &tokens[index];
    if token.value == "else"
    {
        index += 1;
        token = &tokens[index];
        if token.value == "if"
        {
            // Recurse
            plp.annotate("Start else chain");
            return compile_conditional(tokens, expected_return_type, return_label, break_label, continue_label, chain_name, else_block_index, start_index, namespace, registers, symbol_table, plp);
        }
        else if token.value == "{"
        {
            let mut body_name = chain_name.to_string();
            body_name.push_str(&*(else_block_index + 1).to_string());
            body_name.push_str("_nested");

            // Index AFTER the closing brace
            plp.annotate("Start else body");
            index = compile_body(tokens, expected_return_type, &*body_name, return_label, break_label, continue_label, index + 1, namespace, registers, symbol_table, plp);
            plp.annotate("End else body");
        }
        else
        {
            println!("Unwrapped bodies are not currently supported");
            panic!("compile_conditional: Expected {{ or 'if' found {} at {}", token.value, index);
        }
    }

    // If there is no else block, or if the else block has no 'if' attached to it, write the end label and return
    plp.label(&*chain_end_label);
    index
}

/// A statement includes any executable statement inside an executable body, which cannot have its own body
///
/// Specifically, this includes:
/// * method calls
/// * variable assignments
/// * symbol sequences (e.g. accessed method calls and accessed variables)
/// And excludes:
/// * conditional statements
/// * loops
/// * blocks
/// * method declarations
/// * class declarations
///
/// This explicitly ignores:
/// * variable declarations
///
/// range should start ON the open brace for the method body, and
/// range should end ON the closing brace for the method body
/// @return the index AFTER the end of this statement (e.g. after a semi-colon or end brace)
pub fn compile_statement(   tokens: &Vec<Token>,
                            start_index: usize,
                            namespace: &str,
                            registers: (&str, &str, &str, &str, &str),
                            symbol_table: &StaticSymbolTable,
                            plp: &mut PLPWriter) -> usize
{
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
        else if token.name == "type"
        {
            // IGNORE
            println!("compile_statement:ignoring token at {}", index);
            index += 1;
        }
        else if token.name.starts_with("literal")
        {
            panic!("compile_statement: Literal on left hand side");
        }
        else if token.name == "identifier"
        {
            println!("compile_statement: found identifier {} | {}: {}", index, token.value, token.name);
            // TODO: determine memory location of nested access
            index = compile_symbol_sequence(tokens, index, namespace, registers.0, (registers.1, registers.2), target_register, Some(address_register), symbol_table, plp);
            println!("compile_statement: new index is {}", index);
        }
        else if token.value == "="
        {
            println!("compile_statement: found assignment {} | {}: {}", index, token.value, token.name);
            plp.push(address_register);
            let (result_type, new_index) = compile_arithmetic_statement(tokens,
                                                                        index + 1,
                                                                        namespace,
                                                                        registers.0,
                                                                        (registers.1, registers.2),
                                                                        target_register,
                                                                        symbol_table,
                                                                        plp);
            plp.pop(address_register);
            plp.sw(target_register, 0, address_register);
            index = new_index;
            println!("compile_statement: new index is {}", index);
        }
        else if token.value == "+="
        {
            plp.push(address_register);
            let (result_type, new_index) = compile_arithmetic_statement(tokens,
                                                                        index + 1,
                                                                        namespace,
                                                                        registers.0,
                                                                        (registers.1, registers.2),
                                                                        target_register,
                                                                        symbol_table,
                                                                        plp);
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

    return index;
}

/// Write PLP code to evaluate the given symbol sequence, and load the result into a specific register
/// A sequence can be:
/// * a  variable reference,
/// * a method call,
/// * a variable accessed from another symbol (e.g. foo.bar or Foo.staticBar),
/// * a method accessed from another symbol (e.g. foo.bar() or Foo.staticBar()), or
/// * a complex chain of the above (e.g. foo.method().valueInReturnValue.value.method())
///
/// The start index should be the first symbol in the sequence
/// @return the first index AFTER this symbol sequence (e.g. a semi-colon or parenthesis)
pub fn compile_symbol_sequence( tokens: &Vec<Token>,
                                start: usize,
                                namespace: &str,
                                temp_register: &str,
                                load_registers: (&str, &str),
                                target_register: &str,
                                address_register: Option<&str>,
                                symbols: &StaticSymbolTable,
                                plp: &mut PLPWriter) -> usize
{
    // TODO: handle array access

    let mut index = start;
    let mut valid_address = false;

    // Save call buffer
    plp.annotate("Save call buffer");
    plp.li(load_registers.0, "call_buffer");
    plp.lw(load_registers.0, 0, load_registers.0);
    plp.push(load_registers.0);
    plp.annotate("End save call buffer");
    plp.annotate_newline();

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
                let (return_type, new_index) = compile_method_call(tokens, index, namespace, temp_register, load_registers, symbols, plp);
                plp.annotate("Retreive return value from method call");
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
                println!("\tcompile_symbol_sequence: symbol lookup: {} : {}", namespace, &*token.value);
                let symbol = symbols.lookup_variable(namespace, &*token.value).unwrap();
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
    plp.annotate("Restore call buffer");
    plp.li(load_registers.1, "call_buffer");
    plp.pop(load_registers.0);
    plp.sw(load_registers.0, 0, load_registers.1);
    plp.annotate("End restore call buffer");
    plp.annotate_newline();

    if !valid_address
    {
        match address_register
        {
            Some(_) => { panic!("Cannot store address of register or method call"); },
            None    => { /* DO NOTHING */ },
        }
    }

    index
}

/// The range should start at the method identifier
/// The returned end_index will be the index AFTER the closing parenthesis
/// @return (return_type, end_index)
pub fn compile_method_call( tokens: &Vec<Token>,
                            start: usize,
                            current_namespace: &str,
                            arg_register: &str,
                            load_registers: (&str, &str),
                            symbols: &StaticSymbolTable,
                            plp: &mut PLPWriter) -> (String, usize)
{
    // start at the token AFTER the open parenthesis
    let mut index = start + 2;

    // Index OF the closing parenthesis
    let end_index = identify_body_bounds(&tokens, index, ("(", ")")).unwrap();

    // TODO: Keep track of argument types, in order, to determine the method signature
    let mut argument_types: Vec<String> = Vec::new();

    plp.annotate("Start method call");
    plp.annotate("Evaluate method arguments, and push each argument to the stack");

    let mut argument_index = 1;
    while index < end_index
    {
        let token = &tokens[index];
        println!("\t\tcompile_method_call: processing token at {} | {}: {}", index, token.value, token.name);
        if token.value == ","
        {
            // Skip commas, arguments are separated by the stack divisors
            index += 1;
            argument_index += 1;
            continue;
        }
        else
        {
            let mut annotation = "@argument".to_string();
            annotation.push_str(&*argument_index.to_string());
            plp.annotate(&*annotation);

            // Load argument into arg_register
            println!("\t\tcompile_method_call: outsourcing to compile_arithmetic_statement");
            let (argument_type, new_index) = compile_arithmetic_statement(tokens, index, current_namespace, "$t9", load_registers, arg_register, symbols, plp);
            index = new_index;

            // Push argument_type to argument_types
            argument_types.push(argument_type.clone());

            // Push argument to the stack
            plp.push(arg_register);
        }
    }

    plp.annotate("End argument evaluation");

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

    plp.annotate("End method call");

    //Return index AFTER the closing parenthesis
    return (return_type.to_string(), end_index + 1);
}

/// Compiles one or more symbol sequences linked by zero or more operators.
/// @return (result_type, end_index)
pub fn compile_arithmetic_statement(tokens: &Vec<Token>,            // used
	                                start: usize,                   // used
	                                namespace: &str,        // indirect
	                                temp_register: &str,   			// used
	                                load_registers: (&str, &str),	// indirect
	                                target_register: &str,			// used
	                                symbols: &StaticSymbolTable,	// indirect
                                    plp: &mut PLPWriter) -> (String, usize)
{
    // TODO: handle order of operations
    let mut index = start;
    let token = &tokens[index];

    println!("\tcompile_arithmetic_statement: Received {} | {}: {}", index, token.value, token.name);

    // Evaluate first symbol and store it in target_register, then push the result to the stack

    if token.value == "("
    {
        // TODO: verify result type
        // Begin evaluation AFTER the parenthesis
        let (result_type, end_index) = compile_arithmetic_statement(tokens, index + 1, namespace, temp_register, load_registers, target_register, symbols, plp);
        plp.push(target_register);
        // Continue parsing AFTER closing parenthesis
        index = end_index + 1;
    }
    else
    {
        index = compile_evaluation(tokens, index, namespace, temp_register, load_registers, target_register, symbols, plp);
    	plp.push(target_register);
    }

    // Recurse until arithmetic sequence ends (e.g. sees a non-oporator pattern)
	let operator_token = &tokens[index];
    if operator_token.name.starts_with("operator")
    {
        // PRESUMPTION: The first operand is at the top of the stack

        // Evaluate the second operand and store the result in target_register
        let (operand_type, new_index) = compile_arithmetic_statement(tokens, index + 1, namespace, temp_register, load_registers, target_register, symbols, plp);
		index = new_index;
        // TODO: determine return type from operand_types

		// Retreive the first operand from the stack and store it in temp_register
		plp.pop(temp_register);

        // Perform the operation on the first (target_register) and second operand (temp_register) and store the result in target_register
        compile_arithmetic_operation(&operator_token, (temp_register, target_register), target_register, plp);

        // push the value to the stack, for the next operand
        plp.push(target_register);
    }

    // Load the final result into target_register
    plp.pop(target_register);

    // TODO: determine actual return type instead of "Number"
    return ("Number".to_string(),index);
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
/// @return the index after the evaluation (e.g. the index OF another symbol sequence, a semi-colon, a parenthesis, etc)
pub fn compile_evaluation(  tokens: &Vec<Token>,            // used
                            start: usize,                   // used
                            namespace: &str,        // indirect-------------
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
        // evaluate identifier
        end_index = compile_symbol_sequence(tokens, start, namespace, temp_register, load_registers, target_register, None, symbols, plp);
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
pub fn compile_arithmetic_operation(operator: &Token, operand_registers: (&str, &str), result_register: &str, plp: &mut PLPWriter)
{
    match &*operator.value
    {
        "+" => plp.addu(result_register, operand_registers.0, operand_registers.1),
        "-" => plp.subu(result_register, operand_registers.0, operand_registers.1),
        "*" => plp.mullo(result_register, operand_registers.0, operand_registers.1),
         _  => panic!("Unsupported operator: {}: {}", operator.name, operator.value)
    };
}
