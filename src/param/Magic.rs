/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MAGIC_PARAM_ST.rs");

/// Type: MAGIC_PARAM_ST

pub type Magic = ParamStruct<MAGIC_PARAM_ST>;
impl Param for ParamStruct<MAGIC_PARAM_ST> {
	const NAME: &'static str = "Magic";
	const TYPE_NAME: &'static str = "MAGIC_PARAM_ST";
	const VERSION: u16 = 6;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::Magic::Magic;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<Magic>(), 168)
	}
}
