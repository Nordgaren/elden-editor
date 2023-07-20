/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::MULTI_SOUL_BONUS_RATE_PARAM_ST::MULTI_SOUL_BONUS_RATE_PARAM_ST;

/// Type: MULTI_SOUL_BONUS_RATE_PARAM_ST

pub struct MultiSoulBonusRateParam {
	_data: MULTI_SOUL_BONUS_RATE_PARAM_ST
}
impl Param for MultiSoulBonusRateParam {
	type Def = MULTI_SOUL_BONUS_RATE_PARAM_ST;
	const NAME: &'static str = "MultiSoulBonusRateParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for MultiSoulBonusRateParam {
	type Target = MULTI_SOUL_BONUS_RATE_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for MultiSoulBonusRateParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MultiSoulBonusRateParam::MultiSoulBonusRateParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MultiSoulBonusRateParam>(), 128)
	}
}
