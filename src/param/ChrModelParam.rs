/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CHR_MODEL_PARAM_ST.rs");

/// Type: CHR_MODEL_PARAM_ST

pub type ChrModelParam = ParamStruct<CHR_MODEL_PARAM_ST>;
impl Param for ParamStruct<CHR_MODEL_PARAM_ST> {
	const NAME: &'static str = "ChrModelParam";
	const TYPE_NAME: &'static str = "CHR_MODEL_PARAM_ST";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ChrModelParam::ChrModelParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ChrModelParam>(), 16)
	}
}
