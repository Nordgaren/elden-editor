/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::MULTI_PLAY_CORRECTION_PARAM_ST::MULTI_PLAY_CORRECTION_PARAM_ST;

/// Type: MULTI_PLAY_CORRECTION_PARAM_ST

pub struct MultiPlayCorrectionParam {
	_data: MULTI_PLAY_CORRECTION_PARAM_ST
}
impl Param for MultiPlayCorrectionParam {
	type Def = MULTI_PLAY_CORRECTION_PARAM_ST;
	const NAME: &'static str = "MultiPlayCorrectionParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for MultiPlayCorrectionParam {
	type Target = MULTI_PLAY_CORRECTION_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for MultiPlayCorrectionParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MultiPlayCorrectionParam::MultiPlayCorrectionParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MultiPlayCorrectionParam>(), 32)
	}
}
