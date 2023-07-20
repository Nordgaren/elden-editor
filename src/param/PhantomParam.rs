/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::PHANTOM_PARAM_ST::PHANTOM_PARAM_ST;

/// Type: PHANTOM_PARAM_ST

pub struct PhantomParam {
	_data: PHANTOM_PARAM_ST
}
impl Param for PhantomParam {
	type Def = PHANTOM_PARAM_ST;
	const NAME: &'static str = "PhantomParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for PhantomParam {
	type Target = PHANTOM_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for PhantomParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::PhantomParam::PhantomParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<PhantomParam>(), 56)
	}
}
