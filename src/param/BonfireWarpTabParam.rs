/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::BONFIRE_WARP_TAB_PARAM_ST::BONFIRE_WARP_TAB_PARAM_ST;

/// Type: BONFIRE_WARP_TAB_PARAM_ST

pub struct BonfireWarpTabParam {
	_data: BONFIRE_WARP_TAB_PARAM_ST
}
impl Param for BonfireWarpTabParam {
	type Def = BONFIRE_WARP_TAB_PARAM_ST;
	const NAME: &'static str = "BonfireWarpTabParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for BonfireWarpTabParam {
	type Target = BONFIRE_WARP_TAB_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for BonfireWarpTabParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::BonfireWarpTabParam::BonfireWarpTabParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<BonfireWarpTabParam>(), 16)
	}
}
