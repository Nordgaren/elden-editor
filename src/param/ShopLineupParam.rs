/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::SHOP_LINEUP_PARAM::SHOP_LINEUP_PARAM;

/// Type: SHOP_LINEUP_PARAM

pub struct ShopLineupParam {
	_data: SHOP_LINEUP_PARAM
}
impl Param for ShopLineupParam {
	type Def = SHOP_LINEUP_PARAM;
	const NAME: &'static str = "ShopLineupParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for ShopLineupParam {
	type Target = SHOP_LINEUP_PARAM;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for ShopLineupParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ShopLineupParam::ShopLineupParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ShopLineupParam>(), 52)
	}
}
