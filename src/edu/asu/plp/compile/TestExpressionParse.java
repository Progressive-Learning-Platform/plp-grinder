package edu.asu.plp.compile;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.ListIterator;

import edu.asu.plp.Token;
import edu.asu.plp.Token.Type;
import edu.asu.plp.compile.TestExpressionParse.Tree.NodeType;
import edu.asu.plp.compile.lex.LexException;
import edu.asu.plp.compile.lex.Lexer;

public class TestExpressionParse
{
	//TODO non right hand sides such as a++;
	//TODO add int a = 1 + 2; initialization support
	
	static List<Token> tokens;
	static ListIterator tokenIterator;
	
	static Token currentToken;
	
	static Tree leftNode = null;
	static Tree rightNode = null;
	static Tree operatorNode = null;
	static Tree treeNode = null;
	
	public static void main(String[] args) throws FileNotFoundException, LexException
	{	
		File inputFile = new File("sampleData/BasicExpression.java");
		File dumpFile = new File("sampleData/BasicExpression.java.PREPROCESS");
		
		Lexer lexer = new Lexer(inputFile);
		lexer.dumpPreprocessData(dumpFile);
		
		tokens = lexer.lex();
		tokenIterator = tokens.listIterator();
		
		System.out.println(parseOrder());
		//parseOrder();
	}
	
