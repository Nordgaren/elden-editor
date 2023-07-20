/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::GRASS_TYPE_PARAM_ST::GRASS_TYPE_PARAM_ST;

/// Type: GRASS_TYPE_PARAM_ST

pub struct GrassTypeParam {
	_data: GRASS_TYPE_PARAM_ST
}
impl Param for GrassTypeParam {
	type Def = GRASS_TYPE_PARAM_ST;
	const NAME: &'static str = "GrassTypeParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for GrassTypeParam {
	type Target = GRASS_TYPE_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for GrassTypeParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::GrassTypeParam::GrassTypeParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<GrassTypeParam>(), 276)
	}
}
