package edu.asu.plp.compile.compiler;

import java.util.LinkedList;
import java.util.List;
import java.util.ListIterator;

import edu.asu.plp.Token;
import edu.asu.plp.Token.Type;
import edu.asu.plp.compile.parser.ParseException;
import edu.asu.plp.compile.parser.Parser;
import edu.asu.plp.compile.parser.constructs.ClassConstruct;
import edu.asu.plp.compile.parser.constructs.MethodConstruct;
import edu.asu.plp.compile.parser.constructs.Signature;
import edu.asu.plp.compile.parser.constructs.Variable;
import edu.asu.plp.compile.parser.support.TreeSupport;
import edu.asu.plp.compile.parser.tree.OperatorNode;
import edu.asu.plp.compile.parser.tree.ParseNode;
import edu.asu.plp.compile.parser.tree.SequenceNode;
import edu.asu.plp.compile.parser.tree.ValueNode;

public class ParseClass
{
	ClassConstruct classConstruct;
	LinkedList<Token> classTokens;
	ListIterator<Token> classIterator;
	Token currentToken;
	int peerCount = 0;
	boolean peerActive = false;
	Parser parser;
	
	public ParseClass(List<Token> tokens)
	{
		classConstruct = new ClassConstruct();
		classConstruct.setLocalInitializer(new MethodConstruct(new Signature()));
		classConstruct.getLocalInitializer().setBody(new SequenceNode());
		classConstruct.setStaticInitializer(new MethodConstruct(new Signature()));
		classConstruct.getStaticInitializer().setBody(new SequenceNode());
		
		classTokens = (LinkedList<Token>) tokens;
		classIterator = classTokens.listIterator();
		currentToken = classIterator.next();
		parser = new Parser(classTokens);
	}

	public ClassConstruct parseClassConstruct(List<Token> tokens) throws ParseException
	{
		//TODO parse package
		
		//TODO parse imports
		
		//Parse class
		while(!currentToken.getValue().equals("}"))
		{
			if(currentToken.getType() == Type.MODIFIER_ACCESS || currentToken.getType() == Type.MODIFIER_ACCESS_PERMISSIONS) 
			{ 
				checkAgainst();
			}
			nextToken();
		}

		return classConstruct;
	}

	private void checkAgainst() throws ParseException
	{
		//TODO check for static scopes
		startPeer();
		nextToken();
		
		while(true)
		{
			if(currentToken.getType() == Type.CONSTRUCT_TYPE_DEF)
			{
				//TODO check for classes, currently ignoring because were assuming there is only one class total
				revertPeer();
				nextToken(3);
				
				classConstruct.setClassName(currentToken.getValue());
				nextToken();
				break;
			}
			else if(currentToken.getType() == Type.TYPE)
			{
				nextToken(2);
				if(currentToken.getType() == Type.CONTROL)
				{
					revertPeer();
					parseMethod();
					break;
				}
				else if(currentToken.getType() == Type.OPERATOR_BINARY)
				{
					revertPeer();
					parseVariable();
					break;
				}
			}
			
			nextToken();
		}
	}
	
