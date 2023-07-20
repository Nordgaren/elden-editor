/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::MATERIAL_EX_PARAM_ST::MATERIAL_EX_PARAM_ST;

/// Type: MATERIAL_EX_PARAM_ST

pub struct MaterialExParam {
	_data: MATERIAL_EX_PARAM_ST
}
impl Param for MaterialExParam {
	type Def = MATERIAL_EX_PARAM_ST;
	const NAME: &'static str = "MaterialExParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for MaterialExParam {
	type Target = MATERIAL_EX_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for MaterialExParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MaterialExParam::MaterialExParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MaterialExParam>(), 96)
	}
}
