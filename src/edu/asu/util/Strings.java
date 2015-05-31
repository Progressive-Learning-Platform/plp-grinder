package edu.asu.util;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Strings
{
	public static List<String> splitAndRetain(String line, String regex)
	{
		if (line.trim().length() == 0)
			return Collections.<String> emptyList();
		
		List<String> subStrings = new ArrayList<>();
		Pattern pattern = Pattern.compile(regex);
		Matcher matcher = pattern.matcher(line);
		int charIndex = 0;
		
		while (matcher.find())
		{
			String nonToken = line.substring(charIndex, matcher.start()).trim();
			String token = line.substring(matcher.start(), matcher.end()).trim();
			charIndex = matcher.end();
			
			if (nonToken.length() > 0)
				subStrings.add(nonToken);
			if (token.length() > 0)
				subStrings.add(token);
		}
		
		String token = line.substring(charIndex, line.length()).trim();
		if (token.length() > 0)
			subStrings.add(token);
		
		return subStrings;
	}
}
