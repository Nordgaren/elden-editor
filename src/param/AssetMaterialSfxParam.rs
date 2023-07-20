/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::ASSET_MATERIAL_SFX_PARAM_ST::ASSET_MATERIAL_SFX_PARAM_ST;

/// Type: ASSET_MATERIAL_SFX_PARAM_ST

pub struct AssetMaterialSfxParam {
	_data: ASSET_MATERIAL_SFX_PARAM_ST
}
impl Param for AssetMaterialSfxParam {
	type Def = ASSET_MATERIAL_SFX_PARAM_ST;
	const NAME: &'static str = "AssetMaterialSfxParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for AssetMaterialSfxParam {
	type Target = ASSET_MATERIAL_SFX_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for AssetMaterialSfxParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::AssetMaterialSfxParam::AssetMaterialSfxParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<AssetMaterialSfxParam>(), 128)
	}
}
