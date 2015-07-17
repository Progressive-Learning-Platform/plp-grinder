public class BasicArithmatic
{
	private static class StaticClass
	{
		private BasicArithmatic arithmatic;
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
		StaticClass staticClassExample = new StaticClass();
		staticClassExample.aNumber = 20;
		StaticClass staticClassinstance2 = new StaticClass();
		staticClassinstance2.aNumber = 10;
		StaticClass staticClassinstance3 = new StaticClass();
		staticClassinstance3.aNumber = 50;

		int first = staticClassExample.aNumber + staticClassinstance2.aNumber + staticClassinstance3.aNumber;
		int a = staticClassExample.aNumber - 16;
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
