/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::BULLET_CREATE_LIMIT_PARAM_ST::BULLET_CREATE_LIMIT_PARAM_ST;

/// Type: BULLET_CREATE_LIMIT_PARAM_ST

pub struct BulletCreateLimitParam {
	_data: BULLET_CREATE_LIMIT_PARAM_ST
}
impl Param for BulletCreateLimitParam {
	type Def = BULLET_CREATE_LIMIT_PARAM_ST;
	const NAME: &'static str = "BulletCreateLimitParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for BulletCreateLimitParam {
	type Target = BULLET_CREATE_LIMIT_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for BulletCreateLimitParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::BulletCreateLimitParam::BulletCreateLimitParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<BulletCreateLimitParam>(), 32)
	}
}
