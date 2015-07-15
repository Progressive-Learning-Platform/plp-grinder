public class BasicArithmatic
{
	private static class StaticClass
	{
		private int aNumber = 1;
		private static int aSecondNumber = 2;
	}

	private static int constant = 10;

	public static int casualMethod()
	{
		return 1;
	}

	public static void main(String args)
	{
		StaticClass test = new StaticClass();
		int a = 4;
		int b = 2 + casualMethod();
		if (true)
		{
			b += 1;
		}
		int c = constant;
		if (false)
		{
			c += 89;
		}
		else
		{
			c += 1;
		}
		int sum = a + b;
		int product = a*b;
		int difference = a -b;
		int addImmediate = a + 2;
		int subImmediate = a - 2;
		//int addConstant = b * constant;
		//addConstant += 2;
		int multiplyImmediate = b * 0x05;
		int parenthesis = a + ( a + b );
		int pemdasFull = (a + b) * 3 - 10;
		int pemdasFullAlternate = a + b * (3 - 10);
		int multipleParenthis = 1 + (a + (b - 5) * 2) * 7;
		//multiplyImmediate = multiplyImmediate << 1;
		//multiplyImmediate++;
		//boolean test = true | false;


		//CasualMethod();

		//Sum(a, b);
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
