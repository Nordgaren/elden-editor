/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::CHARACTER_INIT_PARAM::CHARACTER_INIT_PARAM;

/// Type: CHARACTER_INIT_PARAM

pub struct CharaInitParam {
	_data: CHARACTER_INIT_PARAM
}
impl Param for CharaInitParam {
	type Def = CHARACTER_INIT_PARAM;
	const NAME: &'static str = "CharaInitParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for CharaInitParam {
	type Target = CHARACTER_INIT_PARAM;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for CharaInitParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::CharaInitParam::CharaInitParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<CharaInitParam>(), 320)
	}
}
