/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::MAP_GD_REGION_DRAW_PARAM::MAP_GD_REGION_DRAW_PARAM;

/// Type: MAP_GD_REGION_DRAW_PARAM

pub struct MapGdRegionDrawParam {
	_data: MAP_GD_REGION_DRAW_PARAM
}
impl Param for MapGdRegionDrawParam {
	type Def = MAP_GD_REGION_DRAW_PARAM;
	const NAME: &'static str = "MapGdRegionDrawParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for MapGdRegionDrawParam {
	type Target = MAP_GD_REGION_DRAW_PARAM;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for MapGdRegionDrawParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MapGdRegionDrawParam::MapGdRegionDrawParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MapGdRegionDrawParam>(), 8)
	}
}
