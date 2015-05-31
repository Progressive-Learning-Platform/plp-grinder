package edu.asu.plp.compile.lex;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.function.Supplier;
import java.util.regex.Matcher;

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
			
			Matcher matcher = Token.STRING_LITERAL_PATTERN.matcher(line);
			String[] noStringLiterals = line.split(Token.Type.LITERAL_STRING.regex);
			StringBuilder lineBuilder = new StringBuilder();
			for (int index = 0; index < noStringLiterals.length; index++)
			{
				String substring = noStringLiterals[index];
				PreprocessResultToken result = new PreprocessResultToken();
				result.blockCommentActive = blockCommentActive;
				result.lineEnd = false;
				result = preprocess(substring, lineBuilder, result);
				blockCommentActive = result.blockCommentActive;
				if (result.lineEnd)
					break;
				
				if (matcher.find())
					lineBuilder.append(" " + matcher.group() + " ");
			}
			
			String parsedLine = lineBuilder.toString().trim();
			if (parsedLine.length() > 0)
				lines.add(parsedLine);
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
			boolean blockEnds = blockEndIndex >= 0;
			
			if (blockEnds)
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
			int blockCommentStartIndex = substring.indexOf("/*");
			boolean blockCommentStarts = blockCommentStartIndex >= 0;
			
			int lineCommentStartIndex = substring.indexOf("//");
			boolean lineCommentStarts = lineCommentStartIndex >= 0;
			
			Supplier<PreprocessResultToken> noComment = () -> {
				String string = substring.trim();
				if (string.length() > 0)
					lineBuilder.append(string);
				
				return result;
			};
			
			Supplier<PreprocessResultToken> lineComment = () -> {
				lineBuilder.append(substring.substring(0, lineCommentStartIndex));
				result.lineEnd = true;
				return result;
			};
			
			Supplier<PreprocessResultToken> blockComment = () -> {
				result.blockCommentActive = true;
				lineBuilder.append(substring.substring(0, blockCommentStartIndex));
				return preprocess(substring.substring(blockCommentStartIndex + 2),
						lineBuilder, result);
			};
			
			if (!(lineCommentStarts || blockCommentStarts))
			{
				return noComment.get();
			}
			else if (lineCommentStarts && blockCommentStarts)
			{
				return (lineCommentStartIndex < blockCommentStartIndex) ? lineComment
						.get() : blockComment.get();
				
			}
			else
			{
				return (lineCommentStartIndex >= 0) ? lineComment.get() : blockComment
						.get();
			}
		}
	}
}
