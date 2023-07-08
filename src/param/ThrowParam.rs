/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/THROW_PARAM_ST.rs");

/// Type: THROW_PARAM_ST

pub type ThrowParam = ParamStruct<THROW_PARAM_ST>;
impl Param for ParamStruct<THROW_PARAM_ST> {
	const NAME: &'static str = "ThrowParam";
	const TYPE_NAME: &'static str = "THROW_PARAM_ST";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ThrowParam::ThrowParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ThrowParam>(), 128)
	}
}
