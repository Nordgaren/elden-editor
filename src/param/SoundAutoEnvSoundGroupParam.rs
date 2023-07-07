/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST.rs");

/// Type: SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST

pub type SoundAutoEnvSoundGroupParam = ParamStruct<SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST>;
impl Param for ParamStruct<SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST> {
	const NAME: &'static str = "SoundAutoEnvSoundGroupParam";
	const TYPE_NAME: &'static str = "SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST";
	const VERSION: u16 = 0;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::SoundAutoEnvSoundGroupParam::SoundAutoEnvSoundGroupParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<SoundAutoEnvSoundGroupParam>(), 16)
	}
}
