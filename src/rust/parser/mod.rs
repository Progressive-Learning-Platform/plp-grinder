mod tree;

pub fn get_start_symbols<'a>() -> Vec<&'a str>
{
	// TODO: Read symbols from rules file
    let mut start_symbols: Vec<&str> = Vec::new();

	start_symbols.push("");

	start_symbols
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
