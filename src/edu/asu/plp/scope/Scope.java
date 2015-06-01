package edu.asu.plp.scope;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import edu.asu.plp.compile.parser.Variable;

public class Scope
{
	/** Maps source-variables to variable id */
	private Map<Variable, String> variables;
	private Scope parentScope;
	private List<Scope> children;
	
	public static Scope makeRootScope()
	{
		return new Scope(null);
	}
	
	private Scope(Scope parentScope)
	{
		this.children = new ArrayList<>();
		this.parentScope = parentScope;
		this.variables = new HashMap<>();
	}
	
	public Scope makeChild()
	{
		Scope child = new Scope(this);
		children.add(child);
		return child;
	}
	
	public boolean contains(Variable variable)
	{
		if (parentScope == null)
			return variables.containsKey(variable);
		else
			return parentScope.contains(variable) || variables.containsKey(variable);
	}
	
	public String getIDof(Variable variable)
	{
		if (variables.containsKey(variable))
			return variables.get(variable);
		else if (parentScope == null)
			throw new IllegalArgumentException("Variable is not in scope");
		else
			return parentScope.getIDof(variable);
	}
	
	public void addVariable(Variable variable)
	{
		if (this.contains(variable))
			throw new IllegalArgumentException("Variable already in scope: " + variable);
		else
			variables.put(variable, null);
	}
}
