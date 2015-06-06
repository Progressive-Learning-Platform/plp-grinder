package edu.asu.plp.compile.parser;

import java.util.Iterator;
import java.util.List;

import edu.asu.plp.Token;
import edu.asu.plp.Token.Type;
import edu.asu.plp.compile.parser.patterns.TokenPatternParser;

public class Parser
{
	private Iterator<Token> iterator;
	
	public Parser(List<Token> tokens)
	{
		iterator = tokens.iterator();
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
		TokenPatternParser parser = new TokenPatternParser();
	}
	
	
}
