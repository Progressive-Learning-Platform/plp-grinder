package edu.asu.plp.compile.parser.tree;

import java.util.Collections;
import java.util.List;

public class ValueNode implements ParseNode
{
	private String value;
	
	public ValueNode(String value)
	{
		super();
		this.value = value;
	}
	
	@Override
	public NodeType getType()
	{
		return NodeType.VALUE;
	}
	
	@Override
	public String getValue()
	{
		return value;
	}
	
	@Override
	public List<ParseNode> getChildren()
	{
		return Collections.<ParseNode> emptyList();
	}
	
}
