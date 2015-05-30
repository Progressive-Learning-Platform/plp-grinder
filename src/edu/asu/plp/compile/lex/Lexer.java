package edu.asu.plp.compile.lex;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.LinkedList;
import java.util.List;

import edu.asu.plp.Token;

public class Lexer
{
	private List<String> lines;
	
	public Lexer(File file) throws FileNotFoundException
	{
		this.lines = Preprocessor.parse(file);
		
		// TODO: remove - for testing purposes only
		File outputFile = new File("sampleData/output/BasicArithmatic.java.PREPROCESS");
		PrintWriter output = new PrintWriter(outputFile);
		for (String line : lines)
		{
			output.println(line);
		}
		output.close();
	}
	
	public Lexer(String filePath) throws FileNotFoundException
	{
		this(new File(filePath));
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
	
	private List<Token> lex(String line) throws LexException
	{
		ArrayList<String> strings = new ArrayList<>();
		strings.add(line);
		
		// TODO: Extract String Literals
		
		for (String control : Token.CONTROL_TOKENS)
		{
			ArrayList<String> holder = new ArrayList<>();
			for (String string : strings)
				holder.addAll(splitAndRetain(string, control));
			strings = holder;
		}
		
		ArrayList<String> holder = new ArrayList<>();
		for (String string : strings)
		{
			// Split by whitespace
			holder.addAll(Arrays.asList(string.split("\\s")));
		}
		strings = holder;
		
		try
		{
			return Token.makeTokens(strings);
		}
		catch (LexException lexException)
		{
			System.err.println("Context: " + line);
			throw lexException;
		}
	}
	
	private List<String> splitAndRetain(String line, String delimeter)
	{
		if (line.trim().length() == 0)
			return Collections.<String> emptyList();
		
		boolean delimeterIsEscaped = delimeter.startsWith("\\");
		String prependex = delimeterIsEscaped ? delimeter.substring(1) : delimeter;
		List<String> subStrings = new ArrayList<>();
		String[] split = line.split(delimeter);
		
		if (split.length == 0)
		{
			subStrings.add(prependex);
		}
		else
		{
			// FIXME: verify that an extra delimiter cannot be placed
			if (split[0].length() > 0)
				subStrings.add(split[0]);
			for (int index = 1; index < split.length; index++)
			{
				String subString = split[index].trim();
				if (subString.length() > 0)
				{
					subStrings.add(prependex);
					subStrings.add(subString);
				}
			}
			
			if (line.endsWith(prependex))
				subStrings.add(prependex);
		}
		
		return subStrings;
	}
}
