package edu.asu.plp.compile.parser;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.LinkedList;
import java.util.List;
import java.util.ListIterator;
import java.util.Stack;

import edu.asu.plp.Token;
import edu.asu.plp.Token.Type;
import edu.asu.plp.compile.parser.statements.Statement;
import edu.asu.plp.compile.parser.statements.Statement.StatementType;
import edu.asu.plp.compile.parser.statements.components.Expression;
import edu.asu.plp.compile.parser.statements.components.ExpressionComponent;
import edu.asu.plp.compile.parser.statements.components.Primitive;
import edu.asu.plp.compile.parser.statements.components.ValueNode;
import edu.asu.plp.compile.parser.statements.components.Primitive.PrimitiveType;
import edu.asu.plp.scope.Scope;

public class Parser
{
	public enum BodyType
	{
		CLASS("class"),
		METHOD("method");
		
		private String value;
		
		private BodyType(String value)
		{
			this.value = value;
		}
		
		public String getValue()
		{
			return value;
		}
	}
	
	private List<Statement> statements;
	private Stack<Scope> scopes;
	private Scope currentScope;
	private Scope classStaticScope;
	
	private Token currentToken;
	private ListIterator<Token> tokenIterator;
	private int peerCount = 0;
	private boolean peerActive = false;
	
	public Parser()
	{
		
	}
	
	public List<Statement> parse(List<Token> tokens) throws ParseException
	{
		statements = new LinkedList<>();
		
		scopes = new Stack<Scope>();
		currentScope = Scope.makeRootScope();
		scopes.push(currentScope);
		
		tokenIterator = (ListIterator<Token>) tokens.iterator();
		
		parseConstructDefinition();
		
		return statements;
	}
	
	private Statement parseStatement()
	{
		nextToken();
		Statement statement = null;
		Variable leftHandSide;
		String operator;
		Expression rightHandSide;
		
		// either existing variable or UserMadeClass
		if (currentToken.getType() == Type.REFERNCE)
		{
			// Check against user-made classes for initialization
			
			// Check against initialized objects
			// TODO: Remove, using for debugging
			// call initialized parsing method?
			statement = new Statement(StatementType.ASSIGNMENT);
			
			boolean isConstant = false;
			boolean isStatic = false;
			String type = null;
			String name = null;
			
			while (currentToken.getType() != Type.OPERATOR)
			{
				if (currentToken.getValue().equals("static"))
					isStatic = true;
				else if (currentToken.getValue().equals("final"))
					isConstant = true;
				else if (currentToken.getType() == Type.TYPE)
					type = currentToken.getValue();
				else if (currentToken.getType() == Type.REFERNCE)
					name = currentToken.getValue();
				
				nextToken();
			}
			operator = currentToken.getValue();
			parseExpression();
			
			// Check Against Methods
			// Method is a list of statements (add LinkedList<Statement> to
			// statement).
			
			// else check against variables in scope
			// Waiting on change to scope.
			//@formatter:off
			/*
			if(currentScope.contains(currentToken.getValue())) 
			{
				//Get string id back for variable.
				
			}
			*/
			//@formatter:on
			
			// TODO remove, parseExpression() currently for running purposes.
			parseExpression();
		}
		else if (currentToken.getType() == Type.ACTION)
		{
			// TODO: return|continue|break|throw|assert
			
			// TODO remove, parseExpression() currently for running purposes.
			parseExpression();
		}
		// Parse left hand side for new variable
		else
		{
			statement = new Statement(StatementType.ASSIGNMENT);
			
			boolean isConstant = false;
			boolean isStatic = false;
			String type = null;
			String name = null;
			
			while (currentToken.getType() != Type.OPERATOR)
			{
				if (currentToken.getValue().equals("static"))
					isStatic = true;
				else if (currentToken.getValue().equals("final"))
					isConstant = true;
				else if (currentToken.getType() == Type.TYPE)
					type = currentToken.getValue();
				else if (currentToken.getType() == Type.REFERNCE)
					name = currentToken.getValue();
				
				nextToken();
			}
			
			// TODO check for variable inside scope
			leftHandSide = new Variable(type, name, isConstant);
			
			if (isStatic)
				classStaticScope.addVariable(leftHandSide);
			else
				currentScope.addVariable(leftHandSide);
			
			operator = currentToken.getValue();
			parseExpression();
		}
		
		return statement;
	}
	
