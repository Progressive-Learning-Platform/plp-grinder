package edu.asu.plp.compile.parser.patterns;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;

import edu.asu.plp.Token;
import edu.asu.plp.Token.Type;

public class TokenPatternParser
{
	private Map<String, List<String>> patterns;
	private List<List<String>> remainingPatterns;
	private List<Token> currentSequence;
	
	public TokenPatternParser()
	{
		this.patterns = new HashMap<>();
		this.remainingPatterns = new LinkedList<>();
		this.currentSequence = new LinkedList<>();
	}
	
	private List<String> getMatch()
	{
		if (remainingPatterns.isEmpty())
			return null;
		else if (remainingPatterns.size() >= 2)
			throw new IllegalStateException("Singular match not found");
		else
			return remainingPatterns.get(0);
	}
	
	public String getMatchName()
	{
		List<String> match = getMatch();
		if (match == null)
			return null;
		
		name_loop:
		for (String name : patterns.keySet())
		{
			List<String> pattern = patterns.get(name);
			if (pattern.size() == match.size())
			{
				for (int i = 0; i < pattern.size(); i++)
				{
					if (!pattern.get(i).equals(match.get(i)))
						continue name_loop;
				}
				
				// Match found
				return name;
			}
		}
		
		throw new IllegalStateException("A match was found, but is no longer in memory");
	}
	
	public List<Token> getCurrentSequence()
	{
		return currentSequence;
	}
	
	public boolean add(Token token)
	{
		int currentIndex = currentSequence.size();
		currentSequence.add(token);
		
		List<List<String>> matches = new LinkedList<>();
		for (List<String> pattern : remainingPatterns)
		{
			if (pattern.size() <= currentIndex)
				continue;
			
			String current = pattern.get(currentIndex);
			if (current.equals(token.getType().name())
					|| current.equals(token.getValue()))
				matches.add(pattern);
		}
		
		this.remainingPatterns = matches;
		return matches.size() <= 1;
	}
	
	/**
	 * @param name
	 * @param pattern
	 * @return true if an old pattern was replaced
	 */
	private boolean addPattern(String name, List<String> pattern)
	{
		return patterns.put(name, pattern) != null;
	}
	
	/**
	 * @param name
	 * @param pattern
	 * @return true if an old pattern was replaced
	 */
	public boolean addPattern(String name, Object... pattern)
	{
		List<String> patternList = new ArrayList<>();
		
		for (Object token : pattern)
			if (token instanceof Type)
				patternList.add(((Type) token).name());
			else if (token instanceof String)
				patternList.add((String) token);
		
		return addPattern(name, patternList);
	}
}
