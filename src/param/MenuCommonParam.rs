/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::MENU_COMMON_PARAM_ST::MENU_COMMON_PARAM_ST;

/// Type: MENU_COMMON_PARAM_ST

pub struct MenuCommonParam {
	_data: MENU_COMMON_PARAM_ST
}
impl Param for MenuCommonParam {
	type Def = MENU_COMMON_PARAM_ST;
	const NAME: &'static str = "MenuCommonParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for MenuCommonParam {
	type Target = MENU_COMMON_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for MenuCommonParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MenuCommonParam::MenuCommonParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MenuCommonParam>(), 256)
	}
}
