/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::WORLD_MAP_LEGACY_CONV_PARAM_ST::WORLD_MAP_LEGACY_CONV_PARAM_ST;

/// Type: WORLD_MAP_LEGACY_CONV_PARAM_ST

pub struct WorldMapLegacyConvParam {
	_data: WORLD_MAP_LEGACY_CONV_PARAM_ST
}
impl Param for WorldMapLegacyConvParam {
	type Def = WORLD_MAP_LEGACY_CONV_PARAM_ST;
	const NAME: &'static str = "WorldMapLegacyConvParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for WorldMapLegacyConvParam {
	type Target = WORLD_MAP_LEGACY_CONV_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for WorldMapLegacyConvParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::WorldMapLegacyConvParam::WorldMapLegacyConvParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<WorldMapLegacyConvParam>(), 48)
	}
}
