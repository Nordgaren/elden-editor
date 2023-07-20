/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::BONFIRE_WARP_SUB_CATEGORY_PARAM_ST::BONFIRE_WARP_SUB_CATEGORY_PARAM_ST;

/// Type: BONFIRE_WARP_SUB_CATEGORY_PARAM_ST

pub struct BonfireWarpSubCategoryParam {
	_data: BONFIRE_WARP_SUB_CATEGORY_PARAM_ST
}
impl Param for BonfireWarpSubCategoryParam {
	type Def = BONFIRE_WARP_SUB_CATEGORY_PARAM_ST;
	const NAME: &'static str = "BonfireWarpSubCategoryParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for BonfireWarpSubCategoryParam {
	type Target = BONFIRE_WARP_SUB_CATEGORY_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for BonfireWarpSubCategoryParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::BonfireWarpSubCategoryParam::BonfireWarpSubCategoryParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<BonfireWarpSubCategoryParam>(), 16)
	}
}
