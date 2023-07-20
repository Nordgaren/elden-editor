/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::PLAYER_COMMON_PARAM_ST::PLAYER_COMMON_PARAM_ST;

/// Type: PLAYER_COMMON_PARAM_ST

pub struct PlayerCommonParam {
	_data: PLAYER_COMMON_PARAM_ST
}
impl Param for PlayerCommonParam {
	type Def = PLAYER_COMMON_PARAM_ST;
	const NAME: &'static str = "PlayerCommonParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for PlayerCommonParam {
	type Target = PLAYER_COMMON_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for PlayerCommonParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::PlayerCommonParam::PlayerCommonParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<PlayerCommonParam>(), 256)
	}
}
