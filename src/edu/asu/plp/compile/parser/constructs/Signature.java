package edu.asu.plp.compile.parser.constructs;

import java.util.ArrayList;
import java.util.List;

public class Signature
{
	private List<String> exceptions;
	private List<Variable> arguments;
	private String methodName;
	private String returnType;
	private String accessModifier;
	private boolean isStatic;
	private List<String> genericArguments;
	
	public Signature()
	{
		this.exceptions = new ArrayList<>();
		this.arguments = new ArrayList<>();
		this.genericArguments = new ArrayList<>();
	}
	
	public Signature(String methodName, String returnType, String accessModifier)
	{
		this();
		this.methodName = methodName;
		this.returnType = returnType;
		this.accessModifier = accessModifier;
	}

	public void addException(String exception)
	{
		exceptions.add(exception);
	}
	
	public void addArgument(Variable argument)
	{
		arguments.add(argument);
	}
	
	public void addArgument(String type, String name, boolean constant)
	{
		addArgument(new Variable(type, name, constant));
	}
	
	public void addArgument(String type, String name)
	{
		addArgument(new Variable(type, name));
	}
	
	public void addGenericArgument(String genericArgument)
	{
		genericArguments.add(genericArgument);
	}

	public List<String> getExceptions()
	{
		return exceptions;
	}

	public void setExceptions(List<String> exceptions)
	{
		this.exceptions = exceptions;
	}

	public List<Variable> getArguments()
	{
		return arguments;
	}

	public void setArguments(List<Variable> arguments)
	{
		this.arguments = arguments;
	}

	public String getMethodName()
	{
		return methodName;
	}

	public void setMethodName(String methodName)
	{
		this.methodName = methodName;
	}

	public String getReturnType()
	{
		return returnType;
	}

	public void setReturnType(String returnType)
	{
		this.returnType = returnType;
	}

	public String getAccessModifier()
	{
		return accessModifier;
	}

	public void setAccessModifier(String accessModifier)
	{
		this.accessModifier = accessModifier;
	}

	public boolean isStatic()
	{
		return isStatic;
	}

	public void setStatic(boolean isStatic)
	{
		this.isStatic = isStatic;
	}

	public List<String> getGenericArguments()
	{
		return genericArguments;
	}

	public void setGenericArguments(List<String> genericArguments)
	{
		this.genericArguments = genericArguments;
	}
}
