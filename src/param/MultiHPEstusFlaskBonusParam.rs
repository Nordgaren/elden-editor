/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::MULTI_ESTUS_FLASK_BONUS_PARAM_ST::MULTI_ESTUS_FLASK_BONUS_PARAM_ST;

/// Type: MULTI_ESTUS_FLASK_BONUS_PARAM_ST

pub struct MultiHPEstusFlaskBonusParam {
	_data: MULTI_ESTUS_FLASK_BONUS_PARAM_ST
}
impl Param for MultiHPEstusFlaskBonusParam {
	type Def = MULTI_ESTUS_FLASK_BONUS_PARAM_ST;
	const NAME: &'static str = "MultiHPEstusFlaskBonusParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for MultiHPEstusFlaskBonusParam {
	type Target = MULTI_ESTUS_FLASK_BONUS_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for MultiHPEstusFlaskBonusParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MultiHPEstusFlaskBonusParam::MultiHPEstusFlaskBonusParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MultiHPEstusFlaskBonusParam>(), 64)
	}
}
