package edu.asu.plp.compile.compiler;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.PrintWriter;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;

import edu.asu.plp.Token;
import edu.asu.plp.compile.lex.LexException;
import edu.asu.plp.compile.lex.Lexer;
import edu.asu.plp.compile.parser.ParseException;
import edu.asu.plp.compile.parser.constructs.ClassConstruct;
import edu.asu.plp.compile.parser.constructs.MethodConstruct;
import edu.asu.plp.compile.parser.constructs.Variable;
import edu.asu.plp.compile.parser.support.TreeSupport;
import edu.asu.plp.compile.parser.tree.DeclarationNode;
import edu.asu.plp.compile.parser.tree.NodeType;
import edu.asu.plp.compile.parser.tree.ParseNode;
import edu.asu.plp.compile.parser.tree.SequenceNode;

public class PlpCompiler
{
	// TODO: Change to a list of classes. Temporary fix for presentation to
	// group
	private ClassConstruct mainClass;
	private Map<String, Integer> typeSize; // Type size in bits
	private Map<Integer, String> registers;
	private Map<String, String> commands;
	private final String[] operators = { "=", "+=", "+", "-", "*", "<<", ">>",
			"|" };
	private final String newLine = "\n";
	private final String tab = "\t";
	private int spaceRequired;
	int registerCap = 17;
	int registerFloor = 4;
	
	public PlpCompiler(ClassConstruct classConstruct)
	{
		mainClass = classConstruct;
		initializeTypeSizesAndRegisters();
		initializeStaticVariableSpace();
	}
	
	private void initializeStaticVariableSpace()
	{
		spaceRequired = 0;
		
		List<Variable> globals = mainClass.getGlobalVariables();
		
		for (Variable global : globals)
		{
			spaceRequired += typeSize.get(global.getType());
		}
	}
	
	public List<String> compile()
	{
		LinkedList<String> compiledPlp = new LinkedList<>();
		int starting = (int) Integer.parseInt("10000000", 16) + spaceRequired;
		
		compiledPlp
				.add(".org " + ("0x" + Long.toHexString(starting)) + newLine);
		if (mainClass.getMainMethod() != null)
		{
			compiledPlp.add(mainClass.getMainMethod().getSignature()
					.getMethodName()
					+ ":");
			convertMethod(mainClass.getMainMethod(), compiledPlp);
			
		}
		
		compiledPlp.add("");
		
		for (MethodConstruct method : mainClass.getClassMethods())
		{
			compiledPlp.add(method.getSignature().getMethodName() + ":");
			// convertMethod(method, compiledPlp);
			compiledPlp.add("");
		}
		
		for (MethodConstruct method : mainClass.getStaticMethods())
		{
			compiledPlp.add(method.getSignature().getMethodName() + ":");
			// convertMethod(method, compiledPlp);
			compiledPlp.add("");
		}
		
		return compiledPlp;
	}
	
	private void convertMethod(MethodConstruct mainMethod,
			LinkedList<String> compiledPlp)
	{
		HashMap<String, String> registerMap = new HashMap<>();
		int registerNumber = 4;
		
		System.out.println("\n\t\t\t STATEMENTS\n");
		for (ParseNode node : mainMethod.getBody().getChildren())
		{
			registerNumber = convertStatement((SequenceNode) node,
					registerNumber, registerMap, compiledPlp);
		}
	}
	