	private void parseVariable() throws ParseException
	{
		List<Variable> variables = new LinkedList<Variable>();
		List<Token> expressionTokens;
		
		boolean isStatic = false;
		String type = "";
		String operator = "";
		
		//Parse variables
		while(!currentToken.getValue().equals(";"))
		{
			if(currentToken.getType() == Type.MODIFIER_ACCESS)
				isStatic = true;
			else if(currentToken.getType() == Type.TYPE)
				type = currentToken.getValue();
			else if(currentToken.getType() == Type.REFERNCE)
				variables.add(new Variable(type, currentToken.getValue(), isStatic));
			else if(currentToken.getType() == Type.OPERATOR_BINARY)
				break;
			
			nextToken();
		}
		
		//Add operator and name to SequenceNode. i.e a = 1 + 2;
		operator = currentToken.getValue();
		
		expressionTokens = parseExpression();
		//Parse Assignment
		SequenceNode expression = new SequenceNode();
		expression = parser.parseArithmeticStatement(expressionTokens);
		OperatorNode operatorNode = new OperatorNode(operator);
		
		List<ParseNode> localInitializerChildren = classConstruct.getLocalInitializer().getBody().getChildren();
		List<ParseNode> globalInitializerChildren = classConstruct.getStaticInitializer().getBody().getChildren();
		
		for(Variable variable : variables)
		{
			if(isStatic)
			{
				classConstruct.addGlobalVariable(variable);
				
				List<ParseNode> operatorChildren = operatorNode.getChildren();
				operatorChildren.add(new ValueNode(variable.getName()));
				operatorChildren.add(expression.getChildren().get(0));
				TreeSupport.insertChildOver(expression.getChildren(), 0, operatorNode);
				globalInitializerChildren.add(expression);
			}
			else
			{
				classConstruct.addLocalVariable(variable);
				
				List<ParseNode> operatorChildren = operatorNode.getChildren();
				operatorChildren.add(new ValueNode(variable.getName()));
				operatorChildren.add(expression.getChildren().get(0));
				TreeSupport.insertChildOver(expression.getChildren(), 0, operatorNode);
				localInitializerChildren.add(expression);
			}
		}
	}

	private List<Token> parseExpression()
	{
		List<Token> expression = new LinkedList<>();
		
		while(!currentToken.getValue().equals(";"))
		{
			nextToken();
			expression.add(currentToken);
		}
		
		return expression;
	}

	private void parseMethod() throws ParseException
	{
		Signature signature = new Signature();
		MethodConstruct methodConstruct = new MethodConstruct(signature);
		
		List<Variable> arguments = new LinkedList<>();
		boolean isStatic = false;
		boolean isMain = false;
		String type = "";
		String methodName = "";
		String permission = "";
		
		//Parse signature
		while(!currentToken.getValue().equals("{"))
		{
			if(currentToken.getType() == Type.MODIFIER_ACCESS)
				isStatic = true;
			else if(currentToken.getType() == Type.MODIFIER_ACCESS_PERMISSIONS)
				permission = currentToken.getValue();
			else if(currentToken.getType() == Type.TYPE)
				type = currentToken.getValue();
			else if(currentToken.getType() == Type.REFERNCE)
			{
				methodName = currentToken.getValue();
				if(methodName.equals("main"))
					isMain = true;
			}
			else if(currentToken.getType() == Type.CONTROL)
			{
				nextToken();
				while(!currentToken.getValue().equals(")"))
				{
					if(currentToken.getType() == Type.TYPE)
					{
						String argType = currentToken.getValue();
						nextToken();
						String argName = currentToken.getValue();
						arguments.add(new Variable(argType, argName));
					}
					
					nextToken();
				}
			}
			
			nextToken();
		}
		nextToken();
		signature.setAccessModifier(permission);
		signature.setStatic(isStatic);
		signature.setMethodName(methodName);
		signature.setReturnType(type);
		signature.setArguments(arguments);
		
		//Parse method body
		SequenceNode body = parseMethodBody();
		methodConstruct.setBody(body);
		if(isMain)
			classConstruct.setMainMethod(methodConstruct);
		else
		{
			if(isStatic)
				classConstruct.addStaticMethod(methodConstruct);
			else
				classConstruct.addClassMethod(methodConstruct);
		}
	}
	
