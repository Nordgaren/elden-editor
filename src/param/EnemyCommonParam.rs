/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::ENEMY_COMMON_PARAM_ST::ENEMY_COMMON_PARAM_ST;

/// Type: ENEMY_COMMON_PARAM_ST

pub struct EnemyCommonParam {
	_data: ENEMY_COMMON_PARAM_ST
}
impl Param for EnemyCommonParam {
	type Def = ENEMY_COMMON_PARAM_ST;
	const NAME: &'static str = "EnemyCommonParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for EnemyCommonParam {
	type Target = ENEMY_COMMON_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for EnemyCommonParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::EnemyCommonParam::EnemyCommonParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<EnemyCommonParam>(), 256)
	}
}
