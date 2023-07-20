/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::THROW_PARAM_ST::THROW_PARAM_ST;

/// Type: THROW_PARAM_ST

pub struct ThrowParam {
	_data: THROW_PARAM_ST
}
impl Param for ThrowParam {
	type Def = THROW_PARAM_ST;
	const NAME: &'static str = "ThrowParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for ThrowParam {
	type Target = THROW_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for ThrowParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ThrowParam::ThrowParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ThrowParam>(), 128)
	}
}
