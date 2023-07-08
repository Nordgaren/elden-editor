/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/BONFIRE_WARP_TAB_PARAM_ST.rs");

/// Type: BONFIRE_WARP_TAB_PARAM_ST

pub type BonfireWarpTabParam = ParamStruct<BONFIRE_WARP_TAB_PARAM_ST>;
impl Param for ParamStruct<BONFIRE_WARP_TAB_PARAM_ST> {
    const NAME: &'static str = "BonfireWarpTabParam";
    const TYPE_NAME: &'static str = "BONFIRE_WARP_TAB_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::BonfireWarpTabParam::BonfireWarpTabParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<BonfireWarpTabParam>(), 16)
    }
}
