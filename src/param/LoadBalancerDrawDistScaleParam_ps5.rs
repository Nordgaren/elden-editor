/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::param::traits::*;

include!("defs/LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST.rs");

/// Type: LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST

pub type LoadBalancerDrawDistScaleParam_ps5 = ParamStruct<LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST>;
impl Param for ParamStruct<LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST> {
	const NAME: &'static str = "LoadBalancerDrawDistScaleParam_ps5";
	const TYPE_NAME: &'static str = "LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST";
	const VERSION: u16 = 0;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::LoadBalancerDrawDistScaleParam_ps5::LoadBalancerDrawDistScaleParam_ps5;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<LoadBalancerDrawDistScaleParam_ps5>(), 128)
	}
}
