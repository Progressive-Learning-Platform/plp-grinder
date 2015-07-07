pub struct PLPWriter
{
	/// If true, tabs will be used to indent. Else, spaces will be used to indent
	pub use_tabs: bool,

	/// Number of indent characters to prefix each line of code with.
	/// Adjusting this value will not affect code that has already been written
	pub indent_level: u16,

	/// PLP output of this writer
	pub code: String,
}

impl PLPWriter
{
	// TODO: implement indentation writing
	pub fn new() -> PLPWriter
	{
		PLPWriter {
			use_tabs: true,
			indent_level: 0,
			code: String::new(),
			}
	}

	pub fn reset(&mut self)
	{
		self.indent_level = 0;
		self.code = String::new();
	}

	pub fn li(&mut self, register: &str, value: &str) -> String
	{
		let mut code = String::new();
		code.push_str("li ");
		code.push_str(register);
		code.push_str(", ");
		code.push_str(value);
		code.push_str("\n");

		self.code.push_str(&*code);
		return code;
	}

	pub fn mov(&mut self, register_to: &str, register_from: &str) -> String
	{
		let mut code = String::new();
		code.push_str("move ");
		code.push_str(register_to);
		code.push_str(", ");
		code.push_str(register_from);
		code.push_str("\n");

		self.code.push_str(&*code);
		return code;
	}

	pub fn addu(&mut self, register_sum: &str, register_addend1: &str, register_addend2: &str) -> String
	{
		let mut code = String::new();
		code.push_str("addu ");
		code.push_str(register_sum);
		code.push_str(", ");
		code.push_str(register_addend1);
		code.push_str(", ");
		code.push_str(register_addend2);
		code.push_str("\n");

		self.code.push_str(&*code);
		return code;
	}

	pub fn subu(&mut self, register_difference: &str, register_addend1: &str, register_addend2: &str) -> String
	{
		let mut code = String::new();
		code.push_str("subu ");
		code.push_str(register_difference);
		code.push_str(", ");
		code.push_str(register_addend1);
		code.push_str(", ");
		code.push_str(register_addend2);
		code.push_str("\n");

		self.code.push_str(&*code);
		return code;
	}

	pub fn mullo(&mut self, register_product: &str, register_multiplicand1: &str, register_multiplicand2: &str) -> String
	{
		let mut code = String::new();
		code.push_str("subu ");
		code.push_str(register_product);
		code.push_str(", ");
		code.push_str(register_multiplicand1);
		code.push_str(", ");
		code.push_str(register_multiplicand2);
		code.push_str("\n");

		self.code.push_str(&*code);
		return code;
	}

	pub fn sw(&mut self, register_target: &str, offset: u16, register_address: &str) -> String
	{
		let mut code = String::new();
		code.push_str("sw ");
		code.push_str(register_target);
		code.push_str(", ");
		code.push_str(&*offset.to_string());
		code.push_str(" (");
		code.push_str(register_address);
		code.push_str(")\n");

		self.code.push_str(&*code);
		return code;
	}

	pub fn lw(&mut self, register_target: &str, offset: u16, register_address: &str) -> String
	{
		let mut code = String::new();
		code.push_str("lw ");
		code.push_str(register_target);
		code.push_str(", ");
		code.push_str(&*offset.to_string());
		code.push_str(" (");
		code.push_str(register_address);
		code.push_str(")\n");

		self.code.push_str(&*code);
		return code;
	}

	pub fn push(&mut self, register_target: &str) -> String
	{
		let mut code = String::new();
		code.push_str("push ");
		code.push_str(register_target);
		code.push_str("\n");

		self.code.push_str(&*code);
		return code;
	}

	pub fn pop(&mut self, register_target: &str) -> String
	{
		let mut code = String::new();
		code.push_str("pop ");
		code.push_str(register_target);
		code.push_str("\n");

		self.code.push_str(&*code);
		return code;
	}

	pub fn call(&mut self, function_label: &str) -> String
	{
		let mut code = String::new();
		code.push_str("call ");
		code.push_str(function_label);
		code.push_str("\n");
		code.push_str("nop");
		code.push_str("\n");

		self.code.push_str(&*code);
		return code;
	}

	pub fn nop(&mut self) -> String
	{
		let mut code = String::new();
		code.push_str("nop");
		code.push_str("\n");

		self.code.push_str(&*code);
		return code;
	}
}
