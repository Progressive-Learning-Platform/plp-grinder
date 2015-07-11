package edu.asu.plp.compile.parser.tree;

import java.util.LinkedList;
import java.util.List;

public class SequenceNode implements ParseNode
{
	private List<ParseNode> statements;
	
	public SequenceNode()
	{
		super();
		statements = new LinkedList<ParseNode>();
	}
	
	@Override
	public NodeType getType()
	{
		return NodeType.SEQUENCE;
	}
	
	@Override
	public String getValue()
	{
		return "";
	}
	
	@Override
	public List<ParseNode> getChildren()
	{
		return statements;
	}
	
}
