/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST::SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST;

/// Type: SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST

pub struct SoundAssetSoundObjEnableDistParam {
	_data: SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST
}
impl Param for SoundAssetSoundObjEnableDistParam {
	type Def = SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST;
	const NAME: &'static str = "SoundAssetSoundObjEnableDistParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for SoundAssetSoundObjEnableDistParam {
	type Target = SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for SoundAssetSoundObjEnableDistParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::SoundAssetSoundObjEnableDistParam::SoundAssetSoundObjEnableDistParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<SoundAssetSoundObjEnableDistParam>(), 4)
	}
}
