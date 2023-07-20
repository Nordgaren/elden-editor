/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::MENUPROPERTY_LAYOUT::MENUPROPERTY_LAYOUT;

/// Type: MENUPROPERTY_LAYOUT

pub struct MenuPropertyLayoutParam {
	_data: MENUPROPERTY_LAYOUT
}
impl Param for MenuPropertyLayoutParam {
	type Def = MENUPROPERTY_LAYOUT;
	const NAME: &'static str = "MenuPropertyLayoutParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for MenuPropertyLayoutParam {
	type Target = MENUPROPERTY_LAYOUT;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for MenuPropertyLayoutParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MenuPropertyLayoutParam::MenuPropertyLayoutParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MenuPropertyLayoutParam>(), 32)
	}
}
