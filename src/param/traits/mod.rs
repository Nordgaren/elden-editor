pub trait Param {
	const NAME: &'static str;
	type Def: Paramdef;

	fn data(&self) -> &Self::Def;
	fn data_mut(&mut self) -> &mut Self::Def;
	fn name() -> &'static str {
		Self::NAME
	}
}
pub trait Paramdef {
	const NAME: &'static str;
	const VERSION: u16;
	// So you can query the type constant from an `impl Param`
	fn param_type_name() -> &'static str {
		Self::NAME
	}
	fn version() -> u16 {
		Self::VERSION
	}
	// etc...
}




