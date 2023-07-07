/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/BASECHR_SELECT_MENU_PARAM_ST.rs");

/// Type: BASECHR_SELECT_MENU_PARAM_ST

pub type BaseChrSelectMenuParam = ParamStruct<BASECHR_SELECT_MENU_PARAM_ST>;
impl Param for ParamStruct<BASECHR_SELECT_MENU_PARAM_ST> {
	const NAME: &'static str = "BaseChrSelectMenuParam";
	const TYPE_NAME: &'static str = "BASECHR_SELECT_MENU_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::BaseChrSelectMenuParam::BaseChrSelectMenuParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<BaseChrSelectMenuParam>(), 32)
	}
}
