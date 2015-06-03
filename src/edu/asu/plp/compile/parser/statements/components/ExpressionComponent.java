package edu.asu.plp.compile.parser.statements.components;

public class ExpressionComponent
{
	private ExpressionComponent group;
	private ValueNode value;
	private boolean isGroup;
	
	public ExpressionComponent(ExpressionComponent group)
	{
		this(group, null, true);
	}
	
	public ExpressionComponent(ValueNode value)
	{
		this(null, value, false);
	}
	
	private ExpressionComponent(ExpressionComponent group, ValueNode value, boolean isGroup)
	{
		this.group = group;
		this.value = value;
		this.isGroup = isGroup;
	}
	
}
