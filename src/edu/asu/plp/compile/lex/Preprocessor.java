package edu.asu.plp.compile.lex;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.function.Supplier;

import edu.asu.plp.Token;

public class Preprocessor
{
	private static class PreprocessResultToken
	{
		private boolean blockCommentActive;
		private boolean lineEnd;
	}
	
	public static List<String> parse(File file) throws FileNotFoundException
	{
		List<String> lines = new ArrayList<>();
		Scanner scanner = new Scanner(file);
		boolean blockCommentActive = false;
		
		while (scanner.hasNext())
		{
			String line = scanner.nextLine().trim();
			
			String[] noStringLiterals = line.split(Token.Type.LITERAL_STRING.regex);
			StringBuilder lineBuilder = new StringBuilder();
			for (String substring : noStringLiterals)
			{
				PreprocessResultToken result = new PreprocessResultToken();
				result.blockCommentActive = blockCommentActive;
				result.lineEnd = false;
				result = preprocess(substring, lineBuilder, result);
				blockCommentActive = result.blockCommentActive;
				if (result.lineEnd)
					break;
			}
			
			lines.add(lineBuilder.toString());
		}
		
		scanner.close();
		return lines;
	}
	
	private static PreprocessResultToken preprocess(String substring,
			StringBuilder lineBuilder, PreprocessResultToken result)
	{
		if (result.blockCommentActive)
		{
			int blockEndIndex = substring.indexOf("*/");
			if (blockEndIndex >= 0)
			{
				result.blockCommentActive = false;
				return preprocess(substring.substring(blockEndIndex + 2), lineBuilder,
						result);
			}
			else
			{
				result.lineEnd = true;
				return result;
			}
		}
		else
		{
			int blockStartIndex = substring.indexOf("/*");
			int lineCommentStartIndex = substring.indexOf("//");
			
			Supplier<PreprocessResultToken> lineComment = () -> {
				lineBuilder.append(substring.substring(0, lineCommentStartIndex));
				result.lineEnd = true;
				return result;
			};
			
			Supplier<PreprocessResultToken> blockComment = () -> {
				result.blockCommentActive = true;
				lineBuilder.append(substring.substring(0, blockStartIndex));
				return preprocess(substring.substring(blockStartIndex + 2), lineBuilder,
						result);
			};
			
			Supplier<PreprocessResultToken> noComment = () -> {
				String string = substring.trim();
				if (string.length() > 0)
					lineBuilder.append(string);
				
				return result;
			};
			
			if (lineCommentStartIndex < 0 && blockStartIndex < 0)
			{
				return noComment.get();
			}
			else if (lineCommentStartIndex >= 0 && blockStartIndex >= 0)
			{
				if (lineCommentStartIndex < blockStartIndex)
				{
					return lineComment.get();
				}
				else
				{
					return blockComment.get();
				}
			}
			else
			{
				if (lineCommentStartIndex < 0)
				{
					return blockComment.get();
				}
				else
				{
					return lineComment.get();
				}
			}
		}
	}
}
