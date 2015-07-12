extern crate regex;
extern crate getopts;

mod matching;
mod support;
mod tokens;
mod lexer;
mod files;
mod symbols;
mod plp;
mod compiler;

use std::env;
use std::process;
use std::vec::Vec;
use tokens::*;
use symbols::*;
use symbols::symbol_table::*;
use lexer::*;
use support::*;
use files::dump;
use compiler::*;
use plp::PLPWriter;

fn main()
{
    let mut temp_source: String =  String::new();
    let default_source = "sampleData/BasicArithmatic.java";
    let lex_output_file = "sampleData/output/stable/BasicArithmatic.java.lexed";
    let preprocessed_output_file = "sampleData/output/stable/BasicArithmatic.java.preprocessed";

    let args: Vec<String> = env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optopt("s", "src", "Set input file name", "NAME");

    let matches = match opts.parse(&args[1..])
    {
        Ok(m) => m,
        Err(f) => {
                println!("{}", f);
                process::exit(1);
            }
    };

    if matches.opt_present("s")
    {
        let brief = format!("Usage: grinder File [options]");
        println!("{}", opts.usage(&brief));
        temp_source = match matches.opt_str("s")
        {
            Some(ref x) => x.clone(),
            None => String::new(),
        };
    }

    //TODO match options
    if !matches.free.is_empty() {
        println!("Free arguments: {:?}", matches.free);
    }

    if temp_source.is_empty()
    {
        temp_source = default_source.to_string();
    }

    let source_file = &*temp_source.clone();
    let was_compile_successful = compile_oracle(&["javac", "-d", "output/temp/class", source_file]);

    if was_compile_successful
    {
        let mut tokens: Vec<Token> = lex_file(source_file, false);
        tokens.print_to(lex_output_file, false);

        remove_meta(&mut tokens);
        tokens.print_to(preprocessed_output_file, false);

        let mut symbols_table: SymbolTable = SymbolTable::new();
        let class_structure = parse_class(&tokens, 1, &mut symbols_table);

        let main_symbol = symbols_table.lookup_by_name("main")[0];
        let main_label = match main_symbol.location {
                SymbolLocation::Memory(ref address) => address.label_name.clone(),
                _ => { panic!("Main found was not a function!"); },
            };
        let mut plp = PLPWriter::new();
        plp.org("0x10000000");
        plp.equ("true", 1);
        plp.equ("false", 0);
        plp.li("$sp", "0x10fffffc");
        plp.println();

        plp.call(&*main_label);
        plp.j("end");
        plp.println();

        plp.label("call_buffer");
        plp.indent_level += 1;
        plp.word(0);
        plp.indent_level -= 1;

        plp.label("caller");
        plp.indent_level += 1;
        plp.word(0);
        plp.indent_level -= 1;

        plp.label("arg_stack");
        plp.indent_level += 1;
        plp.word(0);
        plp.indent_level -= 1;
        for static_method in class_structure.static_methods
        {
            let range = (static_method.0, static_method.1);
            let name = static_method.2;
            let namespace = static_method.3;
            let argument_types = static_method.4;

            let function_symbol = symbols_table.lookup_function(&*namespace, &*name, &argument_types.unwrap()).unwrap();

            let registers = ("$t0", "$t1", "$t2", "$t3", "$t4");
            compile_method_body(&tokens, range, function_symbol, &*namespace, registers, &symbols_table, &mut plp);
        }
        plp.label("end");

        dump("output.asm", plp.code);
    }
}

