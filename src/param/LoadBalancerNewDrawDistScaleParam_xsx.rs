/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST.rs");

/// Type: LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST

pub type LoadBalancerNewDrawDistScaleParam_xsx =
    ParamStruct<LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST>;
impl Param for ParamStruct<LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST> {
    const NAME: &'static str = "LoadBalancerNewDrawDistScaleParam_xsx";
    const TYPE_NAME: &'static str = "LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST";
    const VERSION: u16 = 0;
}

#[cfg(test)]
mod tests {
    use crate::param::LoadBalancerNewDrawDistScaleParam_xsx::LoadBalancerNewDrawDistScaleParam_xsx;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<LoadBalancerNewDrawDistScaleParam_xsx>(), 48)
    }
}
