/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::GRAPHICS_COMMON_PARAM_ST::GRAPHICS_COMMON_PARAM_ST;

/// Type: GRAPHICS_COMMON_PARAM_ST

pub struct GraphicsCommonParam {
	_data: GRAPHICS_COMMON_PARAM_ST
}
impl Param for GraphicsCommonParam {
	type Def = GRAPHICS_COMMON_PARAM_ST;
	const NAME: &'static str = "GraphicsCommonParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for GraphicsCommonParam {
	type Target = GRAPHICS_COMMON_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for GraphicsCommonParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::GraphicsCommonParam::GraphicsCommonParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<GraphicsCommonParam>(), 256)
	}
}
