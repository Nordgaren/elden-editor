/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::SHOP_LINEUP_PARAM::SHOP_LINEUP_PARAM;

/// Type: SHOP_LINEUP_PARAM

pub struct ShopLineupParam_Recipe {
	_data: SHOP_LINEUP_PARAM
}
impl Param for ShopLineupParam_Recipe {
	type Def = SHOP_LINEUP_PARAM;
	const NAME: &'static str = "ShopLineupParam_Recipe";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for ShopLineupParam_Recipe {
	type Target = SHOP_LINEUP_PARAM;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for ShopLineupParam_Recipe {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ShopLineupParam_Recipe::ShopLineupParam_Recipe;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ShopLineupParam_Recipe>(), 52)
	}
}