	private static Tree parseOrder()
	{
		LinkedList<Tree> nodes = new LinkedList<>();
		nullifyNodes();
		getNextToken();
		
		while(currentToken != null)
		{
			if(currentToken.getValue().equals("("))
			{
				Tree tempLeft = null;
				Tree tempOperator = null;
				Tree tempRight = null;
				Tree tempTree = null;
				
				if(leftNode != null)
					tempLeft = leftNode.clone();
				if(operatorNode != null)
					tempOperator = operatorNode.clone();
				if(rightNode != null)
					tempRight = rightNode.clone();
				if(treeNode != null)
					tempTree = treeNode.clone();
				
				Tree parenthesisNode = parseParenthesis();
				
				leftNode = tempLeft;
				operatorNode = tempOperator;
				rightNode = tempRight;
				treeNode = tempTree;
				
				if(leftNode == null)
				{
					//System.out.println("LEFT NULL PARENTHESIS: " + parenthesisNode);
					leftNode = parenthesisNode.clone();
				}
				else if(rightNode == null)
				{
					//System.out.println("RIGHT NULL PARENTHESIS: " + parenthesisNode);
					treeNode.rightChild = parenthesisNode.clone();
					
					nodes.add(treeNode);
					
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
			else if(currentToken.getType() == Type.OPERATOR)
			{
				if(currentToken.getValue().equals("+") || currentToken.getValue().equals("-"))
				{
					operatorNode = new Tree(NodeType.OPERATOR);
					operatorNode.opOrLitOrRef = currentToken.getValue();
					
					if(leftNode != null)
						nodes.add(leftNode.clone());
					nodes.add(operatorNode.clone());
					
					nullifyNodes();
				}
				else if(currentToken.getValue().equals("*"))
				{
					operatorNode = new Tree(NodeType.OPERATOR);
					operatorNode.opOrLitOrRef = currentToken.getValue();
					
					treeNode = new Tree(NodeType.TREE);
					treeNode.leftChild = leftNode;
					treeNode.opOrLitOrRef = currentToken.getValue();
				}
			}
			else if(currentToken.getValue().equals(";"))
			{
				if(leftNode != null)
					nodes.add(leftNode);
			}
			else
			{
				if(leftNode == null)
				{
					if(currentToken.getType() == Type.REFERNCE)
					{
						leftNode = new Tree(NodeType.REFERENCE);
					}
					//Its a literal
					else
					{
						leftNode = new Tree(NodeType.LITERAL);
					}
					leftNode.opOrLitOrRef = currentToken.getValue();
				}
				else if(rightNode == null)
				{
					if(currentToken.getType() == Type.REFERNCE)
					{
						rightNode = new Tree(NodeType.REFERENCE);
					}
					//Its a literal
					else
					{
						rightNode = new Tree(NodeType.LITERAL);
					}
					rightNode.opOrLitOrRef = currentToken.getValue();
					
					treeNode.rightChild = rightNode.clone();
					nodes.add(treeNode);
					nullifyNodes();
				}
			}
			
			getNextToken();
		}
		
		return assemble(nodes);
	}
	
	private static void nullifyNodes()
	{
		leftNode = null;
		rightNode = null;
		operatorNode = null;
		treeNode = null;
	}

	private static Tree parseParenthesis()
	{
		return parseOrder();
	}
	
	private static Tree assemble(LinkedList<Tree> nodes)
	{
		//System.out.println(nodes);
		Tree returnTree = null;
		
		if(nodes.size() == 0)
		{
			//System.out.println("Arrange 0");
			return returnTree;
		}
		else if(nodes.size() == 1)
		{
			//System.out.println("Arrange 1");
			returnTree = nodes.get(0);
		}
		else if(nodes.size() == 3)
		{
			returnTree = new Tree(NodeType.TREE);
			returnTree.leftChild = nodes.get(0);
			returnTree.opOrLitOrRef = nodes.get(1).opOrLitOrRef;
			returnTree.rightChild = nodes.get(2);
			//System.out.println("3: " + returnTree.leftChild);
		}
		else
		{
			ArrayList<Tree> holderTrees = new ArrayList<>();
			
			//System.out.println(nodes);
			//Loop for multiplication first
			for(int index = 0; index < nodes.size(); index++)
			{
				if(nodes.get(index).type == NodeType.OPERATOR)
				{
					if(nodes.get(index).opOrLitOrRef.equals("*"))
					{
						Tree temporary = new Tree(NodeType.TREE);
						holderTrees.remove(index - 1);
						temporary.opOrLitOrRef = nodes.get(index).opOrLitOrRef;
						temporary.leftChild = nodes.get(index - 1);
						temporary.rightChild = nodes.get(index + 1);
						holderTrees.add(temporary);
						
						if(index + 2 < nodes.size())
						{
							index = index + 1;
						}
					}
					else
					{
						holderTrees.add(nodes.get(index));
					}
				}
				else
				{
					holderTrees.add(nodes.get(index));
				}
			}
			
			//Loop for addition | subtraction
			for(int index = 0; index < holderTrees.size(); index++)
			{
				if(holderTrees.get(index).type == NodeType.OPERATOR)
				{
					if(holderTrees.get(index).opOrLitOrRef.equals("+") || holderTrees.get(index).opOrLitOrRef.equals("-"))
					{
						if(returnTree == null)
						{
							returnTree = new Tree(NodeType.TREE);
							returnTree.opOrLitOrRef = holderTrees.get(index).opOrLitOrRef;
							returnTree.leftChild = holderTrees.get(index - 1);
							returnTree.rightChild = holderTrees.get(index + 1);
						}
						else
						{
							Tree temporary = returnTree.clone();
							returnTree = new Tree(NodeType.TREE);
							returnTree.opOrLitOrRef = holderTrees.get(index).opOrLitOrRef;
							returnTree.leftChild = temporary;
							returnTree.rightChild = holderTrees.get(index + 1);
						}
						
						if(index + 2 < holderTrees.size())
							index = index + 1;
					}
				}
			}
		}
		//System.out.println("RETURN: " + returnTree);
		return returnTree;
	}
	
	public static class Tree
	{
		public static enum NodeType
		{
			TREE,
			REFERENCE,
			OPERATOR,
			LITERAL;
		}
		
		String opOrLitOrRef;
		public Tree leftChild;
		public Tree rightChild;

		public NodeType type;
		
		public Tree(NodeType type)
		{
			this.type = type;
			leftChild = null;
			rightChild = null;
			opOrLitOrRef = null;
		}
		
		public Tree clone()
		{
			Tree temporary = new Tree(type);
			temporary.opOrLitOrRef = opOrLitOrRef;
			temporary.leftChild = (leftChild == null) ? null : leftChild.clone();
			temporary.rightChild = (rightChild == null) ? null : rightChild.clone();
			return temporary;
		}
		
		public String toString()
		{
			return "OP: " + opOrLitOrRef + ((leftChild != null) ? (" LEFT: " + leftChild.toString() + ((rightChild != null) ? " RIGHT: " +  rightChild.toString() + " ": " RIGHT: NULL ")) : " LEFT: NULL ");
		}
	}
	
	private static void getNextToken()
	{
		if(tokenIterator.hasNext())
			currentToken = (Token) tokenIterator.next();
		else
			currentToken = null;
	}
	
	private static void ungetToken()
	{
		currentToken = (Token) tokenIterator.previous();
	}
}
