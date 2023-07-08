/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/EQUIP_PARAM_CUSTOM_WEAPON_ST.rs");

/// Type: EQUIP_PARAM_CUSTOM_WEAPON_ST

pub type EquipParamCustomWeapon = ParamStruct<EQUIP_PARAM_CUSTOM_WEAPON_ST>;
impl Param for ParamStruct<EQUIP_PARAM_CUSTOM_WEAPON_ST> {
	const NAME: &'static str = "EquipParamCustomWeapon";
	const TYPE_NAME: &'static str = "EQUIP_PARAM_CUSTOM_WEAPON_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::EquipParamCustomWeapon::EquipParamCustomWeapon;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<EquipParamCustomWeapon>(), 16)
	}
}
