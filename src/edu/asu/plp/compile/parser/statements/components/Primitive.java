package edu.asu.plp.compile.parser.statements.components;

public class Primitive
{
	public enum PrimitiveType
	{
		BOOLEAN,
		LONG,
		INT,
		BYTE,
		SHORT,
		CHAR,
		DOUBLE,
		FLOAT;
	}
	
	private PrimitiveType type;
	
	private boolean boolValue;
	private long longValue;
	private int intValue;
	private byte byteValue;
	private short shortValue;
	private char charValue;
	private double doubleValue;
	private float floatValue;
	
	public Primitive(PrimitiveType type, boolean bool)
	{
		this(type);
		this.boolValue = bool;
	}

	public Primitive(PrimitiveType type, long longVal)
	{
		this(type);
		this.longValue = longVal;
	}

	public Primitive(PrimitiveType type, int intVal)
	{
		this(type);
		this.intValue = intVal;
	}
	
	public Primitive(PrimitiveType type, byte byteVal)
	{
		this(type);
		this.byteValue = byteVal;
	}
	
	public Primitive(PrimitiveType type, short shortVal)
	{
		this(type);
		this.shortValue = shortVal;
	}

	public Primitive(PrimitiveType type, char charVal)
	{
		this(type);
		this.charValue = charVal;
	}

	public Primitive(PrimitiveType type, double doubleVal)
	{
		this(type);
		this.doubleValue = doubleVal;
	}

	public Primitive(PrimitiveType type, float floatVal)
	{
		this(type);
		this.floatValue = floatVal;
	}
	
	private Primitive(PrimitiveType type)
	{
		this.type = type;
	}
	
	public PrimitiveType getType()
	{
		return type;
	}

	public boolean isBoolValue()
	{
		return boolValue;
	}

	public long getLongValue()
	{
		return longValue;
	}

	public int getIntValue()
	{
		return intValue;
	}

	public byte getByteValue()
	{
		return byteValue;
	}

	public short getShortValue()
	{
		return shortValue;
	}

	public char getCharValue()
	{
		return charValue;
	}

	public double getDoubleValue()
	{
		return doubleValue;
	}

	public float getFloatValue()
	{
		return floatValue;
	}
	
}
