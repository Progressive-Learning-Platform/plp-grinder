package edu.asu.plp.scope;

import java.util.HashMap;
import java.util.Map;

public class Scope
{
	/** Maps source-variable names to variable id */
	private Map<String, String> variables;
	private Scope parentScope;
	
	public static Scope makeRootScope()
	{
		return new Scope(null);
	}
	
	private Scope(Scope parentScope)
	{
		this.parentScope = parentScope;
		this.variables = new HashMap<>();
	}
	
	public Scope makeChild()
	{
		return new Scope(this);
	}
	
	public boolean contains(String variable)
	{
		if (parentScope == null)
			return variables.containsKey(variable);
		else
			return parentScope.contains(variable) || variables.containsKey(variable);
	}
	
	public String getIDof(String variable)
	{
		if (variables.containsKey(variable))
			return variables.get(variable);
		else if (parentScope == null)
			throw new IllegalArgumentException("Variable is not in scope");
		else
			return parentScope.getIDof(variable);
	}
	
	public void addVariable(String variable)
	{
		if (this.contains(variable))
			throw new IllegalArgumentException("Variable already in scope: " + variable);
		else
			variables.put(variable, null);
	}
}
