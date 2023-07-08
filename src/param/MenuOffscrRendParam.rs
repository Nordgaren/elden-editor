/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MENU_OFFSCR_REND_PARAM_ST.rs");

/// Type: MENU_OFFSCR_REND_PARAM_ST

pub type MenuOffscrRendParam = ParamStruct<MENU_OFFSCR_REND_PARAM_ST>;
impl Param for ParamStruct<MENU_OFFSCR_REND_PARAM_ST> {
	const NAME: &'static str = "MenuOffscrRendParam";
	const TYPE_NAME: &'static str = "MENU_OFFSCR_REND_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MenuOffscrRendParam::MenuOffscrRendParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MenuOffscrRendParam>(), 64)
	}
}
