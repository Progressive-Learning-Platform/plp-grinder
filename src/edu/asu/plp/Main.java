package edu.asu.plp;

import java.io.BufferedReader;
import java.io.File;
import java.io.InputStreamReader;
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

		args = new String[1];
		args[0] = "sampleData/BasicArithmatic.java"; 
				
		if(args.length >= 1)
		{
			//TODO allow for compiling multiple classes / Work on all platforms
			int offset = 1;
			boolean encounteredErrors = false;
			String[] commands = new String[args.length + offset];
			String[] classesToDelete = new String[args.length];
			
			commands[0] = "javac";
			
			for(int arguments = 0; arguments < args.length; arguments++)
			{
				commands[arguments + offset] = args[arguments];
				classesToDelete[arguments] = args[arguments].replace(".java", ".class");
			}
			
			ProcessBuilder builder = new ProcessBuilder(commands);
			builder.redirectErrorStream(true);
			
			Process process = builder.start();
			BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()));
			String line = reader.readLine();
			
			while(line != null)
			{
				encounteredErrors = true;
				System.out.println(line);
				line = reader.readLine();
			}
			
			for(int arguments = 0; arguments < args.length; arguments++)
			{
				File file = new File(classesToDelete[arguments]);
				if(file.exists())
					file.delete();
			}
			
			if(encounteredErrors)
			{
				System.out.println("Unable to compile, because code is not valid java.");
				System.out.println("Please fix errors point out above.");
				System.exit(-1);
			}
		}
		
		File inputFile = new File("sampleData/BasicArithmatic.java");
		File outputFile = new File("sampleData/output/BasicArithmatic.java.lexed");
		File dumpFile = new File("sampleData/BasicArithmatic.java.PREPROCESS");
		
		Lexer lexer = new Lexer(inputFile);
		lexer.dumpPreprocessData(dumpFile);
		List<Token> tokens = lexer.lex();
		
		PrintWriter output = new PrintWriter(outputFile);
		for (Token token : tokens)
		{
			//System.out.println(token);
			output.println(token);
		}
		output.close();
		
		Parser parser = new Parser();
		
		List<Statement> statements = parser.parse(tokens);
		
	}
}
