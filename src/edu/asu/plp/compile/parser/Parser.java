package edu.asu.plp.compile.parser;

import java.util.List;

import edu.asu.plp.Token;

public class Parser
{
	private List<Token> tokens;
	private String tokenString;
	
	public Parser(List<Token> tokens)
	{
		this.tokens = tokens;
		this.tokenString = stringify(tokens);
	}
	
	private String stringify(List<Token> tokens)
	{
		StringBuilder stringBuilder = new StringBuilder();
		
		Token first = tokens.get(0);
		stringBuilder.append(stringify(first));
		
		for (int index = 1; index < tokens.size(); index++)
		{
			Token token = tokens.get(index);
			stringBuilder.append(" ");
			stringBuilder.append(stringify(token));
		}
		
		return stringBuilder.toString();
	}
	
	private String stringify(Token token)
	{
		// FIXME: this algorithm breaks if the token value contains spaces
		return token.getType().name() + "-" + token.getValue();
	}
	
	private void parsePackage()
	{
		// TODO
	}
	
	private void parseImports()
	{
		// TODO
	}
	
	private void parseClass()
	{
		
	}
	
	private void parseMethod()
	{
		String declarationRegex = "(TYPE-\\S*) (REFERNCE-\\S*) (CONTROL-;)";
		// TODO: support parenthesis scopes (e.g. var = a + (b * c) - (b + a))
		String assignmentRegex = "(REFERNCE-\\S*) (OPERATOR_BINARY-=) "
				+ "("
					+ "((REFERNCE-\\S*)|(LITERAL_\\S*))"
					+ "( (OPERATOR_BINARY-[^\\s=]*) ((REFERNCE-\\S*)|(LITERAL_\\S*)))*"
				+ ")" + "(CONTROL-;)";
		String declareAndAssignRegex = "(TYPE-\\S*) )" + assignmentRegex;
		String methodRegex = "(REFERNCE-\\S*) (CONTROL-\\()"
				+ "";
		
	}
	
	public static void main(String[] args)
	{
		
	}
	
}
