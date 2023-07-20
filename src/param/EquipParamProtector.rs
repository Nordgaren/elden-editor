/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::EQUIP_PARAM_PROTECTOR_ST::EQUIP_PARAM_PROTECTOR_ST;

/// Type: EQUIP_PARAM_PROTECTOR_ST

pub struct EquipParamProtector {
	_data: EQUIP_PARAM_PROTECTOR_ST
}
impl Param for EquipParamProtector {
	type Def = EQUIP_PARAM_PROTECTOR_ST;
	const NAME: &'static str = "EquipParamProtector";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for EquipParamProtector {
	type Target = EQUIP_PARAM_PROTECTOR_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for EquipParamProtector {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::EquipParamProtector::EquipParamProtector;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<EquipParamProtector>(), 416)
	}
}
