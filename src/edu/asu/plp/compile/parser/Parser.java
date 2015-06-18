package edu.asu.plp.compile.parser;

import java.io.FileNotFoundException;
import java.util.LinkedList;
import java.util.List;

import edu.asu.plp.Token;
import edu.asu.plp.Token.Type;
import edu.asu.plp.compile.lex.LexException;
import edu.asu.plp.compile.parser.constructs.ClassConstruct;
import edu.asu.plp.compile.parser.support.TreeSupport;
import edu.asu.plp.compile.parser.tree.DeclarationNode;
import edu.asu.plp.compile.parser.tree.ParseNode;
import edu.asu.plp.compile.parser.tree.SequenceNode;
import edu.asu.plp.compile.parser.tree.ValueNode;

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
	
	//TODO Change public to private. Testing purposes
	public SequenceNode parseArithmeticStatement(List<Token> sequenceTokens) throws ParseException
	{	
		ParseExpression tree = new ParseExpression(sequenceTokens);
		
		return tree.parse();
	}

	//TODO Change public to private. Testing purposes
	public SequenceNode parseDeclaration(LinkedList<Token> declarationList) throws ParseException
	{
		SequenceNode declaration = new SequenceNode();
		
		List<ParseNode> declarationNodes = declaration.getChildren();
		
		int starter = 0;
		ValueNode originalType = new ValueNode(declarationList.get(starter++).getValue());
		
		for(int index = starter; index < declarationList.size(); index++)
		{
			if(declarationList.get(index).getType() == Type.REFERNCE)
			{
				DeclarationNode declarationNode = new DeclarationNode();
				List<ParseNode> children = declarationNode.getChildren();
				TreeSupport.insertChildAt(children, 0, originalType);
				TreeSupport.insertChildAt(children, 1, new ValueNode(declarationList.get(index).getValue()));
				declarationNodes.add(declarationNode);
			}
		}
		return declaration;
	}
	
	public static void main(String[] args) throws FileNotFoundException, LexException, ParseException
	{
	}
	
}
