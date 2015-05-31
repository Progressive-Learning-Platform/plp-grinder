package edu.asu.plp;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.LinkedList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import edu.asu.plp.compile.lex.LexException;

public class Token
{
	public static enum Type
	{
		CONTROL("\\.", "\\(", "\\)", "\\{", "\\}", "\\[", "\\]", ";"),
		LITERAL_INT("(0(x|b|d))?(\\d)+"),
		LITERAL_LONG("(\\d)+[lL]"),
		LITERAL_FLOAT("(((\\d+)?(\\.\\d+))|((\\d+)(\\.\\d+)?))[fF]"),
		LITERAL_DOUBLE("(\\d+)?(\\.\\d+)"),
		LITERAL_CHAR("'[a-z A-Z]?'"),
		LITERAL_STRING("\"([^\"\\\\\\n\\r]|\\\\.)*\""),
		LITERAL_BOOLEAN("true|false"),
		LITERAL_NULL("null"),
		OPERATOR("((\\+\\+|--|\\+|<<|>>|-|\\/|\\*|\\||&)=?)|="),
		COMPARATOR(">|>=|<|<=|&&|\\|\\||==|instanceof"),
		TYPE("boolean|long|int|byte|short|char|double|float|void"),
		MODIFIER_ACCESS("public|private|protected|static"),
		MODIFIER_BEHAVIOUR("final|volitile|transient|synchronized|native|abstract|throws"),
		MODIFIER_INHERITENCE("extends|implements"),
		ACTION("return|continue|break|throw|new|assert|strictfp"),
		CONSTRUCT_BLOCK("if|else|do|while|switch|case:|default|for|try|catch|finally"),
		CONSTRUCT_TYPE_DEF("class|interface|enum"),
		SPECIAL_ORGANIZATION("package"),
		SPECIAL_RESERVED("goto|const"),
		SPECIAL_IMPORT("import"),
		REFERNCE("[a-zA-Z]+[a-zA-Z\\d]*"),
		UNSUPPORTED(LITERAL_LONG, LITERAL_FLOAT, LITERAL_DOUBLE, LITERAL_CHAR,
				LITERAL_STRING, "\\/", MODIFIER_INHERITENCE, SPECIAL_RESERVED,
				"try|catch|finally|enum|interface|assert|new|throw", SPECIAL_IMPORT,
				"instanceof|double|float|volitile|transient|synchronized|native|abstract|throws|:");
		
		public String regex;
		
		private Type(String regex)
		{
			this.regex = regex;
		}
		
		private Type(Object first, Object... objects)
		{
			StringBuilder regexBuilder = new StringBuilder();
			regexBuilder.append("(");
			if (first instanceof Type)
				regexBuilder.append(((Type) first).regex);
			else
				regexBuilder.append(first.toString());
			regexBuilder.append(")");
			
			for (Object object : objects)
			{
				regexBuilder.append("|(");
				if (object instanceof Type)
					regexBuilder.append(((Type) object).regex);
				else
					regexBuilder.append(object.toString());
				regexBuilder.append(")");
			}
			
			this.regex = regexBuilder.toString();
		}
		
		private Type(String[] strings)
		{
			this(strings[0], Arrays.copyOfRange(strings, 1, strings.length));
		}
		
		public boolean matches(String token)
		{
			return token.matches(regex);
		}
		
		public static String compoundRegex(Object first, Object... objects)
		{
			StringBuilder regexBuilder = new StringBuilder();
			
			regexBuilder.append("(");
			if (first instanceof Type)
				regexBuilder.append(((Type) first).regex);
			else
				regexBuilder.append(first.toString());
			regexBuilder.append(")");
			
			for (Object object : objects)
			{
				regexBuilder.append("|(");
				if (object instanceof Type)
					regexBuilder.append(((Type) object).regex);
				else
					regexBuilder.append(object.toString());
				regexBuilder.append(")");
			}
			
			return regexBuilder.toString();
		}
	}
	
	public static final String[] CONTROL_TOKENS = new String[] { "\\.", "\\(", "\\)",
			"\\{", "}", "\\[", "]", ";" };
	public static final Pattern STRING_LITERAL_PATTERN = Pattern
			.compile(Type.LITERAL_STRING.regex);
	
	private Type type;
	private String value;
	
	public static List<Token> makeTokens(List<String> strings) throws LexException
	{
		List<Token> tokens = new LinkedList<>();
		
		for (String string : strings)
		{
			List<Token> token = makeToken(string);
			
			if (token != null)
				tokens.addAll(token);
		}
		
		return tokens;
	}
	
	private static List<Token> makeToken(String string) throws LexException
	{
		if (string.trim().length() == 0)
			return null;
		
		try
		{
			Token token = new Token(string);
			return Collections.<Token> singletonList(token);
		}
		catch (LexException e)
		{
			if (e.getMessage().startsWith("Type not found for"))
			{
				String regex = Type.compoundRegex(Type.OPERATOR, Type.COMPARATOR);
				return makeTokens(splitAndRetain(string, regex));
			}
			else
			{
				throw e;
			}
		}
	}
	
	public Token(String token) throws LexException
	{
		token = token.trim();
		if (Type.UNSUPPORTED.matches(token))
			throw new LexException("Unsupported Token: " + token);
		
		this.value = token;
		for (Type type : Type.values())
		{
			if (type.matches(token))
			{
				this.type = type;
				break;
			}
		}
		
		if (type == null)
			throw new LexException("Type not found for: " + token);
	}
	
	public String toString()
	{
		return type + " " + value;
	}
	
	public Type getType()
	{
		return type;
	}
	
	public void setType(Type type)
	{
		this.type = type;
	}
	
	public String getValue()
	{
		return value;
	}
	
	public void setValue(String value)
	{
		this.value = value;
	}
	
	private static List<String> splitAndRetain(String line, String regex)
	{
		if (line.trim().length() == 0)
			return Collections.<String> emptyList();
		
		Pattern pattern = Pattern.compile(regex);
		Matcher matcher = pattern.matcher(line);
		List<String> subStrings = new ArrayList<>();
		String[] tokens = line.split(regex);
		
		if (tokens.length == 0)
		{
			if (matcher.find())
				subStrings.add(matcher.group());
			return subStrings;
		}
		else
		{
			for (int index = 0; index < tokens.length; index++)
			{
				String subString = tokens[index].trim();
				if (subString.length() > 0)
					subStrings.add(subString);
				if (matcher.find())
					subStrings.add(matcher.group());
			}
		}
		
		return subStrings;
	}
}
