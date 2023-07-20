/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::SFX_BLOCK_RES_SHARE_PARAM::SFX_BLOCK_RES_SHARE_PARAM;

/// Type: SFX_BLOCK_RES_SHARE_PARAM

pub struct SfxBlockResShareParam {
	_data: SFX_BLOCK_RES_SHARE_PARAM
}
impl Param for SfxBlockResShareParam {
	type Def = SFX_BLOCK_RES_SHARE_PARAM;
	const NAME: &'static str = "SfxBlockResShareParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for SfxBlockResShareParam {
	type Target = SFX_BLOCK_RES_SHARE_PARAM;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for SfxBlockResShareParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::SfxBlockResShareParam::SfxBlockResShareParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<SfxBlockResShareParam>(), 4)
	}
}
