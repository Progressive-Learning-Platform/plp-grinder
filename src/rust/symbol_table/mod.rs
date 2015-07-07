pub mod symbol_table;
use std::collections::HashMap;

pub trait StaticSymbolTable<'a>
{
	/// Return all symbols in this table with the specified name (in any namespace)
	fn lookup_by_name(&self, name: &str) -> Vec<&Symbol<'a>>;

	/// Return all symbols in this table with the specified namespace
	fn lookup_by_namespace(&self, namespace: &str) -> Vec<&Symbol<'a>>;

	/// Lookup a variable by its name and namespace. Duplicate symbols are not allowed, so the result will be unique
	/// @return the specified symbol or None if the specified symbol is not in this namespace
	fn lookup_variable(&self, namespace: &str, name: &str) -> Option<&Symbol<'a>>;

	/// Lookup a function by its name and namespace. Functions with the same signature are not allowed, so the result will be unique
	/// If no result is found in the direct namespace, the parent namespaces will be searched
	/// @return the specified symbol or None if the specified symbol is not in this namespace or a parent namespace
	fn lookup_function(&self, namespace: &str, name: &str, argument_types: &Vec<String>) -> Option<&Symbol<'a>>;

	/// Lookup a structure (class, enum) by its name and namespace.
	/// Duplicate classes in the same namespace are not allowed, so the result will be unique
	/// If no result is found in the direct namespace, the parent namespaces will be searched
	/// @return the specified symbol or None if the specified symbol is not in this namespace or a parent namespace
	fn lookup_structure(&self, namespace: &str, name: &str) -> Option<(&Symbol<'a>)>;

	/// Adds a symbol to this table and allocates it's location
	/// Returns true if the symbol could be added; false otherwise
	/// Duplicate symbols are not allowed
	/// TODO: support overloaded methods
	fn add(&mut self, class: SymbolClass<'a>, namespace: &'a str, name: &'a str) -> bool;
}

pub enum SymbolLocation<'a>
{
	/// Indicates that a register has been reserved for a specific use (e.g. by a variable)
	/// tuple: (name)
	Register(&'a str),

	/// Indicates a location in memory (e.g. for static variables)
	/// tuple: (address)
	Memory(MemoryAddress<'a>),

	/// Indicates an offset location from a structured entity (e.g. a member variable of a class)
	/// tuple: offset
	InstancedMemory(u16),

	MethodArgument(u16),

	/// Indicates that the symbol should not be accessed, as it represents a structured entity
	Structured
}

pub enum SymbolClass<'a>
{
	/// (variable_type)
	Variable(&'a str),

	/// Function signature (return_type, argument_types) //TODO: support exceptions
	Function(&'a str, &'a Vec<&'a str>),

	/// Includes class, enum, and interface
	/// (subtype)
	Structure(&'a str),
}

pub struct Symbol<'a>
{
	/// Namespace of this symbol, without the final "." or the name of this symbol
	pub namespace: &'a str,

	pub is_static: bool,

	/// Identifier for the symbol (e.g. name of variable, function, class, etc. without it's namespace)
	pub name: &'a str,

	/// What this symbol represents (class, enum, variable, function, etc)
	pub symbol_class: SymbolClass<'a>,

	/// Memory location of this symbol. Methods will always map to a SymbolLocation::Memory label with a 0 offset
	pub location: SymbolLocation<'a>,
}

pub struct MemoryAddress<'a>
{
	/// Label marking the base address where the symbol is stored
	pub label_name: &'a str,

	/// Offset from the base address to access the symbol. Methods will always have an offset of 0
	pub offset: u16,
}