fn parse_class(tokens: &Vec<Token>, start_index: usize, symbols_table: &mut SymbolTable) -> ClassStructure
{
    let mut class_structure: ClassStructure = ClassStructure::new();
    let mut current_namespace: String = String::new();
    let mut current_local_class_variables = 0;
    let mut current_static_class_variables = 0;


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

                let (name, variable_type, is_static, symbol_class) = parse_variable(tokens, index);
                symbols_table.add(symbol_class, current_namespace.clone(), name.clone(), is_static, false, false, current_local_class_variables, current_static_class_variables, 0);
                current_static_class_variables += 1;

                class_structure.static_variables.push(MemberBlock (low, high, name.clone(), current_namespace.clone(), None));

                min_value =  low;
                min_value -= index;
            }
            else if tokens[index + skip_amount].name == "identifier"
            {
                //TODO account for final
                println!("------Incoming Static Variable Decl!");

                let low = index + 2;
                let high = find_next(tokens, low, ";").unwrap() + 1;

                let (name, variable_type, is_static, symbol_class) = parse_variable(tokens, index);
                symbols_table.add(symbol_class, current_namespace.clone(), name.clone(), is_static, false, false, current_local_class_variables, current_static_class_variables, 0);
                current_static_class_variables += 1;

                class_structure.static_variables.push(MemberBlock (low, high, name.clone(), current_namespace.clone(), None));

                min_value =  low;
                min_value -= index;
            }
            else if tokens[index + 1].name.starts_with("construct")
            {
                if tokens[index + skip_amount].name.starts_with("control")
                {
                    //TODO add symbol to table
                    //TODO account for final
                    println!("------Incoming Static Class Decl!");
                    let starting_point = find_next(tokens, index, "{").unwrap() + 1;
                    min_value = identify_body_bounds(tokens, starting_point, ("{", "}")).unwrap() + 1;

                    //let temp_symbol = *symbols_table.lookup_variable(&*current_namespace, &*name);
                    class_structure.static_classes.push(MemberBlock (starting_point - 1, min_value, tokens[index + 2].value.clone(), current_namespace.clone(), None));
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

                let (method_name, argument_types) = parse_method(tokens, index, symbols_table, min_value, current_namespace.clone());

                class_structure.static_methods.push(MemberBlock (starting_point - 1, min_value, method_name.clone(), current_namespace.clone(), Some(argument_types)));
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
                //TODO add symbol to table
                //TODO account for final
                println!("------Incoming Non-Static Class Decl!");
                let index_after_brace = index + skip_amount + 1;
                current_namespace = tokens[index + 1].value.clone();
                min_value = identify_body_bounds(tokens, index_after_brace, ("{", "}")).unwrap() + 1;

                //symbols_table.add(symbol_class, current_namespace.clone(), name.clone(), is_static, false, false, current_local_class_variables, current_static_class_variables, 0);

                class_structure.non_static_classes.push(MemberBlock (index_after_brace - 1, min_value, tokens[index + 1].value.clone(), current_namespace.clone(), None));
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

                let (method_name, argument_types) = parse_method(tokens, index, symbols_table, min_value, current_namespace.clone());

                class_structure.non_static_methods.push(MemberBlock (starting_point - 1, min_value, method_name.clone(), current_namespace.clone(), Some(argument_types)));
                min_value -= index + 1;
                //check for control
            }
            else if tokens[index + skip_amount].name.starts_with("operator")
            {
                //TODO account for final
                println!("------Incoming Non-Static Variable Decl!");
                min_value =  find_next(tokens, index, ";").unwrap() + 1;

                let (name, variable_type, is_static, symbol_class) = parse_variable(tokens, index);
                symbols_table.add(symbol_class, current_namespace.clone(), name.clone(), is_static, false, false, current_local_class_variables, current_static_class_variables, 0);
                current_local_class_variables += 1;

                class_structure.non_static_variables.push(MemberBlock (index, min_value, name.clone(), current_namespace.clone(), None));
                min_value -= index + 1;
            }
        }
        println!("\tIndex: {} | Token -> {} : {}", index, token.value, token.name );
        //deal with parameters
    }
    println!("\n<                 Class Overview                  >");
    println!("<---------------- Static Variables --------------->");
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

    let mut symbols_table_dump: String = String::new();

    println!("\n<                    Overview                     >");
    for symbol in symbols_table.children_scopes.iter()
    {
        let mut offset = match symbol.location
            {
                SymbolLocation::Memory(ref memory_address) => memory_address.offset,
                SymbolLocation::MethodArgument(offset) => offset,
                _ => -1,
            };
        let mut label_name_string = match symbol.location
            {
                SymbolLocation::Memory(ref memory_address) => memory_address.label_name.clone(),
                _ => String::new(),
            };
        println!("SYMBOL: {}/{}/{}/{}", symbol.name, label_name_string, symbol.namespace, offset);
        let string: String = symbol.name.clone();
        symbols_table_dump.push_str(&*string);
        symbols_table_dump.push_str("/");
        symbols_table_dump.push_str(&*label_name_string);
        symbols_table_dump.push_str("/");
        symbols_table_dump.push_str(&*symbol.namespace);
        symbols_table_dump.push_str("/");
        symbols_table_dump.push_str(&*offset.to_string());
        symbols_table_dump.push_str("\n");
    }
    dump("symbol_table.txt", symbols_table_dump);
    println!("\n");
    class_structure
}

