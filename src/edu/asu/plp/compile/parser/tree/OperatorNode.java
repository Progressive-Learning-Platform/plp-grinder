package edu.asu.plp.compile.parser.tree;

import java.util.LinkedList;
import java.util.List;

public class OperatorNode implements ParseNode
{
	private String value;
	private List<ParseNode> statements;
	
	public OperatorNode(String value)
	{
		super();
		this.value = value;
		statements = new LinkedList<ParseNode>();
	}
	
	@Override
	public NodeType getType()
	{
		return NodeType.OPERATOR;
	}
	
	@Override
	public String getValue()
	{
		return value;
	}
	
	@Override
	public List<ParseNode> getChildren()
	{
		return statements;
	}
}
