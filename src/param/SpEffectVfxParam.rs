/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::SP_EFFECT_VFX_PARAM_ST::SP_EFFECT_VFX_PARAM_ST;

/// Type: SP_EFFECT_VFX_PARAM_ST

pub struct SpEffectVfxParam {
	_data: SP_EFFECT_VFX_PARAM_ST
}
impl Param for SpEffectVfxParam {
	type Def = SP_EFFECT_VFX_PARAM_ST;
	const NAME: &'static str = "SpEffectVfxParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for SpEffectVfxParam {
	type Target = SP_EFFECT_VFX_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for SpEffectVfxParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::SpEffectVfxParam::SpEffectVfxParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<SpEffectVfxParam>(), 164)
	}
}
