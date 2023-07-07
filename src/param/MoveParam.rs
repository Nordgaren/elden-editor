/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MOVE_PARAM_ST.rs");

/// Type: MOVE_PARAM_ST

pub type MoveParam = ParamStruct<MOVE_PARAM_ST>;
impl Param for ParamStruct<MOVE_PARAM_ST> {
	const NAME: &'static str = "MoveParam";
	const TYPE_NAME: &'static str = "MOVE_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MoveParam::MoveParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MoveParam>(), 144)
	}
}
