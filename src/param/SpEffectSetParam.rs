/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::SP_EFFECT_SET_PARAM_ST::SP_EFFECT_SET_PARAM_ST;

/// Type: SP_EFFECT_SET_PARAM_ST

pub struct SpEffectSetParam {
	_data: SP_EFFECT_SET_PARAM_ST
}
impl Param for SpEffectSetParam {
	type Def = SP_EFFECT_SET_PARAM_ST;
	const NAME: &'static str = "SpEffectSetParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for SpEffectSetParam {
	type Target = SP_EFFECT_SET_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for SpEffectSetParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::SpEffectSetParam::SpEffectSetParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<SpEffectSetParam>(), 16)
	}
}
