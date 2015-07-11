package edu.asu.plp.compile.parser.constructs;

import java.util.ArrayList;
import java.util.List;

import edu.asu.plp.Token;

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
	
	public ClassConstruct()
	{
		imports = new ArrayList<>();
		globalVariables = new ArrayList<>();
		localVariables = new ArrayList<>();
		staticMethods = new ArrayList<>();
		classMethods = new ArrayList<>();
	}
	
	public void addImport(Token importToken)
	{
		this.imports.add(importToken);
	}
	
	public void addGlobalVariable(Variable variable)
	{
		this.globalVariables.add(variable);
	}
	
	public void addLocalVariable(Variable variable)
	{
		this.localVariables.add(variable);
	}
	
	public void addStaticMethod(MethodConstruct method)
	{
		this.staticMethods.add(method);
	}
	
	public void addClassMethod(MethodConstruct method)
	{
		this.classMethods.add(method);
	}
	
	public String getPackagePath()
	{
		return packagePath;
	}
	
	public void setPackagePath(String packagePath)
	{
		this.packagePath = packagePath;
	}
	
	public String getClassName()
	{
		return className;
	}
	
	public void setClassName(String className)
	{
		this.className = className;
	}
	
	public List<Token> getImports()
	{
		return imports;
	}
	
	public void setImports(List<Token> imports)
	{
		this.imports = imports;
	}
	
	public List<Variable> getGlobalVariables()
	{
		return globalVariables;
	}
	
	public void setGlobalVariables(List<Variable> globalVariables)
	{
		this.globalVariables = globalVariables;
	}
	
	public List<Variable> getLocalVariables()
	{
		return localVariables;
	}
	
	public void setLocalVariables(List<Variable> localVariables)
	{
		this.localVariables = localVariables;
	}
	
	public MethodConstruct getStaticInitializer()
	{
		return staticInitializer;
	}
	
	public void setStaticInitializer(MethodConstruct staticInitializer)
	{
		this.staticInitializer = staticInitializer;
	}
	
	public MethodConstruct getLocalInitializer()
	{
		return localInitializer;
	}
	
	public void setLocalInitializer(MethodConstruct localInitializer)
	{
		this.localInitializer = localInitializer;
	}
	
	public MethodConstruct getMainMethod()
	{
		return mainMethod;
	}
	
	public void setMainMethod(MethodConstruct mainMethod)
	{
		this.mainMethod = mainMethod;
	}
	
	public List<MethodConstruct> getStaticMethods()
	{
		return staticMethods;
	}
	
	public void setStaticMethods(List<MethodConstruct> staticMethods)
	{
		this.staticMethods = staticMethods;
	}
	
	public List<MethodConstruct> getClassMethods()
	{
		return classMethods;
	}
	
	public void setClassMethods(List<MethodConstruct> classMethods)
	{
		this.classMethods = classMethods;
	}
	
	public ClassConstruct getParent()
	{
		return parent;
	}
	
	public void setParent(ClassConstruct parent)
	{
		this.parent = parent;
	}
}
