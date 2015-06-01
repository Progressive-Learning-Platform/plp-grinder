package edu.asu.plp;

import java.io.File;
import java.io.PrintWriter;
import java.util.List;

import edu.asu.plp.compile.lex.Lexer;
import edu.asu.plp.compile.parser.Parser;
import edu.asu.plp.compile.parser.statements.Statement;

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
		
		File inputFile = new File("sampleData/BasicArithmatic.java");
		File outputFile = new File("sampleData/output/BasicArithmatic.java.lexed");
		File dumpFile = new File("sampleData/BasicArithmatic.java.PREPROCESS");
		
		Lexer lexer = new Lexer(inputFile);
		lexer.dumpPreprocessData(dumpFile);
		List<Token> tokens = lexer.lex();
		
		PrintWriter output = new PrintWriter(outputFile);
		for (Token token : tokens)
		{
			System.out.println(token);
			output.println(token);
		}
		output.close();
		
		Parser parser = new Parser(tokens);
		
		List<Statement> statements = parser.parse(tokens);
	}
}
