package edu.asu.plp.compile.parser.statements;


public class Statement
{
	public enum StatementType
	{
		ASSIGNMENT,
		ACTION,
		FOR,
		IF,
		GOTO,
		LINK_LIST;
	}
	
	public Statement(StatementType type)
	{
		this.type = type;
	}
	
	private StatementType type;
	private AssignmentStatement assignStatement;
	private ActionStatement actionStatement; 
	
}
