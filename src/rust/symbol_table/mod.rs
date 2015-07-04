

pub trait StaticSymbolTable<'a>
{
	/// Return all symbols in this table with the specified name (in any namespace)
	fn lookup_name(name: &str) -> Vec<Symbol<'a>>;

	/// Return all symbols in this table with the specified namespace
	fn lookup_namespace(namespace: &str) -> Vec<Symbol<'a>>;

	/// Lookup a symbol by its name and namespace. Duplicate symbols are not allowed, so the result will be unique
	fn lookup_variable(namespace: &str, name: &str) -> Symbol<'a>;

	/// Lookup a symbol by its name and namespace. Duplicate symbols are not allowed, so the result will be unique
	fn lookup_function(namespace: &str, name: &str, argument_types: &Vec<&str>) -> Symbol<'a>;

	/// Adds a symbol to this table and allocates it's location
	/// Returns true if the symbol could be added; false otherwise
	/// Duplicate symbols are not allowed
	/// TODO: support overloaded methods
	fn add(class: SymbolClass<'a>, namespace: &'a str, name: &'a str) -> bool;
}

pub enum SymbolLocation<'a>
{
	/// Indicates that a register has been reserved for a specific use (e.g. by a variable)
	Register { name: &'a str },

	/// Indicates a location in memory
	Memory { address: MemoryAddress<'a> },

	/// Indicates that the symbol should not be accessed, as it represents a structured entity
	Structured
}

pub enum SymbolClass<'a>
{
	Variable { variable_type: &'a str },

	/// Function signature //TODO: support exceptions
	Function { return_type: &'a str, argument_types: &'a Vec<&'a str> },

	/// Includes class, enum, and interface
	Structure { subtype: &'a str },
}

pub struct Symbol<'a>
{
	/// Namespace of this symbol, without the final "." or the name of this symbol
	pub namespace: &'a str,

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