	private Expression parseExpression()
	{
		// TODO: Add methods and object calls on right side of assignment
		// statement
		Expression expression;
		ExpressionComponent expressionComponent;
		
		nextToken();
		ArrayList<Token> expressionTokens = new ArrayList<Token>();
		
		while (!currentToken.getValue().equals(";"))
		{
			expressionTokens.add(currentToken);
			nextToken();
		}
		
		expressionComponent = parseIntoExpressionComponents(expressionTokens);
		expression = new Expression(expressionComponent);
		
		return expression;
	}
	
	private ExpressionComponent parseIntoExpressionComponents(
			ArrayList<Token> expressionTokens)
	{
		ExpressionComponent expression;
		ExpressionComponent leftExpression = null;
		String operator;
		ExpressionComponent rightExpression = null;
		
		if (expressionTokens.size() == 1)
		{
			Type type = expressionTokens.get(0).getType();
			String tokenValue = expressionTokens.get(0).getValue();
			
			expression = parseForSingleExpressionComponent(type, tokenValue);
			
			return expression;
		}
		
		ArrayList<ExpressionComponent> components = new ArrayList<>();
		ArrayList<String> stringTokens = new ArrayList<>();
		
		for (int index = 0; index < expressionTokens.size(); index++)
		{
			Type type = expressionTokens.get(index).getType();
			String tokenValue = expressionTokens.get(index).getValue();
			
			if (isValueToken(type))
				components.add(parseForSingleExpressionComponent(type,
						tokenValue));
			else
				stringTokens.add(tokenValue);
		}
		
		operator = "";
		
		expression = new ExpressionComponent(leftExpression, rightExpression,
				operator);
		
		System.out.println(stringTokens);
		
		return expression;
	}
	
	private ExpressionComponent parseParenthesisComponent(
			ArrayList<Token> expressionTokens, int startingPoint)
	{
		ExpressionComponent expressionComponent;
		int elementCount = 0;
		
		ArrayList<ExpressionComponent> values = new ArrayList<>();
		ArrayList<String> operators = new ArrayList<>();
		
		for(int index = startingPoint; index < expressionTokens.size(); index++)
		{
			Type type = expressionTokens.get(index).getType();
			String tokenValue = expressionTokens.get(index).getValue();
			
			if(tokenValue.equals("("))
			{
				elementCount++;
			}
			else if(tokenValue.equals(")"))
			{
				elementCount--;
			}
			else if(isValueToken(type))
			{
				values.add(parseForSingleExpressionComponent(type, tokenValue));
			}
			else if(type == Type.OPERATOR)
			{
				operators.add(tokenValue);
			}
			if(elementCount == 0)
				break;
		}
		
		
		return null;
	}
	
	private boolean isValueToken(Type type)
	{
		// TODO: account for ++
		if (type == Type.REFERNCE || type == Type.LITERAL_BOOLEAN
				|| type == Type.LITERAL_CHAR || type == Type.LITERAL_DOUBLE
				|| type == Type.LITERAL_FLOAT || type == Type.LITERAL_INT
				|| type == Type.LITERAL_LONG)
			return true;
		else
			return false;
	}
	
