/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::SE_MATERIAL_CONVERT_PARAM_ST::SE_MATERIAL_CONVERT_PARAM_ST;

/// Type: SE_MATERIAL_CONVERT_PARAM_ST

pub struct SeMaterialConvertParam {
	_data: SE_MATERIAL_CONVERT_PARAM_ST
}
impl Param for SeMaterialConvertParam {
	type Def = SE_MATERIAL_CONVERT_PARAM_ST;
	const NAME: &'static str = "SeMaterialConvertParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for SeMaterialConvertParam {
	type Target = SE_MATERIAL_CONVERT_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for SeMaterialConvertParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::SeMaterialConvertParam::SeMaterialConvertParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<SeMaterialConvertParam>(), 4)
	}
}
