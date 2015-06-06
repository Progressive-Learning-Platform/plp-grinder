package edu.asu.plp.compile.parser.constructs;

import java.util.List;

import edu.asu.plp.Token;
import edu.asu.plp.compile.parser.Variable;

public class ClassConstruct
{
	private String packagePath;
	private String className;
	private List<Token> imports;
	private List<Variable> globalVariables;
	private List<Variable> localVariables;
	private MethodConstruct staticInitializer;
	private MethodConstruct localInitializer;
	private MethodConstruct mainMethod;
	private List<MethodConstruct> staticMethods;
	private List<MethodConstruct> classMethods;
	private ClassConstruct parent;
}