fn parse_method(tokens: &Vec<Token>, start_index: usize, symbols_table: &mut SymbolTable, end_index: usize, current_namespace: String) -> (String, Vec<String>)
{
    //TODO parse if/else, while, for
    //TODO add bool for if expression
    let mut method_namespace = current_namespace.replace(".", "_").clone();
    let mut parameters: Vec<(String, String)> = Vec::new();
    let mut static_variables: Vec<(String, String, bool, SymbolClass)> = Vec::new();
    let mut current_static_method_variables = 0;
    let mut method_name = String::new();
    let mut method_return_type = String::new();
    let mut symbol_class: SymbolClass;
    let mut is_method_static = false;
    let mut step = start_index;

    if tokens[step].name == "mod.access"
    {
        is_method_static = true;
        step += 1;
    }
    if tokens[step].name == "final"
    {
        //TODO final parse_method
        panic!("in parse_method: currently not supporting final methods");
    }
    if tokens[step].name == "type"
    {
        method_return_type = tokens[step].value.clone();
        step += 1;
    }
    if tokens[step].name == "identifier"
    {
        method_name = tokens[step].value.clone();
        step += 1;
    }
    step += 1;

    let ending_parenthesis = identify_body_bounds(tokens, step, ("(", ")")).unwrap();
    let ending_brace = identify_body_bounds(tokens, ending_parenthesis + 2, ("{", "}")).unwrap();
    let mut skip_amount = 0;

    let mut index = step;
    //Get Parameters
    while index < ending_parenthesis
    {
        if tokens[index].name == "type" || tokens[index].name == "identifier"
        {
            let parameter_type = tokens[index].value.clone();
            index += 1;
            while tokens[index].name != "identifier"
            {
                index += 1;
            }

            let parameter_name = tokens[index].value.clone();

            parameters.push((parameter_name.clone(), tokens[index].value.clone()));
            current_static_method_variables += 1;
            index += 1;
        }
        else if tokens[index].value == ","
        {
            index += 1;
        }
        else if tokens[index].value == "["
        {
            panic!("Array not supported in method parameters!");
        }
        else
        {
            panic!("unexpected token! {}: {}", tokens[index].value, tokens[index].name);
        }

    }

    //Add Parameters
    for index in 0..parameters.len()
    {
        let ref parameter_name = parameters[index].0;
        let ref return_type = parameters[index].1;

        //TODO Equation for parameter offset
        let parameter_offset = (parameters.len() * 4 - (index * 4)) as u16;
        symbols_table.add(SymbolClass::Variable(return_type.clone()), method_namespace.clone(), parameter_name.clone(), false, true, true, 0, 0, parameter_offset);

    }

    //pass ) and {
    index += 2;

    //Parse body
    while index < ending_brace
    {
        if tokens[index].name == "type"
        {
            let semicolon = find_next(tokens, index, ";").unwrap();
            let (name, variable_type, is_static, symbol_class) = parse_variable(tokens, index);

            static_variables.push((name.clone(), variable_type.clone(), is_static, symbol_class));

            current_static_method_variables += 1;
            index = semicolon + 1;
        }
        else
        {
            index += 1;
        }

    }

    method_namespace.push_str("_");
    method_namespace.push_str(&*method_name.clone());
    //Add body variables
    for index in 0..static_variables.len()
    {
        let ref variable_name = static_variables[index].0;
        let ref return_type = static_variables[index].1;
        let ref is_variable_static = static_variables[index].2;

        //TODO Equation for parameter offset
        let variable_offset = (index) as u16;
        symbols_table.add(SymbolClass::Variable(return_type.clone()), method_namespace.clone(), variable_name.clone(), true, true, false, 0, variable_offset, 0);
    }

    let mut parameter_arguments: Vec<String> = Vec::new();
    let mut static_namespace = method_namespace.clone();
    static_namespace.push_str("_static");
    println!("Static method namespace: {}", static_namespace);

    for index in 0..parameters.len()
    {
        let string: String = parameters[index].1.clone();
        parameter_arguments.push(string);
    }

    //Add function symbol
    symbols_table.add(SymbolClass::Function(method_return_type.clone(), parameter_arguments.clone(), static_namespace.clone(), static_variables.len()), current_namespace.clone(), method_name.clone(), is_method_static, false, false, 0, (static_variables.len()) as u16, 0);
    (method_name, parameter_arguments)
}

fn parse_conditional_parameters(tokens: &Vec<Token>, start_index: usize, symbols_table: &mut SymbolTable, end_index: usize, current_namespace: String)
{

}

fn parse_variable<'a>(tokens: &Vec<Token>, start_index: usize) -> (String, String, bool, SymbolClass)
{
    let mut symbol_class: SymbolClass;
    let mut is_static: bool = false;
    let mut index = start_index;

    if tokens[index].name == "mod.access"
    {
        is_static = true;
        index += 1;
    }
    if tokens[index].value == "final"
    {
        index += 1;
    }
    let variable_type = tokens[index].value.clone();
    index += 1;
    let name = tokens[index].value.clone();

    symbol_class = SymbolClass::Variable(variable_type.clone());

    (name.clone(), variable_type.clone(), is_static, symbol_class)
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

    let invalid_types = get_invalid_token_types();
    let invalid_values = get_invalid_token_values();

    for (index, token) in tokens.iter().enumerate()
    {
        // Panic on invalid tokens
        if invalid_types.contains(&token.name) || invalid_values.contains(&&*token.value)
        {
            panic!("Unsupported token: {}: {}", token.value, token.name);
        }
        // Remove imports
        else if token.name == "special.import"
        {
            // invalidate import
            invalid_indecies.push(index);
            // invalidate the semi-colon after the semi-colon
            invalid_indecies.push(index + 1);
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

pub fn compile_oracle(args: &[&str]) -> bool
{
    println!("<----------------- Oracle Compiler --------------------->");
    let was_compile_successful = execute_process(args);

    if was_compile_successful
    {
        println!("Compile Successful!");
    }
    else
    {
        println!("Unable to compile, because code is not valid java.");
        println!("Please fix errors pointed out above.");

    }
    println!("\n");
    was_compile_successful
}
