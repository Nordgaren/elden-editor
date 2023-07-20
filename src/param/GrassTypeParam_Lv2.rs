/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::GRASS_TYPE_PARAM_ST::GRASS_TYPE_PARAM_ST;

/// Type: GRASS_TYPE_PARAM_ST

pub struct GrassTypeParam_Lv2 {
	_data: GRASS_TYPE_PARAM_ST
}
impl Param for GrassTypeParam_Lv2 {
	type Def = GRASS_TYPE_PARAM_ST;
	const NAME: &'static str = "GrassTypeParam_Lv2";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for GrassTypeParam_Lv2 {
	type Target = GRASS_TYPE_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for GrassTypeParam_Lv2 {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::GrassTypeParam_Lv2::GrassTypeParam_Lv2;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<GrassTypeParam_Lv2>(), 276)
	}
}
