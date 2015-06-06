package edu.asu.plp.compile.lex;

import java.util.Comparator;
import java.util.regex.Matcher;

public class MatchSorter implements Comparator<Matcher>
{
	@Override
	public int compare(Matcher arg0, Matcher arg1)
	{
		return arg0.start() - arg1.start();
	}
}
