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
mod parser;

use std::env;
use std::process;
use std::vec::Vec;
use tokens::*;
use parser::*;
use symbols::*;
use symbols::symbol_table::*;
use compiler::symbol_analysis::*;
use lexer::*;
use support::*;
use files::*;
use compiler::*;
use plp::PLPWriter;

fn main()
{
    let (source_files, output_directory, base_writter) = parse_command_arguments();

    // TODO: support multiple source files
    let source_file = &*source_files[0].clone();
    let was_compile_successful = compile_oracle(&["javac", source_file]);

    if was_compile_successful
    {
        let mut static_init_labels: Vec<String> = Vec::new();
        let mut symbols_table: SymbolTable = SymbolTable::new();
        let mut structures: Vec<(Vec<Token>, ClassStructure, String)> = Vec::new();

        // Parse all classes
        for source_file in source_files
        {
            let tokens = lex(source_file);
            //TODO search for package declaration.
            let starting_point = find_next(&tokens, 0, "{").unwrap() + 1;
            let class_structure = parse_class(&tokens, starting_point, tokens[starting_point - 2].value.clone(), "".to_string(), true, &mut symbols_table, output_directory.clone());
            // TODO: calculate actual file name
            let file_name = "output".to_string();

            // Print lexed output
            let mut lex_output_file = output_directory.clone();
            lex_output_file.push_str(&*file_name.clone());
            lex_output_file.push_str(".lexed");
            tokens.print_to(&*lex_output_file, false);

            structures.push((tokens, class_structure, file_name));
        }

        // Compile and output all classes
        for structure in structures
        {
            let ref tokens = structure.0;
            let ref class_structure = structure.1;
            let ref file_name = structure.2;

            let (code, static_init_label) = compile(&tokens, &class_structure, &symbols_table, &base_writter);
            let mut plp = PLPWriter::new();
            plp.code.push_str(&*code);
            dump(&*(output_directory.clone() + &*file_name.clone() + ".asm"), plp.code);
            static_init_labels.push(static_init_label.clone());
        }

        // Compile starting file
        let mut plp = PLPWriter::new();
        let main_symbol = symbols_table.lookup_by_name("main")[0];
        let main_label = match main_symbol.location
        {
            SymbolLocation::Memory(ref address) => address.label_name.clone(),
            _ => { panic!("Main found was not a function!"); },
        };

        compile_program_header(&mut plp, &*main_label, &static_init_labels);
        dump(&*(output_directory.clone() + "main.asm"), plp.code.clone());
    }
}

/// Parse the command line arguments, and determine all source files to compile, based on the arguments and defaults
/// @return a Vector of all source files to be compiled. Each element represents the relative path to one file
/// @return the relative path to the desired output directory
/// @return a base_writter specifying the settings of the PLPWriter
fn parse_command_arguments() -> (Vec<String>, String, PLPWriter)
{
    let default_output_directory = "output/";
    let default_source = "sampleData/BasicArithmatic.java";

    let mut opts = getopts::Options::new();
    opts.optopt("s", "src", "Set input file path", "PATH");
    opts.optopt("d", "dest", "Sets root output directory of all files written to", "PATH");
    opts.optopt("i", "source_folder", "Sets root input directory of all source files to read", "PATH");
    opts.optflag("a", "annotate", "Enables annotation of output source file");
    opts.optflag("m", "map", "Enables mapping of line numbers from Java source to output asm source");
    opts.optflag("h", "help", "Prints usage of options");

    let args: Vec<String> = env::args().collect();
    let matches = match opts.parse(&args[1..])
    {
        Ok(m) => m,
        Err(f) => {
                println!("{}", f);
                process::exit(1);
            }
    };

    if matches.opt_present("h")
    {
        let brief = format!("Usage: {} [options]", args[0]);
        println!("{}", opts.usage(&brief));
        process::exit(0);
    }

    let mut source_file = match matches.opt_str("s")
    {
        Some(ref file_path) => file_path.clone(),
        None => default_source.to_string(),
    };

    let mut output_directory = match matches.opt_str("d")
    {
        Some(ref directory_path) => directory_path.clone() + "/",
        None => default_output_directory.to_string(),
    };

    let mut input_directory = match matches.opt_str("i")
    {
        Some(ref directory_path) => directory_path.clone() + "/",
        None => String::new(),
    };

    //TODO match options
    if !matches.free.is_empty() {
        println!("Free arguments: {:?}", matches.free);
    }

    let mut base_writter = PLPWriter::new();
    base_writter.annotations_enabled = matches.opt_present("a");
    base_writter.mapping_enabled = matches.opt_present("m");

    let mut files = Vec::new();
    files.push(source_file.clone());

    (files, output_directory, base_writter)
}

