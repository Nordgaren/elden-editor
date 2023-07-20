/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::WORLD_MAP_POINT_PARAM_ST::WORLD_MAP_POINT_PARAM_ST;

/// Type: WORLD_MAP_POINT_PARAM_ST

pub struct WorldMapPointParam {
	_data: WORLD_MAP_POINT_PARAM_ST
}
impl Param for WorldMapPointParam {
	type Def = WORLD_MAP_POINT_PARAM_ST;
	const NAME: &'static str = "WorldMapPointParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for WorldMapPointParam {
	type Target = WORLD_MAP_POINT_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for WorldMapPointParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::WorldMapPointParam::WorldMapPointParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<WorldMapPointParam>(), 256)
	}
}
