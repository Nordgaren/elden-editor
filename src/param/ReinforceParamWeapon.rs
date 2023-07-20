/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::REINFORCE_PARAM_WEAPON_ST::REINFORCE_PARAM_WEAPON_ST;

/// Type: REINFORCE_PARAM_WEAPON_ST

pub struct ReinforceParamWeapon {
	_data: REINFORCE_PARAM_WEAPON_ST
}
impl Param for ReinforceParamWeapon {
	type Def = REINFORCE_PARAM_WEAPON_ST;
	const NAME: &'static str = "ReinforceParamWeapon";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for ReinforceParamWeapon {
	type Target = REINFORCE_PARAM_WEAPON_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for ReinforceParamWeapon {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
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
