package edu.asu.plp.compile.parser.statements;

import edu.asu.plp.compile.parser.Variable;

public class AssignmentStatement
{
	private Variable leftHandSide;
	private String operator;
	private Expression rightHandSide;
	
	public AssignmentStatement(Variable leftHandSide, String operator, Expression rightHandSide)
	{
		this.leftHandSide = leftHandSide;
		this.operator = operator;
		this.rightHandSide = rightHandSide;
	}

	public Variable getLeftHandSide()
	{
		return leftHandSide;
	}

	public String getOperator()
	{
		return operator;
	}

	public Expression getRightHandSide()
	{
		return rightHandSide;
	}

	public void setLeftHandSide(Variable leftHandSide)
	{
		this.leftHandSide = leftHandSide;
	}

	public void setOperator(String operator)
	{
		this.operator = operator;
	}

	public void setRightHandSide(Expression rightHandSide)
	{
		this.rightHandSide = rightHandSide;
	}
}
