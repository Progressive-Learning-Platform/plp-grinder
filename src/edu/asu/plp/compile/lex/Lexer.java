package edu.asu.plp.compile.lex;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;

import edu.asu.plp.Token;
import edu.asu.util.Strings;

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
				holder.addAll(Strings.splitAndRetain(string, control));
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
}
