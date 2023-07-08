/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/EQUIP_MTRL_SET_PARAM_ST.rs");

/// Type: EQUIP_MTRL_SET_PARAM_ST

pub type EquipMtrlSetParam = ParamStruct<EQUIP_MTRL_SET_PARAM_ST>;
impl Param for ParamStruct<EQUIP_MTRL_SET_PARAM_ST> {
	const NAME: &'static str = "EquipMtrlSetParam";
	const TYPE_NAME: &'static str = "EQUIP_MTRL_SET_PARAM_ST";
	const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::EquipMtrlSetParam::EquipMtrlSetParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<EquipMtrlSetParam>(), 52)
	}
}
