/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::param::traits::*;

include!("defs/SHOP_LINEUP_PARAM.rs");

/// Type: SHOP_LINEUP_PARAM

pub type ShopLineupParam_Recipe = ParamStruct<SHOP_LINEUP_PARAM>;
impl Param for ParamStruct<SHOP_LINEUP_PARAM> {
	const NAME: &'static str = "ShopLineupParam_Recipe";
	const TYPE_NAME: &'static str = "SHOP_LINEUP_PARAM";
	const VERSION: u16 = 3;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ShopLineupParam_Recipe::ShopLineupParam_Recipe;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ShopLineupParam_Recipe>(), 52)
	}
}
