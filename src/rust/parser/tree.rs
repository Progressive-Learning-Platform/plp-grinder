use tokens::*;

pub struct Tree<'a>
{
	pub root: Node<'a>,
}

pub struct Node<'a>
{
	pub token: Token<'a>,
	pub children: Vec<Node<'a>>
}

impl<'a> Node<'a>
{
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
