

pub trait StaticSymbolTable<'a>
{
	fn lookup_name(name: &str) -> Vec<Symbol<'a>>;
	fn lookup_namespace(namespace: &str) -> Vec<Symbol<'a>>;
	fn lookup_variable(namespace: &str, name: &str) -> Vec<Symbol<'a>>;

	fn add(symbol: Symbol<'a>);
}

pub enum SymbolLocation<'a>
{
	Register { name: &'a str },
	Memory { location: Location<'a> },
	Structured
}

pub enum SymbolClass
{
	Variable,
	Function,
	/// Includes class, enum, and interface
	Structure { subtype: String },
}

pub struct Symbol<'a>
{
	/// Namespace of this symbol, without the final "." or the name of this symbol
	pub namespace: &'a str,

	/// Identifier for the symbol (e.g. name of variable, function, class, etc. without it's namespace)
	pub name: &'a str,

	/// What this symbol represents (class, enum, variable, function, etc)
	pub symbol_class: SymbolClass,

	/// Type can be the return type of a method, the type of a variable, or None if it has no type (for instance if it is a class, enum, etc).
	pub symbol_type: Option<&'a str>,

	/// Memory location of this symbol. Methods will always map to a SymbolLocation::Memory label with a 0 offset
	pub location: SymbolLocation<'a>,
}

pub struct Location<'a>
{
	/// Label marking the base address where the symbol is stored
	pub label_name: &'a str,

	/// Offset from the base address to access the symbol. Methods will always have an offset of 0
	pub offset: u16,
}
