package edu.asu.plp.compile.parser.tree;

import java.util.List;

public interface ParseNode
{
	/**
	 * @return The type of this node
	 */
	NodeType getType();
	
	/**
	 * Some data will require a value expression (for instance a variable reference, or a
	 * primitive value). Other data will not. In the latter case, an empty String will be
	 * returned.
	 * 
	 * @return The value held by this node, or an empty String if no value is present
	 */
	String getValue();
	
	/**
	 * @return The children of this node, ordered from left to right, or an empty list if
	 *         no children are present
	 */
	List<ParseNode> getChildren();
}
