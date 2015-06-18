package edu.asu.plp.compile.parser.support;

import java.util.List;

import edu.asu.plp.compile.parser.ParseException;
import edu.asu.plp.compile.parser.tree.DeclarationNode;
import edu.asu.plp.compile.parser.tree.NodeType;
import edu.asu.plp.compile.parser.tree.OperatorNode;
import edu.asu.plp.compile.parser.tree.ParseNode;
import edu.asu.plp.compile.parser.tree.SequenceNode;
import edu.asu.plp.compile.parser.tree.ValueNode;

public class TreeSupport
{
	public static void insertChildAt(List<ParseNode> childList, int index, ParseNode insert) throws ParseException
	{
		if(index <= childList.size())
		{
			childList.add(index, insert);
		}
		else
		{
			throw new ParseException();
		}
		
	}
	
	public static void insertChildOver(List<ParseNode> childList, int index, ParseNode insert) throws ParseException
	{
		if(index <= childList.size())
		{
			ParseNode childNode = childList.get(index);
			childNode = insert;
		}
		else
		{
			throw new ParseException();
		}
		
	}
	
	public static ParseNode clone(ParseNode node)
	{
		ParseNode returnNode = null;
		List<ParseNode> nodes;
		
		switch(node.getType())
		{
			case VALUE:
				returnNode = new ValueNode(node.getValue());
				break;
			case OPERATOR:
				returnNode = new OperatorNode(node.getValue());
				nodes = returnNode.getChildren();
				for(ParseNode nodeIterator : node.getChildren())
				{
					nodes.add(nodeIterator);
				}
				break;
			case SEQUENCE:
				returnNode = new SequenceNode();
				nodes = returnNode.getChildren();
				for(ParseNode nodeIterator : node.getChildren())
				{
					nodes.add(nodeIterator);
				}
				break;
		}
		
		return returnNode;
	}
	
	public static void printTree(SequenceNode node)
	{
		for(ParseNode nodeIt : node.getChildren())
		{
			if(nodeIt.getType() == NodeType.OPERATOR)
			{
				printOperator(nodeIt);
			}
			else if(nodeIt.getType() == NodeType.SEQUENCE)
			{
				printTree((SequenceNode)nodeIt);
			}
			else if(nodeIt.getType() == NodeType.VALUE)
			{
				System.out.println("Value Node");
				System.out.println(nodeIt.getValue());
			}
			else if(nodeIt.getType() == NodeType.DECLARATION)
			{
				printDeclaration((DeclarationNode)nodeIt);
			}
		}
	}
	
	public static void printOperator(ParseNode node)
	{
		System.out.println("OPERATOR: " + node.getValue());
		ParseNode leftChild = node.getChildren().get(0);
		ParseNode rightChild = node.getChildren().get(1);
		
		if(leftChild.getType() == NodeType.VALUE)
		{
			System.out.println(node.getValue() + " Left: " + leftChild.getValue());
		}
		else if(leftChild.getType() == NodeType.OPERATOR)
		{
			System.out.print(node.getValue() + " Left-> ");
			printOperator(leftChild);
		}
		else if(leftChild.getType() == NodeType.SEQUENCE)
		{
			System.out.print(node.getValue() + " Left-> ");
			printTree((SequenceNode)leftChild);
		}
		
		if(rightChild.getType() == NodeType.VALUE)
		{
			System.out.println(node.getValue() + " Right: " + rightChild.getValue());
		}
		else if(rightChild.getType() == NodeType.OPERATOR)
		{
			System.out.print(node.getValue() + " Right-> ");
			printOperator(rightChild);
		}
		else if(rightChild.getType() == NodeType.SEQUENCE)
		{
			System.out.print(node.getValue() + " Right-> ");
			printTree((SequenceNode)rightChild);
		}
	}

	public static void printDeclaration(DeclarationNode node)
	{
		System.out.print("DECLARATION: ");
		for(ParseNode value : node.getChildren())
		{
			System.out.print(value.getValue() + " ");
		}
		System.out.println();
	}
}
