package edu.asu.plp.compile.parser.statements.components;

public class Expression
{
	private ExpressionComponent rightHandSide;
	
	public Expression(ExpressionComponent rightHandSide)
	{
		this.rightHandSide = rightHandSide;
	}

	public ExpressionComponent getRightHandSide()
	{
		return rightHandSide;
	}
}
