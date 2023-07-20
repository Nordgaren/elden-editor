/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::MAP_NAME_TEX_PARAM_ST::MAP_NAME_TEX_PARAM_ST;

/// Type: MAP_NAME_TEX_PARAM_ST

pub struct MapNameTexParam {
	_data: MAP_NAME_TEX_PARAM_ST
}
impl Param for MapNameTexParam {
	type Def = MAP_NAME_TEX_PARAM_ST;
	const NAME: &'static str = "MapNameTexParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for MapNameTexParam {
	type Target = MAP_NAME_TEX_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for MapNameTexParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MapNameTexParam::MapNameTexParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MapNameTexParam>(), 16)
	}
}
