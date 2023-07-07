/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/WAYPOINT_PARAM_ST.rs");

/// Type: WAYPOINT_PARAM_ST

pub type WaypointParam = ParamStruct<WAYPOINT_PARAM_ST>;
impl Param for ParamStruct<WAYPOINT_PARAM_ST> {
	const NAME: &'static str = "WaypointParam";
	const TYPE_NAME: &'static str = "WAYPOINT_PARAM_ST";
	const VERSION: u16 = 1;
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
