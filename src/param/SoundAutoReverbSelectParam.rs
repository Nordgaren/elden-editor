/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SOUND_AUTO_REVERB_SELECT_PARAM_ST.rs");

/// Type: SOUND_AUTO_REVERB_SELECT_PARAM_ST

pub type SoundAutoReverbSelectParam = ParamStruct<SOUND_AUTO_REVERB_SELECT_PARAM_ST>;
impl Param for ParamStruct<SOUND_AUTO_REVERB_SELECT_PARAM_ST> {
	const NAME: &'static str = "SoundAutoReverbSelectParam";
	const TYPE_NAME: &'static str = "SOUND_AUTO_REVERB_SELECT_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::SoundAutoReverbSelectParam::SoundAutoReverbSelectParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<SoundAutoReverbSelectParam>(), 32)
	}
}
