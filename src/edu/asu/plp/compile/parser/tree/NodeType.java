package edu.asu.plp.compile.parser.tree;

public enum NodeType
{
	//@formatter:off
	/** 
	 * 2 Children
	 * No Value
	 * Left: Value node specifying the type
	 * Right: Value node specifying the variable name
	 */
	DECLARATION,
	
	/** 
	 * 2 Children
	 * No Value
	 * Left: Value of reference on which to act
	 * Right: Node specifying what to access (presumably a subroutine or value node)
	 */
	ACCESSOR,
	
	/** 
	 * 0 Children
	 * Must specify a value
	 * value is the name of a subroutine to call
	 */
	SUBROUTINE,
	
	/** 
	 * 0 Children
	 * Must specify a value
	 * Value can be literal or a reference
	 */
	VALUE,
	
	/** 
	 * 3 Children
	 * Left: Condition (i.e. an operator that evaluates to a value or a value that will be treated as true / false)
	 * Middle: Block that executes if the condition is true
	 * Right: Block that executes if the condition is false
	 */
	CONDITIONAL,
	
	/** 
	 * 2 Children
	 * Left: Condition (i.e. an operator that evaluates to a value or a value that will be treated as true / false)
	 * Right: Block that will execute at least once, and n number of times thereafter, until the condition is false
	 */
	DO,
	
	/** 
	 * 2 Children
	 * Left: Condition (i.e. an operator that evaluates to a value or a value that will be treated as true / false)
	 * Right: Block that will execute until the condition is false
	 */
	WHILE,

	/** 
	 * 4 Children
	 * Left: Block that will execute exactly once, before the loop begins
	 * Middle-Left: Condition (i.e. an operator that evaluates to a value or a value that will be treated as true / false)
	 * Middle-Right: Block that will execute once after each execution of the loop's block
	 * Right: Block that will execute until the condition is false
	 */
	FOR,
	
	/** 
	 * 1:N Children
	 * Must specify a value: value specifies which operator (+, -, =, etc.). Note that the example values are all binary operators, and will therefore have 2 children
	 * Children: value nodes or nodes (such as operators) that evaluate to a value.
	 */
	OPERATOR;
}
