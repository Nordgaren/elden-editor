/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::EQUIP_PARAM_CUSTOM_WEAPON_ST::EQUIP_PARAM_CUSTOM_WEAPON_ST;

/// Type: EQUIP_PARAM_CUSTOM_WEAPON_ST

pub struct EquipParamCustomWeapon {
	_data: EQUIP_PARAM_CUSTOM_WEAPON_ST
}
impl Param for EquipParamCustomWeapon {
	type Def = EQUIP_PARAM_CUSTOM_WEAPON_ST;
	const NAME: &'static str = "EquipParamCustomWeapon";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for EquipParamCustomWeapon {
	type Target = EQUIP_PARAM_CUSTOM_WEAPON_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for EquipParamCustomWeapon {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
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
