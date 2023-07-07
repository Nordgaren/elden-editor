/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/COOL_TIME_PARAM_ST.rs");

/// Type: COOL_TIME_PARAM_ST

pub type CoolTimeParam = ParamStruct<COOL_TIME_PARAM_ST>;
impl Param for ParamStruct<COOL_TIME_PARAM_ST> {
	const NAME: &'static str = "CoolTimeParam";
	const TYPE_NAME: &'static str = "COOL_TIME_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::CoolTimeParam::CoolTimeParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<CoolTimeParam>(), 32)
	}
}
