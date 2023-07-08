/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/GRASS_LOD_RANGE_PARAM_ST.rs");

/// Type: GRASS_LOD_RANGE_PARAM_ST

pub type GrassLodRangeParam = ParamStruct<GRASS_LOD_RANGE_PARAM_ST>;
impl Param for ParamStruct<GRASS_LOD_RANGE_PARAM_ST> {
    const NAME: &'static str = "GrassLodRangeParam";
    const TYPE_NAME: &'static str = "GRASS_LOD_RANGE_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::GrassLodRangeParam::GrassLodRangeParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<GrassLodRangeParam>(), 24)
    }
}
