/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/BONFIRE_WARP_PARAM_ST.rs");

/// Type: BONFIRE_WARP_PARAM_ST

pub type BonfireWarpParam = ParamStruct<BONFIRE_WARP_PARAM_ST>;
impl Param for ParamStruct<BONFIRE_WARP_PARAM_ST> {
    const NAME: &'static str = "BonfireWarpParam";
    const TYPE_NAME: &'static str = "BONFIRE_WARP_PARAM_ST";
    const VERSION: u16 = 6;
}

#[cfg(test)]
mod tests {
    use crate::param::BonfireWarpParam::BonfireWarpParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<BonfireWarpParam>(), 236)
    }
}