	private ExpressionComponent parseForSingleExpressionComponent(Type type,
			String tokenValue)
	{
		// TODO: Figure out ++ component
		ExpressionComponent expression = null;
		
		if (type == Type.REFERNCE)
			expression = new ExpressionComponent(new ValueNode(tokenValue));
		else if (type == Type.LITERAL_BOOLEAN)
			expression = new ExpressionComponent(new ValueNode(new Primitive(
					PrimitiveType.BOOLEAN, Boolean.parseBoolean(tokenValue))));
		else if (type == Type.LITERAL_CHAR)
			expression = new ExpressionComponent(new ValueNode(new Primitive(
					PrimitiveType.CHAR, tokenValue.charAt(0))));
		else if (type == Type.LITERAL_DOUBLE)
			expression = new ExpressionComponent(new ValueNode(new Primitive(
					PrimitiveType.DOUBLE, Double.parseDouble(tokenValue))));
		else if (type == Type.LITERAL_FLOAT)
			expression = new ExpressionComponent(new ValueNode(new Primitive(
					PrimitiveType.FLOAT, Float.parseFloat(tokenValue))));
		else if (type == Type.LITERAL_INT && !tokenValue.contains("x"))
			expression = new ExpressionComponent(new ValueNode(new Primitive(
					PrimitiveType.INT, Integer.parseInt(tokenValue))));
		else if (type == Type.LITERAL_INT && tokenValue.contains("x"))
			expression = new ExpressionComponent(new ValueNode(new Primitive(
					PrimitiveType.INT, Integer.parseInt(
							tokenValue.replaceFirst("0x", ""), 16))));
		else if (type == Type.LITERAL_LONG)
			expression = new ExpressionComponent(new ValueNode(new Primitive(
					PrimitiveType.INT, Integer.parseInt(tokenValue))));
		return expression;
	}
	
	private void parseMethodDefinition()
	{
		parseMethodHeader();
		parseMethodBody();
	}
	
	private void parseConstructDefinition()
	{
		parseClassHeader();
		parseClassBody();
	}
	
	private void parseClassHeader()
	{
		nextToken();
		// TODO: Check for extends and implements
		while (!currentToken.getValue().equals("{"))
		{
			if (currentToken.getType() == Type.MODIFIER_ACCESS)
			{
			}
			else if (currentToken.getType() == Type.CONSTRUCT_TYPE_DEF)
			{
				// TODO: check if interface or enum, and iterate past it
				// consume construct name
				nextToken();
				String getConstructName = currentToken.getValue();
				
				// consume left brace
				nextToken();
				continue;
			}
			else if (currentToken.getType() == Type.REFERNCE)
			{
				
			}
			nextToken();
		}
		newScope(BodyType.CLASS);
	}
	
	private void parseMethodHeader()
	{
		nextToken();
		// TODO: Check for extends and implements
		// TODO: Check for throws
		while (!currentToken.getValue().equals("{"))
		{
			if (currentToken.getType() == Type.MODIFIER_ACCESS)
			{
			}
			else if (currentToken.getType() == Type.REFERNCE)
			{
				nextToken();
				if (currentToken.getValue().equals("("))
				{
					parseArguments();
				}
			}
			else if (currentToken.getType() == Type.TYPE)
			{
				
			}
			nextToken();
		}
		newScope(BodyType.METHOD);
	}
	
	private void parseClassBody()
	{
		nextToken();
		// TODO check for scopes {}
		while (!currentToken.getValue().equals("}"))
		{
			if (currentToken.getType() == Type.MODIFIER_ACCESS)
			{
				if (currentToken.getValue().equals("static"))
				{
					checkAgainst(currentToken);
				}
				else if (currentToken.getValue().equals("final"))
				{
					checkAgainst(currentToken);
				}
			}
			else if (currentToken.getType() == Type.TYPE)
			{
				startPeerAhead();
				nextToken();
				if (currentToken.getType() == Type.REFERNCE)
				{
					nextToken();
					if (currentToken.getType() == Type.OPERATOR)
					{
						revertPeerAhead();
						parseStatement();
					}
					else if (currentToken.getType() == Type.CONTROL)
					{
						// TODO check for classes
						revertPeerAhead();
						parseMethodDefinition();
					}
				}
			}
			else if (currentToken.getType() == Type.REFERNCE)
			{
				startPeerAhead();
				nextToken();
				// TODO remove things you cant do in the body. Copied from
				// method call
				if (currentToken.getValue().equals("("))
				{
					revertPeerAhead();
					// Check for constructor
				}
				else if (currentToken.getType() == Type.OPERATOR)
				{
					revertPeerAhead();
					parseStatement();
				}
			}
			nextToken();
		}
		closeScope(BodyType.CLASS);
	}
	
