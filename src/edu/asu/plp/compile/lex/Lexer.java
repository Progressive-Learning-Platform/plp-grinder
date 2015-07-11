package edu.asu.plp.compile.lex;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Collections;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import edu.asu.plp.Token;

public class Lexer
{
	private List<String> lines;
	
	public Lexer(File file) throws FileNotFoundException
	{
		this.lines = Preprocessor.parse(file);
	}
	
	public Lexer(String filePath) throws FileNotFoundException
	{
		this(new File(filePath));
	}
	
	public void dumpPreprocessData(File dumpSite) throws FileNotFoundException
	{
		PrintWriter output = new PrintWriter(dumpSite);
		for (String line : lines)
		{
			output.println(line);
		}
		output.close();
	}
	
	public List<Token> lex() throws LexException
	{
		List<Token> tokens = new LinkedList<>();
		
		for (String line : lines)
		{
			List<Token> lineTokens = lex(line);
			tokens.addAll(lineTokens);
		}
		
		return tokens;
	}
	
	private List<Matcher> findAndSort(Collection<Matcher> matchers, int startIndex)
	{
		List<Matcher> matches = new ArrayList<>();
		
		for (Matcher matcher : matchers)
			if (matcher.find(startIndex))
				matches.add(matcher);
		
		matches.sort(new MatchSorter());
		
		return matches;
	}
	
	private List<Token> lex(String line) throws LexException
	{
		if (line.trim().length() == 0)
			return Collections.<Token> emptyList();
		
		Map<Matcher, Token.Type> matchers = new HashMap<>();
		
		for (Token.Type tokenType : Token.Type.values())
		{
			Pattern pattern = Pattern.compile(tokenType.regex);
			Matcher matcher = pattern.matcher(line);
			
			matchers.put(matcher, tokenType);
		}
		
		List<Matcher> matches = findAndSort(matchers.keySet(), 0);
		ArrayList<String> tokenStrings = new ArrayList<>();
		int index;
		
		while (!matches.isEmpty())
		{
			Matcher firstMatch = matches.get(0);
			for (int i = 1; i < matches.size(); i++)
			{
				Matcher rival = matches.get(i);
				if (rival.start() == firstMatch.start())
				{
					if (rival.end() > firstMatch.end())
						firstMatch = rival;
					else if (rival.end() == firstMatch.end())
					{
						Token.Type firstType = matchers.get(firstMatch);
						Token.Type rivalType = matchers.get(rival);
						
						List<Token.Type> tokenTypes = Arrays.asList(Token.Type.values());
						int firstPriority = tokenTypes.indexOf(firstType);
						int rivalPriority = tokenTypes.indexOf(rivalType);
						
						if (firstPriority == rivalPriority)
							throw new LexException("Simultanious Matches Found");
						else if (firstPriority > rivalPriority)
							firstMatch = rival;
					}
				}
				else
				{
					break;
				}
			}
			
			String tokenString = line.substring(firstMatch.start(), firstMatch.end());
			tokenStrings.add(tokenString);
			index = firstMatch.end();
			if (index >= line.length())
				break;
			else
				matches = findAndSort(matchers.keySet(), index);
		}
		
		try
		{
			return Token.makeTokens(tokenStrings);
		}
		catch (LexException lexException)
		{
			System.err.println("Context: " + line);
			throw lexException;
		}
	}
}