	private int convertStatement(SequenceNode node, int registerNumber,
			HashMap<String, String> registerMap, LinkedList<String> compiledPlp)
	{
		int currentNumber = registerNumber;
		List<String[]> executables = new LinkedList<>();
		for (ParseNode childNode : node.getChildren())
		{
			if (childNode.getType() == NodeType.DECLARATION)
			{
				if (!registerMap.containsKey(childNode.getChildren().get(1)
						.getValue()))
					registerMap.put(childNode.getChildren().get(1).getValue(),
							registers.get(currentNumber));
				currentNumber = Support.incrementBetween(registerFloor,
						registerCap, currentNumber);
			}
			else if (childNode.getType() == NodeType.OPERATOR)
			{
				convertOperator(childNode, executables);
			}
			else if (childNode.getType() == NodeType.SEQUENCE)
			{
				convertTree(childNode, executables);
			}
		}
		System.out.println("\n<--------------Executables------------->");
		for (String[] strings : executables)
		{
			for (String string : strings)
			{
				System.out.print(string + " ");
			}
			System.out.println();
		}
		
		for (int index = 0; index < executables.size(); index++)
		{
			String[] components = executables.get(index);
			String left = components[0];
			String operator = components[1];
			String right = components[2];
			String command = null;
			String globalLeft = "";
			String globalRight = "";
			boolean isLeftEmpty = false;
			boolean isRightEmpty = false;
			boolean isLeftOperator = false;
			boolean isRightOperator = false;
			boolean isLeftLiteral = false;
			boolean isRightLiteral = false;
			boolean isLeftGlobal = false;
			boolean isRightGlobal = false;
			
			// $t0 is left value
			// $t1 is right value
			// $t2 is assignedValue
			// $t3 is misc calculations
			
			if (left.equals(""))
				isLeftEmpty = true;
			if (right.equals(""))
				isRightEmpty = true;
			
			if (!isLeftEmpty && Support.arrayContains(operators, left))
				isLeftOperator = true;
			else if (!isRightEmpty && Support.arrayContains(operators, right))
				isRightOperator = true;
			
			if (!isLeftEmpty && !isLeftOperator)
			{
				isLeftLiteral = Support.isNumber(left);
			}
			if (!isRightEmpty && !isRightOperator)
			{
				isRightLiteral = Support.isNumber(right);
			}
			if (!isLeftLiteral && !isLeftOperator)
			{
				if (registerMap.get(left) == null)
				{
					isLeftGlobal = true;
					List<Variable> globals = mainClass.getGlobalVariables();
				}
			}
			if (!isRightLiteral && !isRightOperator)
			{
				if (registerMap.get(right) == null)
					isRightGlobal = true;
			}
			
			// Deal with equals
			if (!isLeftOperator && !isRightOperator && !isLeftEmpty
					&& !isRightEmpty)
			{
				if (!isLeftLiteral && !isRightLiteral)
				{
					command = commands.get(operator);
					if (operator.equals("|"))
					{
						boolean isLeftBoolean = false;
						boolean isRightBoolean = false;
						
						if (left.equals("false"))
						{
							compiledPlp.add(tab + "li $t0, 0");
							isLeftBoolean = true;
						}
						else if (left.equals("true"))
						{
							isLeftBoolean = true;
							compiledPlp.add(tab + "li $t0, 1");
						}
						
						if (right.equals("false"))
						{
							isRightBoolean = true;
							compiledPlp.add(tab + "li $t1, 0");
						}
						else if (right.equals("true"))
						{
							isRightBoolean = true;
							compiledPlp.add(tab + "li $t1, 1");
						}
						
						compiledPlp.add(tab
								+ command
								+ " "
								+ "$t2, "
								+ ((isLeftBoolean) ? "$t0" : registerMap
										.get(left))
								+ ", "
								+ ((isRightBoolean) ? "$t1" : registerMap
										.get(right)));
					}
					else
						compiledPlp.add(tab
								+ command
								+ " "
								+ "$t2"
								+ ", "
								+ registerMap.get(left)
								+ ", "
								+ ((!isRightGlobal) ? registerMap.get(right)
										: right));
				}
				else if (isLeftLiteral && !isRightLiteral)
				{
					if (operator.equals("+"))
						command = commands.get(operator + "L");
					else
						command = commands.get(operator);
					compiledPlp.add(tab + command + " " + "$t2, "
							+ registerMap.get(right) + ", " + left);
				}
				else if (isRightLiteral && !isLeftLiteral)
				{
					if (operator.equals("+"))
						command = commands.get(operator + "L");
					else
						command = commands.get(operator);
					if (command == null)
					{
						if (operator.equals("="))
							compiledPlp.add(tab + "li" + " "
									+ registerMap.get(left) + ", " + right);
						else if (operator.equals("+="))
							compiledPlp.add(tab + "addiu" + " "
									+ registerMap.get(left) + ", "
									+ registerMap.get(left) + ", " + right);
						else if (operator.equals("-="))
							compiledPlp.add(tab + "addiu" + " "
									+ registerMap.get(left) + ", "
									+ registerMap.get(left) + ", " + (right));
						else if (operator.equals("<<"))
							compiledPlp.add(tab + "sll $t2, "
									+ registerMap.get(left) + ", " + right);
						else if (operator.equals(">>"))
							compiledPlp.add("");
					}
					else
						compiledPlp.add(tab + command + " $t2, "
								+ registerMap.get(left) + ", " + right);
				}
				else if (isLeftLiteral && isRightLiteral)
				{
					if (operator.equals("+"))
						command = commands.get(operator + "L");
					else
						command = commands.get(operator);
					compiledPlp.add(tab + command + "  $t2, " + left + ", "
							+ right);
				}
			}
			else if (isLeftOperator)
			{
				if (isRightLiteral && operator.equals("+"))
					command = commands.get(operator + "L");
				else
					command = commands.get(operator);
				
				if (isRightLiteral)
					compiledPlp.add(tab + command + " " + "$t2, $t2, " + right);
				else
					compiledPlp.add(tab + command + " " + "$t2, $t2, "
							+ registerMap.get(right));
			}
			else if (isRightOperator)
			{
				command = commands.get(operator);
				if (command == null)
				{
					if (operator.equals("="))
						compiledPlp.add(tab + "li" + " "
								+ registerMap.get(left) + ", " + "$t2");
				}
				else
				{
					if (isLeftLiteral)
					{
						
					}
					else
					{
						// compiledPlp.add(tab);
					}
				}
				
			}
			else if (isLeftEmpty || isRightEmpty)
			{
				if ((isLeftLiteral || isRightLiteral) && operator.equals("+"))
					command = commands.get(operator + "L");
				else
					command = commands.get(operator);
				
				if (isLeftLiteral)
					compiledPlp.add(tab + command + " " + "$t2, $t2, " + left);
				else if (isRightLiteral)
					compiledPlp.add(tab + command + " " + "$t2, $t2, " + right);
				else if (isLeftEmpty)
					compiledPlp.add(tab + command + " " + "$t2, $t2, "
							+ registerMap.get(right));
				else if (isRightEmpty)
					compiledPlp.add(tab + command + " " + "$t2, $t2, "
							+ registerMap.get(left));
			}
		}
		return currentNumber;
	}
	
