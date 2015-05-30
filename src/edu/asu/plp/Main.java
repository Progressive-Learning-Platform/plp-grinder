package edu.asu.plp;

import java.io.File;
import java.io.PrintWriter;
import java.util.List;

import edu.asu.plp.compile.lex.Lexer;

public class Main
{
	public static void main(String[] args) throws Exception
	{
		// Run Oracle Compile
		// if (compileError) ECHO and END
		// Convert and remove modifiers (simplify source code)
		// Create scoped symbol table
		// Convert and simplify variables
		// Create execution graph
		// Write graph to assembly
		
		// TODO: remove comments (preprocess)
		
		File inputFile = new File("BasicArithmatic.java");
		File outputFile = new File("BasicArithmatic.java.LEX");
		Lexer lexer = new Lexer(inputFile);
		PrintWriter output = new PrintWriter(outputFile);
		List<Token> tokens = lexer.lex();
		for (Token token : tokens)
		{
			System.out.println(token);
			output.println(token);
		}
		output.close();
	}
}
