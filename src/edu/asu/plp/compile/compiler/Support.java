package edu.asu.plp.compile.compiler;

public class Support
{
	public static int incrementBetween(int start, int end, int current)
	{
		return incrementBetween(start, end, current, 1);
	}
	
	/**
	 * 
	 * @param start
	 *            inclusive
	 * @param end
	 *            inclusive
	 * @param current
	 *            number to change
	 * @param amount
	 *            amount to change currentNumber
	 * @return
	 */
	public static int incrementBetween(int start, int end, int current,
			int amount)
	{
		if (current + amount > end)
		{
			int overAmount = ((current + amount) - end) - 1;
			current = start + overAmount;
		}
		else if (current + amount < start)
		{
			int underAmount = (start - (current + amount)) - 1;
			current = end + underAmount;
		}
		else
		{
			current += amount;
		}
		return current;
	}
	
	public static boolean arrayContains(String[] array, String value)
	{
		for (String string : array)
		{
			if (string.equals(value))
				return true;
		}
		return false;
	}
	
	public static boolean isNumber(String s)
	{
		try
		{
			Integer.parseInt(s);
		}
		catch(Exception e)
		{
			return false;
		}
		return true;
	}
	
	/**
	 * 
	 * @param s Assumed not a hex number
	 * @return
	 */
	public static boolean isInteger(String s)
	{
		return isInteger(s, 10);
	}
	
	public static boolean isInteger(String s, int radix)
	{
		if (s.isEmpty())
			return false;
		for (int i = 0; i < s.length(); i++)
		{
			if (i == 0 && s.charAt(i) == '-')
			{
				if (s.length() == 1)
					return false;
				else
					continue;
			}
			if (Character.digit(s.charAt(i), radix) < 0)
				return false;
		}
		return true;
	}
}