	private void convertTree(ParseNode childNode, List<String[]> executables)
	{
		for (ParseNode nodeIt : childNode.getChildren())
		{
			if (nodeIt.getType() == NodeType.OPERATOR)
			{
				convertOperator(nodeIt, executables);
			}
			else if (nodeIt.getType() == NodeType.SEQUENCE)
			{
				convertTree((SequenceNode) nodeIt, executables);
			}
			else if (nodeIt.getType() == NodeType.VALUE)
			{
				// System.out.println("Value Node");
				// System.out.println(nodeIt.getValue());
			}
		}
	}
	
	private void convertOperator(ParseNode childNode, List<String[]> executables)
	{
		// System.out.println("OPERATOR: " + childNode.getValue());
		ParseNode leftChild = childNode.getChildren().get(0);
		ParseNode rightChild = childNode.getChildren().get(1);
		
		if (leftChild.getType() == NodeType.VALUE)
		{
			// System.out.println(childNode.getValue() + " ValLeft: " +
			// leftChild.getValue());
		}
		else if (leftChild.getType() == NodeType.OPERATOR)
		{
			// System.out.print(childNode.getValue() + " OperLeft-> ");
			convertOperator(leftChild, executables);
		}
		else if (leftChild.getType() == NodeType.SEQUENCE)
		{
			// System.out.print(childNode.getValue() + " SeqLeft-> ");
			convertTree((SequenceNode) leftChild, executables);
		}
		
		if (rightChild.getType() == NodeType.VALUE)
		{
			// System.out.println(childNode.getValue() + " ValRight: " +
			// rightChild.getValue());
		}
		else if (rightChild.getType() == NodeType.OPERATOR)
		{
			// System.out.print(childNode.getValue() + " OperRight-> ");
			convertOperator(rightChild, executables);
		}
		else if (rightChild.getType() == NodeType.SEQUENCE)
		{
			// System.out.print(childNode.getValue() + " SeqRight-> ");
			convertTree((SequenceNode) rightChild, executables);
		}
		String[] strings = new String[3];
		strings[0] = leftChild.getValue();
		strings[1] = childNode.getValue();
		strings[2] = rightChild.getValue();
		executables.add(strings);
	}
	
	public static void main(String[] args) throws FileNotFoundException,
			LexException, ParseException
	{
		File inputFile = new File("sampleData/BasicArithmatic.java");
		File outputFile = new File(
				"sampleData/output/BasicArithmatic.java.lexed");
		File dumpFile = new File("sampleData/BasicArithmatic.java.PREPROCESS");
		File plpFile = new File("sampleData/main.asm");
		
		Lexer lexer = new Lexer(inputFile);
		lexer.dumpPreprocessData(dumpFile);
		List<Token> tokens = lexer.lex();
		
		// TODO replace with parser
		ParseClass parseClass = new ParseClass(tokens);
		
		ClassConstruct classConstruct = parseClass.parseClassConstruct(tokens);
		
		PlpCompiler compiler = new PlpCompiler(classConstruct);
		List<String> compiledPlp = compiler.compile();
		
		PrintWriter compiledWriter = new PrintWriter(plpFile);
		
		for (String line : compiledPlp)
			compiledWriter.println(line);
		
		compiledWriter.close();
	}
	
	private void initializeTypeSizesAndRegisters()
	{
		typeSize = new HashMap<String, Integer>();
		typeSize.put("byte", 32);
		typeSize.put("short", 32);
		typeSize.put("int", 32);
		typeSize.put("long", 64);
		typeSize.put("boolean", 32);
		typeSize.put("char", 32);
		
		registers = new HashMap<>();
		int tCap = registerFloor + 6;
		
		for (int index = registerFloor; index <= registerCap; index++)
		{
			if (index < tCap)
				registers.put(index, "$t" + index);
			else
				registers.put(index, "$s" + (index - tCap));
		}
		
		commands = new HashMap<>();
		commands.put("+L", "addiu");
		commands.put("+", "addu");
		commands.put("|L", "ori");
		commands.put("|", "or");
		commands.put("-", "subu");
		commands.put("*", "mullo");
		
	}
}
