/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST::SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST;

/// Type: SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST

pub struct SoundAutoReverbEvaluationDistParam {
	_data: SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST
}
impl Param for SoundAutoReverbEvaluationDistParam {
	type Def = SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST;
	const NAME: &'static str = "SoundAutoReverbEvaluationDistParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for SoundAutoReverbEvaluationDistParam {
	type Target = SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for SoundAutoReverbEvaluationDistParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::SoundAutoReverbEvaluationDistParam::SoundAutoReverbEvaluationDistParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<SoundAutoReverbEvaluationDistParam>(), 20)
	}
}
