/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::param::traits::*;

include!("defs/ATK_PARAM_ST.rs");

/// Type: ATK_PARAM_ST

pub type AtkParam_Pc = ParamStruct<ATK_PARAM_ST>;
impl Param for ParamStruct<ATK_PARAM_ST> {
	const NAME: &'static str = "AtkParam_Pc";
	const TYPE_NAME: &'static str = "ATK_PARAM_ST";
	const VERSION: u16 = 4;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::AtkParam_Pc::AtkParam_Pc;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<AtkParam_Pc>(), 456)
	}
}