	private SequenceNode parseMethodBody() throws ParseException
	{
		SequenceNode body = new SequenceNode();
		List<Token> bodyTokens = getBodyTokens();
		List<LinkedList<Token>> statementsList = separateStatements(bodyTokens);
		List<SequenceNode> bodyStatements = new LinkedList<>();
		//TODO support multiple references
		String reference = "";
		
		for(List<Token> statement : statementsList)
		{
			LinkedList<Token> declaration = null;
			SequenceNode declarationNode = null;
			
			if(isDeclaration(statement))
			{
				declaration = parseDelcaration(statement);
				declarationNode = parser.parseDeclaration(declaration);
			}
			reference = getFirstInstance(Type.REFERNCE, statement);
			
			
			LinkedList<Token> expression = findExpression(statement);

			SequenceNode node = parser.parseArithmeticStatement(expression);
			if(node.getChildren().size() >= 1)
			{
				//TreeSupport.printTree(node);
				
				OperatorNode operator = new OperatorNode(getFirstInstance(Type.OPERATOR_BINARY, statement));
				List<ParseNode> operatorChildren = operator.getChildren();
				TreeSupport.insertChildAt(operatorChildren, 0, new ValueNode(reference));
				TreeSupport.insertChildAt(operatorChildren, 1, node.getChildren().get(0));
				node = new SequenceNode();
				TreeSupport.insertChildAt(node.getChildren(), 0, operator);
			}
			
			if(declarationNode != null)
			{
				for(ParseNode declarationChild : declarationNode.getChildren())
				{
					TreeSupport.insertChildAt(node.getChildren(), 0, declarationChild);
				}
			}
			
			bodyStatements.add(node);
			
			System.out.println("\n<----- Parse Arithmetic Statement ----->");
			if(node.getChildren().size() > 0)
				TreeSupport.printTree(node);
			else
				System.out.println("Empty node");
		}
		
		
		List<ParseNode> bodyChildren = body.getChildren();
		
		for(SequenceNode node : bodyStatements)
		{
			bodyChildren.add(node);
		}
		
		return body;
		
	}
	
	private String getFirstInstance(Token.Type type, List<Token> statement)
	{
		for(Token token : statement)
		{
			if(token.getType() == type)
				return token.getValue();
		}
		return null;
	}

	private List<Token> getBodyTokens()
	{
		List<Token> bodyTokens = new LinkedList<>();
		
		while(!currentToken.getValue().equals("}"))
		{
			bodyTokens.add(currentToken);
			nextToken();
		}
		
		return bodyTokens;
	}
	
	private List<LinkedList<Token>> separateStatements(List<Token> tokensParam)
	{
		List<LinkedList<Token>> statements = new LinkedList<LinkedList<Token>>();
		LinkedList<Token> statement = null;
		boolean newStatement = true;
		
		for(int index = 0; index < tokensParam.size(); index++)
		{
			if(newStatement)
			{
				newStatement = false;
				statement = new LinkedList<>();
				statement.add(tokensParam.get(index));
			}
			else if(!newStatement)
			{
				statement.add(tokensParam.get(index));
				if(tokensParam.get(index).getValue().equals(";"))
				{
					newStatement = true;
					statements.add(statement);
				}
			}
		}
		
		return statements;
	}
	
	private boolean isDeclaration(List<Token> statement)
	{
		if(statement.get(0).getType() == Type.TYPE)
			return true;
		else
			return false;
	}

	private LinkedList<Token> parseDelcaration(List<Token> statement)
	{
		LinkedList<Token> declaration = new LinkedList<>();
		
		for(int index = 0; index < statement.size(); index++)
		{
			if(statement.get(index).getType() == Type.OPERATOR_BINARY)
				break;
			else
				declaration.add(statement.get(index));
		}
		
		return declaration;
	}
	
	private LinkedList<Token> findExpression(List<Token> statement)
	{
		LinkedList<Token> expression = new LinkedList<>();
		boolean start = false;
		
		for(int index = 0; index < statement.size(); index++)
		{
			if(statement.get(index).getType() == Type.OPERATOR_BINARY && start == false)
			{
				start = true;
				continue;
			}
			if(statement.get(index).getType() == Type.ACTION)
				break;
			if(start)
			{
				expression.add(statement.get(index));
			}
		}
		return expression;
	}

	private void ungetToken()
	{
		ungetToken(1);
	}
	
	private void ungetToken(int count)
	{
		for (int index = 0; index < count; index++)
		{
			currentToken = classIterator.previous();
		}
	}
	
	private boolean nextToken()
	{
		return nextToken(1);
	}
	
	private boolean nextToken(int steps)
	{
		for(int index = 0; index < steps; index++)
		{
			if (peerActive)
				peerCount++;
			currentToken = classIterator.next();
		}
		return true;
	}
	
	private void startPeer()
	{
		// Use to peer ahead some number of tokens. Call revert peer to go back
		// same starting token.
		peerCount = 1;
		peerActive = true;
		
	}
	
	private void revertPeer()
	{
		peerActive = false;
		ungetToken(peerCount);
	}
	
	
	
}
