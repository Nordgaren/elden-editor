/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::param::traits::*;

include!("defs/ITEMLOT_PARAM_ST.rs");

/// Type: ITEMLOT_PARAM_ST

pub type ItemLotParam_enemy = ParamStruct<ITEMLOT_PARAM_ST>;
impl Param for ParamStruct<ITEMLOT_PARAM_ST> {
	const NAME: &'static str = "ItemLotParam_enemy";
	const TYPE_NAME: &'static str = "ITEMLOT_PARAM_ST";
	const VERSION: u16 = 4;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ItemLotParam_enemy::ItemLotParam_enemy;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ItemLotParam_enemy>(), 152)
	}
}
