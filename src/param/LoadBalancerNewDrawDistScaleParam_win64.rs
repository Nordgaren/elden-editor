/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::param::traits::*;

include!("defs/LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST.rs");

/// Type: LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST

pub type LoadBalancerNewDrawDistScaleParam_win64 = ParamStruct<LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST>;
impl Param for ParamStruct<LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST> {
	const NAME: &'static str = "LoadBalancerNewDrawDistScaleParam_win64";
	const TYPE_NAME: &'static str = "LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST";
	const VERSION: u16 = 0;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::LoadBalancerNewDrawDistScaleParam_win64::LoadBalancerNewDrawDistScaleParam_win64;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<LoadBalancerNewDrawDistScaleParam_win64>(), 48)
	}
}
