/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::EQUIP_MTRL_SET_PARAM_ST::EQUIP_MTRL_SET_PARAM_ST;

/// Type: EQUIP_MTRL_SET_PARAM_ST

pub struct EquipMtrlSetParam {
	_data: EQUIP_MTRL_SET_PARAM_ST
}
impl Param for EquipMtrlSetParam {
	type Def = EQUIP_MTRL_SET_PARAM_ST;
	const NAME: &'static str = "EquipMtrlSetParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for EquipMtrlSetParam {
	type Target = EQUIP_MTRL_SET_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for EquipMtrlSetParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
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
