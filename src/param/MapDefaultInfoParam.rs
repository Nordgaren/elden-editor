/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::MAP_DEFAULT_INFO_PARAM_ST::MAP_DEFAULT_INFO_PARAM_ST;

/// Type: MAP_DEFAULT_INFO_PARAM_ST

pub struct MapDefaultInfoParam {
	_data: MAP_DEFAULT_INFO_PARAM_ST
}
impl Param for MapDefaultInfoParam {
	type Def = MAP_DEFAULT_INFO_PARAM_ST;
	const NAME: &'static str = "MapDefaultInfoParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for MapDefaultInfoParam {
	type Target = MAP_DEFAULT_INFO_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for MapDefaultInfoParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MapDefaultInfoParam::MapDefaultInfoParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MapDefaultInfoParam>(), 64)
	}
}