	private void parseMethodBody()
	{
		nextToken();
		
		while (!currentToken.getValue().equals("}"))
		{
			if (currentToken.getType() == Type.MODIFIER_ACCESS)
			{
				if (currentToken.getValue().equals("static"))
				{
					checkAgainst(currentToken);
				}
				else if (currentToken.getValue().equals("final"))
				{
					checkAgainst(currentToken);
				}
			}
			else if (currentToken.getType() == Type.TYPE)
			{
				startPeerAhead();
				nextToken();
				
				// TODO check for scopes {}
				if (currentToken.getType() == Type.REFERNCE)
				{
					nextToken();
					if (currentToken.getType() == Type.OPERATOR)
					{
						revertPeerAhead();
						parseStatement();
					}
					else if (currentToken.getType() == Type.CONTROL)
					{
						revertPeerAhead();
						parseMethodDefinition();
					}
				}
			}
			else if (currentToken.getType() == Type.REFERNCE)
			{
				startPeerAhead();
				nextToken();
				
				if (currentToken.getValue().equals("(")
						|| currentToken.getValue().equals(".")
						|| currentToken.getType() == Type.OPERATOR)
				{
					revertPeerAhead();
					parseStatement();
				}
			}
			// TODO debugging, remove and change
			else if (currentToken.getValue().equals("return"))
			{
				ungetToken();
				parseStatement();
			}
			nextToken();
		}
		closeScope(BodyType.METHOD);
	}
	
	private void parseArguments()
	{
		nextToken();
		
		while (!currentToken.getValue().equals(")"))
		{
			nextToken();
		}
	}
	
	private void parseObjectCall()
	{
	}
	
	private void parseMethodCall()
	{
		// Placeholder, consuming semicolon for now
		nextToken();
	}
	
	private void checkAgainst(Token modifier)
	{
		startPeerAhead();
		nextToken();
		
		while (true)
		{
			if (currentToken.getType() == Type.CONSTRUCT_TYPE_DEF)
			{
				// Class in class, or local class in method
				revertPeerAhead();
				parseConstructDefinition();
				break;
			}
			else if (currentToken.getType() == Type.REFERNCE)
			{
				nextToken();
				
				// It's an assignment
				if (currentToken.getType() == Type.OPERATOR)
				{
					revertPeerAhead();
					parseStatement();
					break;
				}
				// Method or Class
				else if (currentToken.getType() == Type.CONTROL)
				{
					if (currentToken.getValue().equals("{"))
					{
						revertPeerAhead();
						parseConstructDefinition();
					}
					else if (currentToken.getValue().equals("("))
					{
						revertPeerAhead();
						parseMethodDefinition();
					}
					break;
				}
			}
			nextToken();
		}
	}
	
	private void ungetToken()
	{
		ungetToken(1);
	}
	
	private void ungetToken(int count)
	{
		for (int index = 0; index < count; index++)
		{
			currentToken = tokenIterator.previous();
		}
	}
	
	private boolean nextToken()
	{
		if (tokenIterator.hasNext())
		{
			if (peerActive)
				peerCount++;
			currentToken = tokenIterator.next();
			return true;
		}
		else
			return false;
	}
	
	private void startPeerAhead()
	{
		// Use to peer ahead some number of tokens. Call revert peer to go back
		// same starting token.
		peerCount = 1;
		peerActive = true;
		
	}
	
	private void revertPeerAhead()
	{
		peerActive = false;
		ungetToken(peerCount);
	}
	
	private void newScope(BodyType bodyType)
	{
		if (bodyType == BodyType.CLASS)
		{
			classStaticScope = currentScope.makeChild();
			currentScope = classStaticScope.makeChild();
			scopes.push(classStaticScope);
			scopes.push(currentScope);
		}
		else if (bodyType == BodyType.METHOD)
		{
			classStaticScope = null;
			scopes.push(currentScope);
			currentScope = currentScope.makeChild();
			scopes.push(currentScope);
		}
	}
	
	private void closeScope(BodyType bodyType)
	{
		if (bodyType == BodyType.CLASS)
		{
			scopes.pop();
			currentScope = scopes.peek();
		}
		else if (bodyType == BodyType.METHOD)
		{
			scopes.pop();
			currentScope = scopes.pop();
			classStaticScope = scopes.peek();
		}
	}
	
}
