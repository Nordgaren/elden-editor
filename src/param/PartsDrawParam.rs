/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::PARTS_DRAW_PARAM_ST::PARTS_DRAW_PARAM_ST;

/// Type: PARTS_DRAW_PARAM_ST

pub struct PartsDrawParam {
	_data: PARTS_DRAW_PARAM_ST
}
impl Param for PartsDrawParam {
	type Def = PARTS_DRAW_PARAM_ST;
	const NAME: &'static str = "PartsDrawParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for PartsDrawParam {
	type Target = PARTS_DRAW_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for PartsDrawParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::PartsDrawParam::PartsDrawParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<PartsDrawParam>(), 144)
	}
}
