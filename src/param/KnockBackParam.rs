/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::KNOCKBACK_PARAM_ST::KNOCKBACK_PARAM_ST;

/// Type: KNOCKBACK_PARAM_ST

pub struct KnockBackParam {
	_data: KNOCKBACK_PARAM_ST
}
impl Param for KnockBackParam {
	type Def = KNOCKBACK_PARAM_ST;
	const NAME: &'static str = "KnockBackParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for KnockBackParam {
	type Target = KNOCKBACK_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for KnockBackParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::KnockBackParam::KnockBackParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<KnockBackParam>(), 128)
	}
}
