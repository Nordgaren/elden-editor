/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::WAYPOINT_PARAM_ST::WAYPOINT_PARAM_ST;

/// Type: WAYPOINT_PARAM_ST

pub struct WaypointParam {
	_data: WAYPOINT_PARAM_ST
}
impl Param for WaypointParam {
	type Def = WAYPOINT_PARAM_ST;
	const NAME: &'static str = "WaypointParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for WaypointParam {
	type Target = WAYPOINT_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for WaypointParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::WaypointParam::WaypointParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<WaypointParam>(), 16)
	}
}
