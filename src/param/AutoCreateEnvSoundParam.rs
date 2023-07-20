/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::AUTO_CREATE_ENV_SOUND_PARAM_ST::AUTO_CREATE_ENV_SOUND_PARAM_ST;

/// Type: AUTO_CREATE_ENV_SOUND_PARAM_ST

pub struct AutoCreateEnvSoundParam {
	_data: AUTO_CREATE_ENV_SOUND_PARAM_ST
}
impl Param for AutoCreateEnvSoundParam {
	type Def = AUTO_CREATE_ENV_SOUND_PARAM_ST;
	const NAME: &'static str = "AutoCreateEnvSoundParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for AutoCreateEnvSoundParam {
	type Target = AUTO_CREATE_ENV_SOUND_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for AutoCreateEnvSoundParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::AutoCreateEnvSoundParam::AutoCreateEnvSoundParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<AutoCreateEnvSoundParam>(), 32)
	}
}
