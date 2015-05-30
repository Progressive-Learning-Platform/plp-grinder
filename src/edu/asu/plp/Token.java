package edu.asu.plp;


public class Token
{
	public static enum Type
	{
		CONTROL("\\.", "\\(", "\\)", "\\{", "\\}", "\\[", "\\]", ";"),
		LITERAL_INT("(\\d)+"),
		LITERAL_LONG("(\\d)+[lL]"),
		LITERAL_FLOAT("(((\\d+)?(\\.\\d+))|((\\d+)(\\.\\d+)?))[fF]"),
		LITERAL_DOUBLE("(\\d+)?(\\.\\d+)"),
		LITERAL_CHAR("'[a-z A-Z]?'"),
		LITERAL_STRING("\"([^\"\\\\\\n\\r]|\\\\.)*\""),
		LITERAL_BOOLEAN("true|false"),
		LITERAL_NULL("null"),
		OPERATOR("((\\+|-|\\/|\\*|\\||&)=?)|="),
		COMPARATOR(">|>=|<|<=|&&|\\|\\||==|instanceof"),
		TYPE("boolean|long|int|byte|short|char|double|float|void"),
		MODIFIER_ACCESS("public|private|protected|static"),
		MODIFIER_BEHAVIOUR("final|volitile|transient|synchronized|native|abstract|throws"),
		MODIFIER_INHERITENCE("extends|implements"),
		ACTION("return|continue|break|throw|new|assert|strictfp"),
		CONSTRUCT_BLOCK("if|else|do|while|switch|case|default|for|try|catch|finally"),
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
		
		public boolean matches(String token)
		{
			return token.matches(regex);
		}
	}
}
