public class BasicArithmatic
{
	private static final int constant = 10;
	
	public static void main(String[] args)
	{
		int a = 4;
		int b = 3;
		int sum = a + b;
		int product = a * b;
		int difference = a - b;
		int addImmediate = a + 2;
		int subImmediate = a - 2;
		int addConstant = b * constant;
		int multiplyImmediate = b * 5;
		// DO NOT LEX
		/* Do not lex */
		/*
		 * Do not lex 2
		 */
		// /**/ Do not lex 3
		/*
		 * 
		 * // */
		// Strings are currently unsupported
		// String s = "/*LexAstring" + lex + "LexAstring*/ LexAstring";
	}
}
