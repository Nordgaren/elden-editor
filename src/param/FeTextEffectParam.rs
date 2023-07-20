/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::FE_TEXT_EFFECT_PARAM_ST::FE_TEXT_EFFECT_PARAM_ST;

/// Type: FE_TEXT_EFFECT_PARAM_ST

pub struct FeTextEffectParam {
	_data: FE_TEXT_EFFECT_PARAM_ST
}
impl Param for FeTextEffectParam {
	type Def = FE_TEXT_EFFECT_PARAM_ST;
	const NAME: &'static str = "FeTextEffectParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for FeTextEffectParam {
	type Target = FE_TEXT_EFFECT_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for FeTextEffectParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::FeTextEffectParam::FeTextEffectParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<FeTextEffectParam>(), 32)
	}
}
