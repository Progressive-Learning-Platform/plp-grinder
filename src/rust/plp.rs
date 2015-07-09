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
	pub fn new() -> PLPWriter
	{
		PLPWriter {
			use_tabs: true,
			indent_level: 0,
			code: String::new(),
			}
	}

	fn create_indented_string(&self) -> String
	{
		let mut string = String::new();
		let indent = match self.use_tabs {
				true  => "\t",
				false => " ",
			};

		for _ in (0..self.indent_level)
		{
			string.push_str(indent);
		}

		string
	}

	#[allow(dead_code)]
	pub fn reset(&mut self)
	{
		self.indent_level = 0;
		self.code = String::new();
	}

	pub fn println(&mut self)
	{
		self.code.push_str("\n");
	}

	pub fn org(&mut self, address: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str(".org ");
		code.push_str(address);
		code.push_str("\n");

		self.code.push_str(&*code);
	}

	pub fn word(&mut self, initial_value: u16)
	{
		let mut code = self.create_indented_string();
		code.push_str(".word ");
		code.push_str(&*initial_value.to_string());
		code.push_str("\n");

		self.code.push_str(&*code);
	}

	pub fn li(&mut self, register: &str, value: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str("li ");
		code.push_str(register);
		code.push_str(", ");
		code.push_str(value);
		code.push_str("\n");

		self.code.push_str(&*code);
	}

	pub fn mov(&mut self, register_to: &str, register_from: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str("move ");
		code.push_str(register_to);
		code.push_str(", ");
		code.push_str(register_from);
		code.push_str("\n");

		self.code.push_str(&*code);
	}

	pub fn addu(&mut self, register_sum: &str, register_addend1: &str, register_addend2: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str("addu ");
		code.push_str(register_sum);
		code.push_str(", ");
		code.push_str(register_addend1);
		code.push_str(", ");
		code.push_str(register_addend2);
		code.push_str("\n");

		self.code.push_str(&*code);
	}

	pub fn subu(&mut self, register_difference: &str, register_addend1: &str, register_addend2: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str("subu ");
		code.push_str(register_difference);
		code.push_str(", ");
		code.push_str(register_addend1);
		code.push_str(", ");
		code.push_str(register_addend2);
		code.push_str("\n");

		self.code.push_str(&*code);
	}

	pub fn mullo(&mut self, register_product: &str, register_multiplicand1: &str, register_multiplicand2: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str("mullo ");
		code.push_str(register_product);
		code.push_str(", ");
		code.push_str(register_multiplicand1);
		code.push_str(", ");
		code.push_str(register_multiplicand2);
		code.push_str("\n");

		self.code.push_str(&*code);
	}

	pub fn sw(&mut self, register_target: &str, offset: u16, register_address: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str("sw ");
		code.push_str(register_target);
		code.push_str(", ");
		code.push_str(&*offset.to_string());
		code.push_str("(");
		code.push_str(register_address);
		code.push_str(")\n");

		self.code.push_str(&*code);
	}

	pub fn lw(&mut self, register_target: &str, offset: u16, register_address: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str("lw ");
		code.push_str(register_target);
		code.push_str(", ");
		code.push_str(&*offset.to_string());
		code.push_str("(");
		code.push_str(register_address);
		code.push_str(")\n");

		self.code.push_str(&*code);
	}

	pub fn push(&mut self, register_target: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str("push ");
		code.push_str(register_target);
		code.push_str("\n");

		self.code.push_str(&*code);
	}

	pub fn pop(&mut self, register_target: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str("pop ");
		code.push_str(register_target);
		code.push_str("\n");

		self.code.push_str(&*code);
	}

	pub fn call(&mut self, function_label: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str("call ");
		code.push_str(function_label);
		code.push_str("\n");

		self.code.push_str(&*code);
		self.nop();
	}

	pub fn nop(&mut self)
	{
		let mut code = self.create_indented_string();
		code.push_str("nop");
		code.push_str("\n");

		self.code.push_str(&*code);
	}

	pub fn label(&mut self, label: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str(label);
		code.push_str(":");
		code.push_str("\n");

		self.code.push_str(&*code);
	}

	#[allow(dead_code)]
	pub fn beq(&mut self, register_comparator1: &str, register_comparator2: &str, target_label: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str("beq ");
		code.push_str(register_comparator1);
		code.push_str(", ");
		code.push_str(register_comparator2);
		code.push_str(", ");
		code.push_str(target_label);
		code.push_str("\n");

		self.code.push_str(&*code);
	}

	#[allow(dead_code)]
	pub fn bne(&mut self, register_comparator1: &str, register_comparator2: &str, target_label: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str("bne ");
		code.push_str(register_comparator1);
		code.push_str(", ");
		code.push_str(register_comparator2);
		code.push_str(", ");
		code.push_str(target_label);
		code.push_str("\n");

		self.code.push_str(&*code);
	}

	pub fn ret(&mut self)
	{
		let mut code = self.create_indented_string();
		code.push_str("return\n");

		self.code.push_str(&*code);
		self.nop();
	}

	pub fn j(&mut self, label: &str)
	{
		let mut code = self.create_indented_string();
		code.push_str("j ");
		code.push_str(label);
		code.push_str("\n");

		self.code.push_str(&*code);
		self.nop();
	}

	pub fn space(&mut self, amount: u16)
	{
		let mut code = self.create_indented_string();
		code.push_str(".space ");
		code.push_str(&*amount.to_string());
		code.push_str("\n");

		self.code.push_str(&*code);
	}
}
