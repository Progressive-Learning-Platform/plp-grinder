package edu.asu.plp.compile.parser.constructs;

import edu.asu.plp.compile.parser.tree.ParseNode;

public class MethodConstruct
{
	private Signature signature;
	private ParseNode body;
	
	public MethodConstruct(Signature signature)
	{
		super();
		this.signature = signature;
	}
	
	public Signature getSignature()
	{
		return signature;
	}
	
	public void setSignature(Signature signature)
	{
		this.signature = signature;
	}
	
	public ParseNode getBody()
	{
		return body;
	}
	
	public void setBody(ParseNode body)
	{
		this.body = body;
	}
}
