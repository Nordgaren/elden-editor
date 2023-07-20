/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::WHITE_SIGN_COOL_TIME_PARAM_ST::WHITE_SIGN_COOL_TIME_PARAM_ST;

/// Type: WHITE_SIGN_COOL_TIME_PARAM_ST

pub struct WhiteSignCoolTimeParam {
	_data: WHITE_SIGN_COOL_TIME_PARAM_ST
}
impl Param for WhiteSignCoolTimeParam {
	type Def = WHITE_SIGN_COOL_TIME_PARAM_ST;
	const NAME: &'static str = "WhiteSignCoolTimeParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for WhiteSignCoolTimeParam {
	type Target = WHITE_SIGN_COOL_TIME_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for WhiteSignCoolTimeParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::WhiteSignCoolTimeParam::WhiteSignCoolTimeParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<WhiteSignCoolTimeParam>(), 16)
	}
}
