package edu.asu.plp;

import java.io.BufferedReader;
import java.io.File;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.util.List;

import edu.asu.plp.compile.lex.Lexer;
import edu.asu.plp.compile.parser.Parser;
import edu.asu.plp.compile.parser.statements.Statement;
import edu.asu.util.Strings;

public class Main
{
	public static void main(String[] args) throws Exception
	{
		// Create scoped symbol table
		// Convert and simplify variables
		// Create execution graph
		// Write graph to assembly
		
		if (args.length == 0)
			args = new String[] { "sampleData/BasicArithmatic.java" };
		
		compileOracle(args);
		
		File inputFile = new File("sampleData/BasicArithmatic.java");
		File outputFile = new File("sampleData/output/BasicArithmatic.java.lexed");
		File dumpFile = new File("sampleData/BasicArithmatic.java.PREPROCESS");
		
		Lexer lexer = new Lexer(inputFile);
		lexer.dumpPreprocessData(dumpFile);
		List<Token> tokens = lexer.lex();
		
		PrintWriter output = new PrintWriter(outputFile);
		for (Token token : tokens)
		{
			// System.out.println(token);
			output.println(token);
		}
		output.close();
		
		Parser parser = new Parser();
		
		List<Statement> statements = parser.parse(tokens);
		for (Statement statement : statements)
		{
			System.out.println("s" + statement);
		}
		
	}
	
	private static boolean compileOracle(String[] args)
	{
		File classOutputDirectory = new File("./output/temp/class");
		classOutputDirectory.mkdirs();
		
		// TODO allow for compiling multiple classes / Work on all platforms
		String[] commands = new String[3];
		
		commands[0] = "javac";
		commands[1] = "-d";
		commands[2] = classOutputDirectory.getPath();
		
		// Concatenate args onto commands
		commands = Strings.concatArrays(commands, args);
		
		boolean success = build(commands);
		
		for (File childFile : classOutputDirectory.listFiles())
			childFile.delete();
		
		return success;
	}
	
	private static boolean build(String[] commands)
	{
		try
		{
			boolean encounteredErrors = false;
			
			ProcessBuilder builder = new ProcessBuilder(commands);
			builder.redirectErrorStream(true);
			
			Process process = builder.start();
			InputStream inputStream = process.getInputStream();
			BufferedReader reader = new BufferedReader(new InputStreamReader(inputStream));
			String line = reader.readLine();
			
			while (line != null)
			{
				encounteredErrors = true;
				System.out.println(line);
				line = reader.readLine();
			}
			
			if (encounteredErrors)
			{
				System.out.println("Unable to compile, because code is not valid java.");
				System.out.println("Please fix errors point out above.");
				System.exit(-1);
			}
			
			return encounteredErrors;
		}
		catch (Exception e)
		{
			e.printStackTrace();
			return false;
		}
	}
}
