package edu.asu.plp.compile.parser.statements.components;

public class ExpressionComponent
{
	private ExpressionComponent leftGroup;
	private String operator;
	private ExpressionComponent rightGroup;
	
	private ValueNode value;
	private boolean isGroup;
	
	public ExpressionComponent(ExpressionComponent leftGroup, ExpressionComponent rightGroup, String operator)
	{
		this(leftGroup, rightGroup, operator, null, true);
	}
	
	public ExpressionComponent(ValueNode value)
	{
		this(null, null, null, value, false);
	}
	
	private ExpressionComponent(ExpressionComponent leftGroup, ExpressionComponent rightGroup, String operator, ValueNode value, boolean isGroup)
	{
		this.leftGroup = leftGroup;
		this.operator = operator;
		this.rightGroup = rightGroup;
		this.value = value;
		this.isGroup = isGroup;
	}

	public ExpressionComponent getLeftGroup()
	{
		return leftGroup;
	}

	public String getOperator()
	{
		return operator;
	}

	public ExpressionComponent getRightGroup()
	{
		return rightGroup;
	}

	public ValueNode getValue()
	{
		return value;
	}

	public boolean isGroup()
	{
		return isGroup;
	}
	
	
	
}
