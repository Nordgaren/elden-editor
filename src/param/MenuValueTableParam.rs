/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MENU_VALUE_TABLE_SPEC.rs");

/// Type: MENU_VALUE_TABLE_SPEC

pub type MenuValueTableParam = ParamStruct<MENU_VALUE_TABLE_SPEC>;
impl Param for ParamStruct<MENU_VALUE_TABLE_SPEC> {
	const NAME: &'static str = "MenuValueTableParam";
	const TYPE_NAME: &'static str = "MENU_VALUE_TABLE_SPEC";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MenuValueTableParam::MenuValueTableParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MenuValueTableParam>(), 12)
	}
}
