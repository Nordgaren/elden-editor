/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/EQUIP_PARAM_WEAPON_ST.rs");

/// Type: EQUIP_PARAM_WEAPON_ST

pub type EquipParamWeapon = ParamStruct<EQUIP_PARAM_WEAPON_ST>;
impl Param for ParamStruct<EQUIP_PARAM_WEAPON_ST> {
	const NAME: &'static str = "EquipParamWeapon";
	const TYPE_NAME: &'static str = "EQUIP_PARAM_WEAPON_ST";
	const VERSION: u16 = 6;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::EquipParamWeapon::EquipParamWeapon;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<EquipParamWeapon>(), 664)
	}
}
