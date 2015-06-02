package edu.asu.plp.compile.parser;

import java.util.LinkedList;
import java.util.List;
import java.util.ListIterator;
import java.util.Stack;

import edu.asu.plp.Token;
import edu.asu.plp.Token.Type;
import edu.asu.plp.compile.parser.statements.Statement;
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
	
	List<Statement> statements;
	Stack<Scope> scopes;
	Scope currentScope;
	Scope classStaticScope;
	
	Token currentToken;
	ListIterator<Token> tokenIterator;
	int peerCount = 0;
	boolean peerActive = false;
	
	public Parser()
	{
		
	}
	
	public List<Statement> parse(List<Token> tokens) throws ParseException
	{
		System.out.println("<--------Parser------>");
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
		Statement statement = null;
		
		Variable leftHandSide;
		
		while (!currentToken.getValue().equals(";"))
		{
			nextToken();
			System.out.println("Statement: " + currentToken);
		}
		
		return statement;
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
				//TODO: check if interface or enum, and iterate past it
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
		//TODO check for scopes {}
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
				startPeer();
				nextToken();
				if(currentToken.getType() == Type.REFERNCE)
				{
					nextToken();
					if(currentToken.getType() == Type.OPERATOR)
					{
						revertPeer();
						parseStatement();
					}
					else if(currentToken.getType() == Type.CONTROL)
					{
						//TODO check for classes
						revertPeer();
						parseMethodDefinition();
					}
				}
			}
			else if (currentToken.getType() == Type.REFERNCE)
			{
				startPeer();
				nextToken();
				//TODO remove things you cant do in the body. Copied from method call
				if (currentToken.getValue().equals("("))
				{
					revertPeer();
					//Check for constructor
				}
				else if (currentToken.getType() == Type.OPERATOR)
				{
					revertPeer();
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
				startPeer();
				nextToken();

				//TODO check for scopes {}
				if(currentToken.getType() == Type.REFERNCE)
				{
					nextToken();
					if(currentToken.getType() == Type.OPERATOR)
					{
						revertPeer();
						parseStatement();
					}
					else if(currentToken.getType() == Type.CONTROL)
					{
						revertPeer();
						parseMethodDefinition();
					}
				}
			}
			else if (currentToken.getType() == Type.REFERNCE)
			{
				startPeer();
				nextToken();
				
				if (currentToken.getValue().equals("("))
				{
					revertPeer();
					// TODO parse arguments than pass those to method call?
					parseArguments();
					parseMethodCall();
				}
				else if (currentToken.getValue().equals("."))
				{
					revertPeer();
					parseObjectCall();
					
				}
				else if (currentToken.getType() == Type.OPERATOR)
				{
					revertPeer();
					parseStatement();
				}
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
		//Placeholder, consuming semicolon for now
		nextToken();
	}
	
	private void checkAgainst(Token modifier)
	{
		startPeer();
		nextToken();
		
		while (true)
		{
			if (currentToken.getType() == Type.CONSTRUCT_TYPE_DEF)
			{
				//Class in class, or local class in method
				revertPeer();
				parseConstructDefinition();
				break;
			}
			else if (currentToken.getType() == Type.REFERNCE)
			{
				nextToken();
				
				// It's an assignment
				if (currentToken.getType() == Type.OPERATOR)
				{
					revertPeer();
					parseStatement();
					break;
				}
				// Method or Class
				else if (currentToken.getType() == Type.CONTROL)
				{
					if (currentToken.getValue().equals("{"))
					{
						revertPeer();
						parseConstructDefinition();
					}
					else if (currentToken.getValue().equals("("))
					{
						revertPeer();
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
	
	private void newScope(BodyType bodyType)
	{
		if(bodyType == BodyType.CLASS)
		{
			classStaticScope = currentScope.makeChild();
			currentScope = classStaticScope.makeChild();
			scopes.push(classStaticScope);
			scopes.push(currentScope);
		}
		else if(bodyType == BodyType.METHOD)
		{
			classStaticScope = null;
			scopes.push(currentScope);
			currentScope = currentScope.makeChild();
			scopes.push(currentScope);
		}	
	}
	
	private void closeScope(BodyType bodyType)
	{
		if(bodyType == BodyType.CLASS)
		{
			scopes.pop();
			currentScope = scopes.peek();
		}
		else if(bodyType == BodyType.METHOD)
		{
			scopes.pop();
			currentScope = scopes.pop();
			classStaticScope = scopes.peek();
		}
	}
	
}
