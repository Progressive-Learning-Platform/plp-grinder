pub mod symbol_table;
use std::collections::HashMap;

pub trait StaticSymbolTable
{
	/// Return all symbols in this table with the specified name (in any namespace)
	fn lookup_by_name(&self, name: &str) -> Vec<&Symbol>;

	/// Return all symbols in this table with the specified namespace
	fn lookup_by_namespace(&self, namespace: &str) -> Vec<&Symbol>;

	/// Lookup a variable by its name and namespace. Duplicate symbols are not allowed, so the result will be unique
	/// @return the specified symbol or None if the specified symbol is not in this namespace
	fn lookup_variable(&self, namespace: &str, name: &str) -> Option<&Symbol>;

	/// Lookup a function by its name and namespace. Functions with the same signature are not allowed, so the result will be unique
	/// If no result is found in the direct namespace, the parent namespaces will be searched
	/// @return the specified symbol or None if the specified symbol is not in this namespace or a parent namespace
	fn lookup_function(&self, namespace: &str, name: &str, argument_types: &Vec<String>) -> Option<&Symbol>;

	/// Lookup a structure (class, enum) by its name and namespace.
	/// Duplicate classes in the same namespace are not allowed, so the result will be unique
	/// If no result is found in the direct namespace, the parent namespaces will be searched
	/// @return the specified symbol or None if the specified symbol is not in this namespace or a parent namespace
	fn lookup_structure(&self, namespace: &str, name: &str) -> Option<(&Symbol)>;

	/// Adds a symbol to this table and allocates it's location
	/// Returns true if the symbol could be added; false otherwise
	/// Duplicate symbols are not allowed
	/// TODO: support overloaded methods
	fn add(&mut self, class: SymbolClass, namespace: String, name: String, is_static: bool, in_method: bool, is_parameter: bool, local_variable_count: u16, static_variable_count: u16, parameter_offset: u16) -> bool;

	///Concatenate to Strings together to get the correct namepsace
	///@return the concatenated namespace
	fn concatenate_namespace(&self, namespace: String, extension: String) -> String;
}

pub enum SymbolLocation
{
	/// Indicates that a register has been reserved for a specific use (e.g. by a variable)
	/// tuple: (name)
	Register(String),

	/// Indicates a location in memory (e.g. for static variables)
	/// tuple: (address)
	Memory(MemoryAddress),

	/// Indicates an offset location from a structured entity (e.g. a member variable of a class)
	/// tuple: offset
	InstancedMemory(u16),

	MethodArgument(u16),

	/// Indicates that the symbol should not be accessed, as it represents a structured entity
	Structured
}

pub enum SymbolClass
{
	/// (variable_type)
	Variable(String),

	/// Function signature (return_type, argument_types, static_memory_label, static_memory_size (in words)) //TODO: support exceptions
	Function(String, Vec<String>, String, usize),

	/// Includes class, enum, and interface
	/// (subtype)
	Structure(String),
}

pub struct Symbol
{
	/// Namespace of this symbol, without the final "." or the name of this symbol
	pub namespace: String,

	pub is_static: bool,

	/// Identifier for the symbol (e.g. name of variable, function, class, etc. without it's namespace)
	pub name: String,

	/// What this symbol represents (class, enum, variable, function, etc)
	pub symbol_class: SymbolClass,

	/// Memory location of this symbol. Methods will always map to a SymbolLocation::Memory label with a 0 offset
	pub location: SymbolLocation,
}

pub struct MemoryAddress
{
	/// Label marking the base address where the symbol is stored
	pub label_name: String,

	/// Offset from the base address to access the symbol. Methods will always have an offset of 0
	pub offset: u16,
}
