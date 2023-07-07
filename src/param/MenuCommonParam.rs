/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MENU_COMMON_PARAM_ST.rs");

/// Type: MENU_COMMON_PARAM_ST

pub type MenuCommonParam = ParamStruct<MENU_COMMON_PARAM_ST>;
impl Param for ParamStruct<MENU_COMMON_PARAM_ST> {
	const NAME: &'static str = "MenuCommonParam";
	const TYPE_NAME: &'static str = "MENU_COMMON_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MenuCommonParam::MenuCommonParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MenuCommonParam>(), 256)
	}
}
