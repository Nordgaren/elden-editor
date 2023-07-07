/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/EQUIP_PARAM_GOODS_ST.rs");

/// Type: EQUIP_PARAM_GOODS_ST

pub type EquipParamGoods = ParamStruct<EQUIP_PARAM_GOODS_ST>;
impl Param for ParamStruct<EQUIP_PARAM_GOODS_ST> {
	const NAME: &'static str = "EquipParamGoods";
	const TYPE_NAME: &'static str = "EQUIP_PARAM_GOODS_ST";
	const VERSION: u16 = 3;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::EquipParamGoods::EquipParamGoods;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<EquipParamGoods>(), 176)
	}
}
