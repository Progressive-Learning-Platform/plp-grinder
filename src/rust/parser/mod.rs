mod tree;
use std::collections::HashMap;
use parser::tree::ParseTree;

pub struct BodyParser<'a>
{
	pub variables: HashMap<Variable<'a>, &'a str>,
	pub body: ParseTree<'a>,
}

impl<'a> BodyParser<'a>
{
	fn parse() -> Option<ParseResult<'a>>
	{
		None
	}
}

pub struct ParseResult<'a>
{
	end_index: usize,
	tree: ParseTree<'a>,
}

pub struct Variable<'a>
{
	pub name: &'a str,
	pub vtype: &'a str,
}
