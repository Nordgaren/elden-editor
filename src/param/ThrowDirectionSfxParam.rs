/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::THROW_DIRECTION_SFX_PARAM_ST::THROW_DIRECTION_SFX_PARAM_ST;

/// Type: THROW_DIRECTION_SFX_PARAM_ST

pub struct ThrowDirectionSfxParam {
	_data: THROW_DIRECTION_SFX_PARAM_ST
}
impl Param for ThrowDirectionSfxParam {
	type Def = THROW_DIRECTION_SFX_PARAM_ST;
	const NAME: &'static str = "ThrowDirectionSfxParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for ThrowDirectionSfxParam {
	type Target = THROW_DIRECTION_SFX_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for ThrowDirectionSfxParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ThrowDirectionSfxParam::ThrowDirectionSfxParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ThrowDirectionSfxParam>(), 144)
	}
}
