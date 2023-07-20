/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::EQUIP_PARAM_GOODS_ST::EQUIP_PARAM_GOODS_ST;

/// Type: EQUIP_PARAM_GOODS_ST

pub struct EquipParamGoods {
	_data: EQUIP_PARAM_GOODS_ST
}
impl Param for EquipParamGoods {
	type Def = EQUIP_PARAM_GOODS_ST;
	const NAME: &'static str = "EquipParamGoods";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for EquipParamGoods {
	type Target = EQUIP_PARAM_GOODS_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for EquipParamGoods {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
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
