/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/PARTS_DRAW_PARAM_ST.rs");

/// Type: PARTS_DRAW_PARAM_ST

pub type PartsDrawParam = ParamStruct<PARTS_DRAW_PARAM_ST>;
impl Param for ParamStruct<PARTS_DRAW_PARAM_ST> {
	const NAME: &'static str = "PartsDrawParam";
	const TYPE_NAME: &'static str = "PARTS_DRAW_PARAM_ST";
	const VERSION: u16 = 5;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::PartsDrawParam::PartsDrawParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<PartsDrawParam>(), 144)
	}
}