fn lex<'a>(source_file: String) -> Vec<Token<'a>>
{
    let mut tokens: Vec<Token> = lex_file(&*source_file, false);
    remove_meta(&mut tokens);

    tokens
}

/// @return (code, static_init_label)
fn compile(tokens: &Vec<Token>, class_structure: &ClassStructure, symbols_table: &SymbolTable, base_writter: &PLPWriter) -> (String, String)
{
    let class_symbol = &class_structure.class_symbol;

    let mut plp = base_writter.copy();
    let (static_memory_label, static_init_label, local_init_label) = get_class_labels(&class_symbol);

    // Static class memory
    let static_size = class_structure.static_variables.len();
    plp.annotate("=============== Static Class Memory =================");
    plp.label(&*static_memory_label);
    plp.indent_level += 1;
    plp.space(static_size as u16);
    plp.indent_level -= 1;
    plp.annotate("============= END Static Class Memory ===============");

    // Compile static_init for class
    plp.println();
    plp.annotate("================ Static Init Block =================");
    plp.label(&*static_init_label);
    plp.indent_level += 1;
    for index in 0..class_structure.static_variables.len()
    {
        let ref static_variable = class_structure.static_variables[index];
        let start = static_variable.0;
        let name = static_variable.2.clone();
        let namespace = static_variable.3.clone();

        let registers = ("$t0", "$t1", "$t2", "$t3", "$t4");
        compile_statement(&tokens, start, &*namespace, registers, symbols_table, &mut plp);
    }
    plp.ret();
    plp.indent_level -= 1;
    plp.annotate("============== END Static Init Block ===============");
    // TODO: handle static init blocks ("static { ...[logic]... }")

    // Compile local_init for class
    plp.println();
    plp.annotate("================ Local Init Block ==================");
    plp.label(&*local_init_label);
    plp.indent_level += 1;
    for index in 0..class_structure.non_static_variables.len()
    {
        let ref local_variable = class_structure.non_static_variables[index];
        let start = local_variable.0;
        let name = local_variable.2.clone();
        let namespace = local_variable.3.clone();

        let registers = ("$t0", "$t1", "$t2", "$t3", "$t4");
        compile_statement(&tokens, start, &*namespace, registers, symbols_table, &mut plp);
    }
    plp.ret();
    plp.indent_level -= 1;
    plp.annotate("=============== END Local Init Block ===============");

    // TODO: compile constructors

    // Compile local methods
    plp.annotate("================== Local Methods ===================");
    for index in 0..class_structure.non_static_methods.len()
    {
        let ref local_method = class_structure.non_static_methods[index];
        let range = (local_method.0, local_method.1);
        let name = local_method.2.clone();
        let namespace = local_method.3.clone();
        let argument_types = local_method.4.clone().unwrap();

        let method_symbol = symbols_table.lookup_function(&*namespace, &*name, &argument_types).unwrap();

        let registers = ("$t0", "$t1", "$t2", "$t3", "$t4");
        compile_method_body(&tokens, range, method_symbol, &*namespace, registers, symbols_table, &mut plp);
    }
    plp.annotate("================ END Local Methods =================");

    // Compile static methods
    plp.annotate("================== Static Methods ==================");
    for index in 0..class_structure.static_methods.len()
    {
        let ref static_method = class_structure.static_methods[index];
        let range = (static_method.0, static_method.1);
        let name = static_method.2.clone();
        let namespace = static_method.3.clone();
        let argument_types = static_method.4.clone().unwrap();

        let method_symbol = symbols_table.lookup_function(&*namespace, &*name, &argument_types).unwrap();

        let registers = ("$t0", "$t1", "$t2", "$t3", "$t4");
        compile_method_body(&tokens, range, method_symbol, &*namespace, registers, symbols_table, &mut plp);
    }
    plp.annotate("================ END Static Methods ================");

    // Compile nested classes
    for index in 0..class_structure.static_classes.len()
    {
        let ref class = class_structure.static_classes[index];

        plp.println();
        plp.indent_level += 1;
        plp.annotate("========== Begin Nested Class Definition ===========");
        // TODO: support nested static init blocks
        let (code, static_init_label) = compile(tokens, class, symbols_table, &plp);
        plp.code.push_str(&*code);
        plp.annotate("=========== End Nested Class Definition ============");
        plp.indent_level -= 1;

    }
    plp.annotate("================ END Static Methods ================");

    // TODO: compile non-static classes

    (plp.code, static_init_label)
}

