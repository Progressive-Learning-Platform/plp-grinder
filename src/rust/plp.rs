pub struct PLPWriter
{
	pub use_tabs: bool,
	pub indent_level: u16,
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
}
