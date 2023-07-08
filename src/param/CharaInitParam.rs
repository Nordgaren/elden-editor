/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CHARACTER_INIT_PARAM.rs");

/// Type: CHARACTER_INIT_PARAM

pub type CharaInitParam = ParamStruct<CHARACTER_INIT_PARAM>;
impl Param for ParamStruct<CHARACTER_INIT_PARAM> {
	const NAME: &'static str = "CharaInitParam";
	const TYPE_NAME: &'static str = "CHARACTER_INIT_PARAM";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::CharaInitParam::CharaInitParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<CharaInitParam>(), 320)
	}
}
