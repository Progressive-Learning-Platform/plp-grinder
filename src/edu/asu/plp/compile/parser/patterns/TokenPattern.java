package edu.asu.plp.compile.parser.patterns;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

import edu.asu.plp.Token;
import edu.asu.plp.Token.Type;
import edu.asu.plp.compile.lex.LexException;

public class TokenPattern
{
	public static void main(String[] args) throws LexException
	{
		TokenPattern pattern = new TokenPattern("Declaration");
		pattern.add(Type.MODIFIER_ACCESS_PERMISSIONS, true);
		pattern.add("static", true);
		pattern.add(Type.TYPE, false);
		pattern.add(Type.REFERNCE, false);
		pattern.add(";", false);
		
		List<Token> test = new ArrayList<>();
		test.add(new Token("int"));
		test.add(new Token("bear"));
		
		TokenPattern declarationPattern = new TokenPattern("Declaration");
		declarationPattern.add(Type.TYPE, false);
		declarationPattern.add(Type.REFERNCE, false);
		
		TokenPattern assignmentPattern = new TokenPattern("Assignment");
		assignmentPattern.add("=", false);
		
		TokenPattern methodCallStartPattern = new TokenPattern("MethodStart");
		methodCallStartPattern.add(Type.REFERNCE, false);
		methodCallStartPattern.add("\\(", false);
		
		TokenPattern methodCallEndPattern = new TokenPattern("MethodStart");
		methodCallEndPattern.add("\\)", false);
		
		System.out.println(pattern.isPotentialMatch(test));
	}
	
	private static class Node
	{
		String regex;
		boolean optional;
		
		public Node(String regex, boolean optional)
		{
			this.regex = regex;
			this.optional = optional;
		}
		
		public boolean matches(Token token)
		{
			return token.getValue().matches(regex);
		}
	}
	
	private List<Node> sequence;
	private String name;
	
	public TokenPattern(String name)
	{
		this.name = name;
		this.sequence = new ArrayList<>();
	}
	
	public boolean add(String regex, boolean optional)
	{
		return this.sequence.add(new Node(regex, optional));
	}
	
	public boolean add(Token.Type type, boolean optional)
	{
		return this.add(type.regex, optional);
	}
	
	public boolean matches(List<Token> sequence)
	{
		Iterator<Node> iterator = this.sequence.iterator();
		Node current;
		
		for (Token token : sequence)
		{
			if (!iterator.hasNext())
				break;
			else
				current = iterator.next();
			
			boolean tokenMatched = false;
			
			while (!tokenMatched)
			{
				if (current.matches(token))
					tokenMatched = true;
				else if (current.optional && iterator.hasNext())
					current = iterator.next();
				else
					return false;
			}
		}
		
		return !iterator.hasNext();
	}
	
	public boolean isPotentialMatch(List<Token> sequence)
	{
		Iterator<Node> iterator = this.sequence.iterator();
		Node current;
		
		for (Token token : sequence)
		{
			if (!iterator.hasNext())
				break;
			else
				current = iterator.next();
			
			boolean tokenMatched = false;
			
			while (!tokenMatched)
			{
				if (current.matches(token))
					tokenMatched = true;
				else if (current.optional && iterator.hasNext())
					current = iterator.next();
				else
					return false;
			}
		}
		
		return true;
	}
	
	public String getName()
	{
		return name;
	}
}
