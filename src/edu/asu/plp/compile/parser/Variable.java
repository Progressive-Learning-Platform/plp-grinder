package edu.asu.plp.compile.parser;

public class Variable
{
	private String type;
	private String name;
	private boolean constant;
	
	@Override
	public int hashCode()
	{
		return name.hashCode();
	}
	
	public Variable(String type, String name)
	{
		super();
		this.type = type;
		this.name = name;
		this.constant = false;
	}
	
	public Variable(String type, String name, boolean constant)
	{
		super();
		this.type = type;
		this.name = name;
		this.constant = constant;
	}
	
	public String getType()
	{
		return type;
	}
	
	public String getName()
	{
		return name;
	}
	
	public void setName(String name)
	{
		this.name = name;
	}
	
	public boolean isConstant()
	{
		return constant;
	}
}