///Start on open curly brace
fn parse_class(tokens: &Vec<Token>, start_index: usize, class_name: String, namespace: String, is_class_static: bool, symbols_table: &mut SymbolTable, output_directory: String) -> ClassStructure
{
    let mut class_structure: ClassStructure = ClassStructure::new();
    let mut current_namespace: String = String::new();
    if !namespace.is_empty()
    {
        current_namespace.push_str(&*namespace.clone());
        current_namespace.push_str(".");
    }
    current_namespace.push_str(&*class_name.clone());

    let mut current_local_class_variables = 0;
    let mut current_static_class_variables = 0;

    //println!("\n<------------ Parse Class --------------->");
    let mut min_value = 0;
    let mut skip_amount = 0;
    let mut tokens_index;

    for (index, token) in tokens[start_index..].iter().enumerate()
    {
        tokens_index = index + start_index;
        if min_value < 0
        {
            min_value = 0;
        }
        if min_value != 0
        {
            min_value -= 1;
            continue;
        }
        else if tokens[tokens_index].value == "}"
        {
            break;
        }
        else if tokens[tokens_index].value == "final"
        {
            panic!("Unsupported or Unexpected token: {} + {}.", tokens[tokens_index].value, tokens[tokens_index].name);
        }
        //Static Variable/Method/Class
        else if token.name == "mod.access"
        {
            skip_amount = 3;
            if tokens[tokens_index + 1].value == "final"
            {
                panic!("Unsupported or Unexpected token: {} + {}.", tokens[tokens_index + 1].value, tokens[tokens_index + 1].name);
            }
            else if tokens[tokens_index + skip_amount].name.starts_with("operator") || tokens[tokens_index + skip_amount].value == ";"
            {
                //println!("------Incoming Static Variable Decl!");

                let low = tokens_index + 1;
                let high = find_next(tokens, low, ";").unwrap() + 1;

                let (name, variable_type, is_static, symbol_class) = parse_variable(tokens, tokens_index);
                symbols_table.add(symbol_class, current_namespace.clone(), name.clone(), is_static, false, false, current_local_class_variables, current_static_class_variables, 0);
                current_static_class_variables += 1;

                class_structure.static_variables.push(MemberBlock (low, high, name.clone(), current_namespace.clone(), None));

                min_value =  high;
                min_value -= tokens_index + 1;
            }
            else if tokens[tokens_index + 1].name.starts_with("construct")
            {
                if tokens[tokens_index + skip_amount].name.starts_with("control")
                {
                    //println!("------Incoming Static Class Decl!");
                    let starting_point = find_next(tokens, tokens_index, "{").unwrap() + 1;
                    min_value = identify_body_bounds(tokens, starting_point, ("{", "}")).unwrap() + 1;

                    class_structure.static_classes.push(parse_class(tokens, starting_point, tokens[tokens_index + 2].value.clone(), current_namespace.clone(), true, symbols_table, output_directory.clone()));

                    min_value -= tokens_index + 1;
                }
                else
                {
                    panic!("Unsupported or Unexpected token: {} + {}.", tokens[tokens_index + skip_amount].value, tokens[tokens_index + skip_amount].name);
                }
            }
            else if tokens[tokens_index + skip_amount].name.starts_with("control")
            {
                //println!("------Incoming Static Method Decl!");
                let starting_point = find_next(tokens, tokens_index, "{").unwrap() + 1;
                min_value = identify_body_bounds(tokens, starting_point, ("{", "}")).unwrap() + 1;

                let (method_name, argument_types) = parse_method(tokens, tokens_index, symbols_table, min_value, current_namespace.clone());

                class_structure.static_methods.push(MemberBlock (starting_point - 1, min_value, method_name.clone(), current_namespace.clone(), Some(argument_types)));
                min_value -=  tokens_index + 1;
            }
            else
            {
                panic!("Unsupported or Unexpected token: {}: {}.", tokens[tokens_index + skip_amount].value, tokens[tokens_index + skip_amount].name);
            }

        }
        //Non-Static Variable/Method/Class
        else if token.name.starts_with("construct")
        {
            skip_amount = 2;
            if tokens[tokens_index + skip_amount].name.starts_with("control")
            {
                //println!("------Incoming Non-Static Class Decl!");
                let starting_point = find_next(tokens, tokens_index, "{").unwrap() + 1;
                min_value = identify_body_bounds(tokens, starting_point, ("{", "}")).unwrap() + 1;

                class_structure.non_static_classes.push(parse_class(tokens, starting_point, tokens[tokens_index + 1].value.clone(), current_namespace.clone(), false, symbols_table, output_directory.clone()));

                min_value -= tokens_index + 1;
            }
            else
            {
                panic!("Unsupported or Unexpected token: {} + {}.", tokens[tokens_index + skip_amount].value, tokens[tokens_index + skip_amount].name);
            }
        }
        //Non-Static Variable/Method/Class
        else if token.name == "type" || token.name == "identifier"
        {
            skip_amount = 2;

            if tokens[tokens_index + skip_amount].name.starts_with("operator") || tokens[tokens_index + skip_amount].value == ";"
            {
                //println!("------Incoming Non-Static Variable Decl!");
                min_value =  find_next(tokens, tokens_index, ";").unwrap() + 1;

                let (name, variable_type, is_static, symbol_class) = parse_variable(tokens, tokens_index);
                symbols_table.add(symbol_class, current_namespace.clone(), name.clone(), is_static, false, false, current_local_class_variables, current_static_class_variables, 0);
                current_local_class_variables += 1;

                class_structure.non_static_variables.push(MemberBlock (tokens_index, min_value, name.clone(), current_namespace.clone(), None));

                min_value -= tokens_index + 1;
            }
            else if tokens[tokens_index + skip_amount].name.starts_with("control")
            {
                //println!("------Incoming Non-Static Method Decl!");
                let starting_point = find_next(tokens, tokens_index, "{").unwrap() + 1;
                min_value = identify_body_bounds(tokens, starting_point, ("{", "}")).unwrap() + 1;

                let (method_name, argument_types) = parse_method(tokens, tokens_index, symbols_table, min_value, current_namespace.clone());

                class_structure.non_static_methods.push(MemberBlock (starting_point - 1, min_value, method_name.clone(), current_namespace.clone(), Some(argument_types)));
                min_value -= tokens_index + 1;
                //check for control
            }
        }
        //println!("\tIndex: {} | Token -> {} : {}", tokens_index, token.value, token.name );
        //deal with parameters
    }
    class_structure.class_symbol = Symbol {namespace: namespace.clone(), is_static: is_class_static, name: class_name.clone(), symbol_class: SymbolClass::Structure("class".to_string(), current_local_class_variables as usize), location: SymbolLocation::Structured};
    symbols_table.add(SymbolClass::Structure("class".to_string(), current_local_class_variables as usize), namespace.clone(), class_name.clone(), is_class_static, false, false, 0, 0, 0);
    //println!("\n\n=============================================================");
    //println!("<                 {} Overview                  >\n", class_name);
    /*
    println!("<                 Class Structure Overview                  >");
    println!("Class Symbol: name {}, namespace {}, is_static {}\n", class_structure.class_symbol.name, class_structure.class_symbol.namespace, class_structure.class_symbol.is_static);
    println!("<---------------- Static Variables --------------->");
    for member_block in class_structure.static_variables.iter()
    {
        println!("Start/End {}/{}: {}", member_block.0, member_block.1, member_block.2);
    }

    println!("\n<---------------- Static Classes ----------------->");
    for inner_class_structure in class_structure.static_classes.iter()
    {
        println!("Static Class: {}/{}", inner_class_structure.class_symbol.name, inner_class_structure.class_symbol.namespace);
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
    for inner_class_structure in class_structure.non_static_classes.iter()
    {
        println!("Non-Static Class: {}/{}", inner_class_structure.class_symbol.name, inner_class_structure.class_symbol.namespace);
    }

    println!("\n<---------------- Non-Static Methods ----------------->");
    for member_block in class_structure.non_static_methods.iter()
    {
        println!("Start/End {}/{}: {}", member_block.0, member_block.1, member_block.2);
    }
    */
    //println!("\n");

    let mut symbols_table_dump: String = String::new();
    let mut formatted_symbol_vec: Vec<(String, String, String, String)> = Vec::new();

    //println!("<                    Symbol Table Overview                     >");
    for symbol in symbols_table.children_scopes.iter()
    {
        let mut location_and_class: String = String::new();
        match symbol.location
            {
                SymbolLocation::Memory(ref memory_address) =>
                {
                    location_and_class.push_str("Memory(");
                    location_and_class.push_str("Label_name: ");
                    location_and_class.push_str(&*memory_address.label_name.clone());
                    location_and_class.push_str("/");
                    location_and_class.push_str("Offset: ");
                    location_and_class.push_str(&*memory_address.offset.to_string());
                    location_and_class.push_str(")");
                },
                SymbolLocation::MethodArgument(offset) =>
                {
                    location_and_class.push_str("MethodArgument(");
                    location_and_class.push_str(&*offset.to_string());
                    location_and_class.push_str(")");
                },
                SymbolLocation::InstancedMemory(offset) =>
                {
                    location_and_class.push_str("InstancedMemory(");
                    location_and_class.push_str(&*offset.to_string());
                    location_and_class.push_str(")");
                },
                SymbolLocation::Structured =>
                {
                    location_and_class.push_str("SymbolLocation::Structured");
                },
                _ =>
                {
                    location_and_class.push_str("No match on location");
                },
            }
        location_and_class.push_str(" | ");
        match symbol.symbol_class
            {
                SymbolClass::Variable(ref variable_type) =>
                {
                    location_and_class.push_str("Class::Variable(");
                    location_and_class.push_str(&*variable_type.clone());
                    location_and_class.push_str(")");
                },
                SymbolClass::Function(ref return_type, _, ref static_memory_label, ref static_memory_size) =>
                {
                    location_and_class.push_str("Class::Function(");
                    location_and_class.push_str(&*return_type.clone());
                    location_and_class.push_str(", ");
                    location_and_class.push_str("_");
                    location_and_class.push_str(", ");
                    location_and_class.push_str(&*static_memory_label.clone());
                    location_and_class.push_str(", ");
                    location_and_class.push_str(&*static_memory_size.to_string());
                    location_and_class.push_str(")");
                },
                SymbolClass::Structure(ref structure_type, ref memory_size) =>
                {
                    location_and_class.push_str("Class::Structure(");
                    location_and_class.push_str(&*structure_type.clone());
                    location_and_class.push_str(", ");
                    location_and_class.push_str(&*memory_size.to_string().clone());
                    location_and_class.push_str(")");
                }
            };
        formatted_symbol_vec.push(("name: ".to_string() + &*symbol.name.clone(), "namespace: ".to_string() + &*symbol.namespace.clone(), "is_static: ".to_string() + &*symbol.is_static.to_string(), location_and_class.clone()));
    }
    formatted_symbol_vec.print_to(&*(output_directory + "symbol_table.txt"), false);
    //symbols_table_dump.push_str(&*formatted_symbol.clone());
    //symbols_table_dump.push_str("\n");
    //dump(&*(output_directory + "symbol_table.txt"), symbols_table_dump);
    //println!("\n");

    //TODO add class to symbols table
    class_structure
}

