public class BasicArithmatic
{
	private static final int constant = 10;
	
	public static int CasualMethod()
	{
		return 1;
	}
	
	public static int Sum(int a, int b)
	{
		return a + b;
	}
	
	public static void main(String[] args)
	{
		int a = 4;
		int b = 3;
		int sum = a + b;
		int product = a*b;
		int difference = a -b;
		int addImmediate = a + 2;
		int subImmediate = a - 2;
		int addConstant = b * constant;
		addConstant += 2;
		int multiplyImmediate = b * 0x05;
		int parenthesis = a + ( a + b );
		int pemdasFull = (a + b) * 3 / 5 - 10;
		int pemdasFullAlternate = a + b * (3 / (5 - 10));
		multiplyImmediate = multiplyImmediate << 1;
		multiplyImmediate++;
		
		
		CasualMethod();
		
		Sum(a, b);
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
