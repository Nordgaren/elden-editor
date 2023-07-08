/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MENU_PARAM_COLOR_TABLE_ST.rs");

/// Type: MENU_PARAM_COLOR_TABLE_ST

pub type MenuColorTableParam = ParamStruct<MENU_PARAM_COLOR_TABLE_ST>;
impl Param for ParamStruct<MENU_PARAM_COLOR_TABLE_ST> {
	const NAME: &'static str = "MenuColorTableParam";
	const TYPE_NAME: &'static str = "MENU_PARAM_COLOR_TABLE_ST";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MenuColorTableParam::MenuColorTableParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MenuColorTableParam>(), 32)
	}
}