fn parse_method(tokens: &Vec<Token>, start_index: usize, symbols_table: &mut SymbolTable, end_index: usize, current_namespace: String) -> (String, Vec<String>)
{
    let mut method_namespace = current_namespace.clone();
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
    if tokens[step].name == "type" || tokens[step].name == "identifier"
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

            parameters.push((parameter_name.clone(), parameter_type));
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
        else if tokens[index].name == "identifier" && tokens[index + 1].name == "identifier"
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

    method_namespace.push_str(".");
    method_namespace.push_str(&*method_name.clone());
    //Add body variables
    for index in 0..static_variables.len()
    {
        let ref variable_name = static_variables[index].0;
        let ref return_type = static_variables[index].1;
        let ref is_variable_static = static_variables[index].2;

        //TODO Equation for parameter offset
        let variable_offset = (index) as u16;
        let occurs: bool = match symbols_table.lookup_variable(&*method_namespace.clone(), &*variable_name.clone())
        {
            Some(ref returned_variable) => true,
            None => false,
        };
        if !occurs
        {
            symbols_table.add(SymbolClass::Variable(return_type.clone()), method_namespace.clone(), variable_name.clone(), true, true, false, 0, variable_offset, 0);
        }
    }

    let mut parameter_arguments: Vec<String> = Vec::new();
    let mut static_namespace = method_namespace.clone();
    static_namespace.push_str("_static");
    //println!("Static method namespace: {}", static_namespace);

    for index in 0..parameters.len()
    {
        let string: String = parameters[index].1.clone();
        parameter_arguments.push(string);
    }

    //Add function symbol
    symbols_table.add(SymbolClass::Function(method_return_type.clone(), parameter_arguments.clone(), static_namespace.clone().replace(".", "_"), static_variables.len()), current_namespace.clone(), method_name.clone(), is_method_static, false, false, 0, (static_variables.len()) as u16, 0);
    (method_name, parameter_arguments)
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
        delete_file(&*args[args.len() - 1].replace(".java", ".class"));
    }
    else
    {
        println!("Unable to compile, because code is not valid java.");
        println!("Please fix errors pointed out above.");

    }
    println!("\n");
    was_compile_successful
}
