/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::MAP_GD_REGION_ID_PARAM_ST::MAP_GD_REGION_ID_PARAM_ST;

/// Type: MAP_GD_REGION_ID_PARAM_ST

pub struct MapGdRegionInfoParam {
	_data: MAP_GD_REGION_ID_PARAM_ST
}
impl Param for MapGdRegionInfoParam {
	type Def = MAP_GD_REGION_ID_PARAM_ST;
	const NAME: &'static str = "MapGdRegionInfoParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for MapGdRegionInfoParam {
	type Target = MAP_GD_REGION_ID_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for MapGdRegionInfoParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::MapGdRegionInfoParam::MapGdRegionInfoParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<MapGdRegionInfoParam>(), 32)
	}
}
