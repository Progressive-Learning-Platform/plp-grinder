use std::vec::Vec;
use regex::Regex;
use matching::*;
use files::*;
use tokens::*;
use support::*;

pub fn get_token_types<'a>() -> Vec<(&'a str, &'a str)>
{
    // TODO: Read types from rules file
    let mut token_types: Vec<(&str, &str)> = Vec::new();

    token_types.push(("comment.line",       	r"//[^\n]*"));
    token_types.push(("comment.block",      	r"/\*([^\*]|(\*[^/]))*\*/"));
    token_types.push(("control",            	r"\.|\(|\)|\{|}|\[|]|;|,"));
    token_types.push(("literal.int",        	r"(0(x|b|o|d))?[:digit:]+"));
    token_types.push(("literal.long",       	r"(0(x|b|o|d))?[:digit:]+[lL]"));
    token_types.push(("literal.float",      	r"(\.(\d+)(f|F)?)|((\d+)\.(\d+)(f|F)?)|((\d+)(f|F))"));
    token_types.push(("literal.double",     	r"[:digit:]*\.[:digit:]+"));
    token_types.push(("literal.char",       	r"'[a-z A-Z]?'"));
    token_types.push(("literal.string",     	"\"([^\"\n\r]|\\.)*\""));
    token_types.push(("literal.boolean",    	r"true|false"));
    token_types.push(("literal.null",       	r"null"));
    token_types.push(("block.generics.args",	r"<( )*([a-zA-Z],?( )*)+( )*>"));
    token_types.push(("operator.unary",     	r"(\+\+)|(--)"));
    token_types.push(("operator.binary",    	r"((\+|<<|>>|-|/|\*|\||&)=?)|="));
    token_types.push(("operator.comparator",	r">|>=|<|<=|&&|\|\||==|instanceof"));
    token_types.push(("type",               	r"boolean|long|int|byte|short|char|double|float|void"));
    token_types.push(("mod.permission",     	r"public|private|protected"));
    token_types.push(("mod.access",    			r"static"));
    token_types.push(("mod.behaviour", 			r"final|volitile|transient|synchronized|native|abstract|throws"));
    token_types.push(("mod.inheritance",		r"extends|implements"));
    token_types.push(("action",             	r"return|continue|break|throw|new|assert|strictfp"));
    token_types.push(("construct.conditional", 	r"if|else"));
    token_types.push(("construct.handles",		r"try|catch|finally"));
    token_types.push(("construct.switch", 		r"switch|case:|default:"));
    token_types.push(("construct.loop",		 	r"do|while|for"));
    token_types.push(("construct.type", 		r"class|interface|enum"));
    token_types.push(("special.package",      	r"package"));
    token_types.push(("special.reserved",   	r"goto|const"));
    token_types.push(("special.import",     	r"import( +static)?( )+([a-zA-Z\._])+\*?"));
    token_types.push(("identifier",				r"([:alpha:]|[\$_])+([:alnum:]|[\$_])*"));

    token_types
}

pub fn get_invalid_token_values<'a>() -> Vec<&'a str>
{
    // TODO: Read invalid types from rules file
    let mut invalid_tokens: Vec<&str> = Vec::new();

    invalid_tokens.push("enum");
    invalid_tokens.push("interface");

    invalid_tokens
}

pub fn get_invalid_token_types<'a>() -> Vec<&'a str>
{
    // TODO: Read invalid types from rules file
    let mut invalid_types: Vec<&str> = Vec::new();

    invalid_types.push("literal.null");
    invalid_types.push("block.generics.args");
    invalid_types.push("literal.string");
    invalid_types.push("literal.char");
    invalid_types.push("literal.float");
    invalid_types.push("literal.double");
    invalid_types.push("special.reserved");
    invalid_types.push("construct.conditional");
    invalid_types.push("construct.handles");
    invalid_types.push("construct.switch");
    invalid_types.push("construct.loop");

    invalid_types
}

pub fn lex_file<'a>(file_path: &str) -> Vec<Token<'a>>
{
    let input = read_in(file_path);
    lex_string(input)
}

pub fn lex_string<'a>(input: String) -> Vec<Token<'a>>
{
    // List the first match from each matcher; the index of the match will correspond to an index in matchers
    let mut matches: Vec<MatchResult> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();

    let token_types: Vec<(&str, &str)> = get_token_types();
    let matchers: Vec<Regex> = assemble_matchers(&token_types);

    let mut string_index = 0;
    while string_index < input.len()
    {
        let string = &input[string_index..];
        for (index, matcher) in matchers.iter().enumerate()
        {
            let token_name = token_types[index].0;
            let result = matcher.find(string);
            let match_result = MatchResult::parse(token_name, result, index);
            matches.push(match_result);
        }

        // print matches
        println!("\nMatches on {}:", string);
        for result in matches.iter()
        {
            println!("\t{}\t({}, {})\t{}", result.token_name, result.start, result.end, result.valid);
        }

        // Find the first match and add it as a token; stop when there are no tokens to be found
        let (valid, index) = find_first_match(&matches);
        if valid
        {
            {
                let match_result = &matches[index];
                let start = match_result.start + string_index;
                let end = match_result.end + string_index;
                let value = slice_of(&input, (start, end));
                let token = Token{name: match_result.token_name, range: (start, end), value: value};
                string_index = end;
                println!("\tThe dominant token is: {} at index: {} = {} with value: {}", matches[index].token_name, index, matches[index].index, token.value);
                println!("\tRange: ({}, {})", start, end);
                tokens.push(token);
            }
            matches.clear();
        }
        else
        {
            println!("no matches found");
            break;
        }
    }

    tokens
}
