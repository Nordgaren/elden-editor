/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/REINFORCE_PARAM_WEAPON_ST.rs");

/// Type: REINFORCE_PARAM_WEAPON_ST

pub type ReinforceParamWeapon = ParamStruct<REINFORCE_PARAM_WEAPON_ST>;
impl Param for ParamStruct<REINFORCE_PARAM_WEAPON_ST> {
	const NAME: &'static str = "ReinforceParamWeapon";
	const TYPE_NAME: &'static str = "REINFORCE_PARAM_WEAPON_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ReinforceParamWeapon::ReinforceParamWeapon;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ReinforceParamWeapon>(), 128)
	}
}
