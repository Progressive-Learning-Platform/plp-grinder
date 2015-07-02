use tokens::*;

pub struct ParseTree<'a>
{
	pub root: Node<'a>,
}

impl<'a> ParseTree<'a>
{

}

pub struct Node<'a>
{
	pub token: Token<'a>,
	pub children: Vec<Node<'a>>
}

impl<'a> Node<'a>
{
	fn new(token: Token<'a>) -> Node<'a>
	{
		Node
		{
			token: token,
			children: Vec::new()
		}
	}

	fn has_children(&self) -> bool
	{
		self.children.len() > 0
	}

	fn add_child(&mut self, child: Node<'a>)
	{
		self.children.push(child);
	}

	fn value(&self) -> String
	{
		self.token.value.clone()
	}
}
