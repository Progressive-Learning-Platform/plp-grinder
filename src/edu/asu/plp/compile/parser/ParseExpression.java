package edu.asu.plp.compile.parser;

import java.util.LinkedList;
import java.util.List;
import java.util.ListIterator;

import edu.asu.plp.Token;
import edu.asu.plp.Token.Type;
import edu.asu.plp.compile.parser.support.TreeSupport;
import edu.asu.plp.compile.parser.tree.NodeType;
import edu.asu.plp.compile.parser.tree.OperatorNode;
import edu.asu.plp.compile.parser.tree.ParseNode;
import edu.asu.plp.compile.parser.tree.SequenceNode;
import edu.asu.plp.compile.parser.tree.ValueNode;

public class ParseExpression
{
	private LinkedList<Token> tokens;
	private ListIterator<Token> tokenIterator;
	private Token currentToken;
	
	private ParseNode leftNode;
	private ParseNode rightNode;
	private ParseNode operatorNode;

	
	public ParseExpression(List<Token> sequenceTokens)
	{
		tokens = (LinkedList<Token>) sequenceTokens;
		tokenIterator = tokens.listIterator();
		
		leftNode = null;
		rightNode = null;
		operatorNode = null;
	}
	
	public SequenceNode parse() throws ParseException
	{
		SequenceNode node = parseExpression();
		
		//TreeSupport.printTree(node);
		
		return node;
	}
	
	private SequenceNode parseExpression() throws ParseException
	{
		LinkedList<ParseNode> nodes = new LinkedList<>();
		nullifyNodes();
		getNextToken();
		
		while(currentToken != null)
		{
			//System.out.println(currentToken.getValue() + " / " + currentToken.getType());
			if(currentToken.getValue().equals("("))
			{
				ParseNode tempLeft = null;
				ParseNode tempRight = null;
				ParseNode tempOperator = null;
				ParseNode tempTree = null;
				
				if(leftNode != null)
					tempLeft = TreeSupport.clone(leftNode);
				if(operatorNode != null)
					tempOperator =  TreeSupport.clone(operatorNode);
				if(rightNode != null)
					tempRight =  TreeSupport.clone(rightNode);
				
				SequenceNode parenthesisNode = parseBlock();
				
				leftNode = tempLeft;
				operatorNode = tempOperator;
				rightNode = tempRight;
				
				if(leftNode == null)
				{
					//System.out.println("LEFT NULL PARENTHESIS: " + parenthesisNode);
					leftNode = TreeSupport.clone(parenthesisNode);
				}
				else if(rightNode == null)
				{
					//System.out.println("RIGHT NULL PARENTHESIS: " + parenthesisNode);
					TreeSupport.insertChildAt(operatorNode.getChildren(), 1, TreeSupport.clone(parenthesisNode));
					
					nodes.add(operatorNode);
					
					nullifyNodes();
				}
			}
			else if(currentToken.getValue().equals(")"))
			{
				if(leftNode != null)
				{
					nodes.add(leftNode);
					nullifyNodes();
				}
				break;
			}
			else if(currentToken.getType() == Type.OPERATOR_BINARY)
			{
				if(currentToken.getValue().equals("+") || currentToken.getValue().equals("-") || !currentToken.getValue().equals("*"))
				{
					operatorNode = new OperatorNode(currentToken.getValue());
					
					if(leftNode != null)
						nodes.add(TreeSupport.clone(leftNode));
					nodes.add(TreeSupport.clone(operatorNode));
					
					nullifyNodes();
				}
				else if(currentToken.getValue().equals("*"))
				{
					operatorNode = new OperatorNode(currentToken.getValue());
					if(leftNode != null)
						TreeSupport.insertChildAt(operatorNode.getChildren(), 0, leftNode);
				}
			}
			else if(currentToken.getValue().equals(";"))
			{
				if(leftNode != null)
					nodes.add(leftNode);
			}
			else if(currentToken.getType() == Type.ACTION)
			{
			}
			else
			{
				if(leftNode == null)
				{
					leftNode = new ValueNode(currentToken.getValue());
				}
				else if(rightNode == null)
				{
					rightNode = new ValueNode(currentToken.getValue());
					TreeSupport.insertChildAt(operatorNode.getChildren(), 1, TreeSupport.clone(rightNode));
					
					nodes.add(operatorNode);
					nullifyNodes();
				}
			}
			
			getNextToken();
		}
		
		return assemble(nodes);
	}

	private SequenceNode parseBlock() throws ParseException
	{
		return parseExpression();
	}
	
	private SequenceNode assemble(List<ParseNode> nodes) throws ParseException
	{
		SequenceNode tree = new SequenceNode();
		List<ParseNode> listOf = tree.getChildren();
		
		if(nodes.size() == 0)	{;}
		else if(nodes.size() == 1)
		{
			TreeSupport.insertChildAt(listOf, 0, nodes.get(0));
		}
		else if(nodes.size() == 3)
		{
			OperatorNode operator = (OperatorNode) nodes.get(1);
			TreeSupport.insertChildAt(operator.getChildren(), 0, nodes.get(0));
			TreeSupport.insertChildAt(operator.getChildren(), 1, nodes.get(2));
			TreeSupport.insertChildAt(listOf, 0, operator);
		}
		else
		{
			
		}
		
		return tree;
	}
	
	private void getNextToken()
	{
		if(tokenIterator.hasNext())
			currentToken = (Token) tokenIterator.next();
		else
			currentToken = null;
	}
	
	private void nullifyNodes()
	{
		leftNode = null;
		rightNode = null;
		operatorNode = null;
	}
}
