/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::BASECHR_SELECT_MENU_PARAM_ST::BASECHR_SELECT_MENU_PARAM_ST;

/// Type: BASECHR_SELECT_MENU_PARAM_ST

pub struct BaseChrSelectMenuParam {
	_data: BASECHR_SELECT_MENU_PARAM_ST
}
impl Param for BaseChrSelectMenuParam {
	type Def = BASECHR_SELECT_MENU_PARAM_ST;
	const NAME: &'static str = "BaseChrSelectMenuParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for BaseChrSelectMenuParam {
	type Target = BASECHR_SELECT_MENU_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for BaseChrSelectMenuParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::BaseChrSelectMenuParam::BaseChrSelectMenuParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<BaseChrSelectMenuParam>(), 32)
	}
}
