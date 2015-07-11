package edu.asu.plp.compile.parser.tree;

import java.util.LinkedList;
import java.util.List;

public class DeclarationNode implements ParseNode
{
	private List<ParseNode> children;
	
	public DeclarationNode()
	{
		super();
		children = new LinkedList<ParseNode>();
	}

	@Override
	public NodeType getType()
	{
		return NodeType.DECLARATION;
	}

	@Override
	public String getValue()
	{
		return "";
	}

	@Override
	public List<ParseNode> getChildren()
	{
		return children;
	}
	
}
