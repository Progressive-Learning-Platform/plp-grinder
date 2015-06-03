package edu.asu.plp.compile.parser.statements.components;


public class ValueNode
{
	String id;
	Primitive primitiveValue;
	boolean isVar;
	
	public ValueNode(String id)
	{
		this(id, null, true);
	}
	
	public ValueNode(Primitive primitiveValue)
	{
		this(null, primitiveValue, false);
	}
	
	private ValueNode(String id, Primitive primitiveValue, boolean isVar)
	{
		this.id = id;
		this.primitiveValue = primitiveValue;
		this.isVar = isVar;
	}

	public String getId()
	{
		return id;
	}

	public Primitive getPrimitiveValue()
	{
		return primitiveValue;
	}

	public boolean isVar()
	{
		return isVar;
	}
	
}
