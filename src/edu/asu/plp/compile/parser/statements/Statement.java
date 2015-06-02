package edu.asu.plp.compile.parser.statements;

public class Statement
{
	public enum StatementType
	{
		ASSIGNMENT,
		ACTION,
		FOR,
		IF,
		GOTO;
	}
	
	public Statement()
	{
		// TODO Auto-generated constructor stub
	}
	
	private StatementType type;
	private AssignmentStatement assignStatement;
	private ActionStatement actionStatement; 
	
}
