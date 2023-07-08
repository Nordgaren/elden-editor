/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MAP_GD_REGION_ID_PARAM_ST.rs");

/// Type: MAP_GD_REGION_ID_PARAM_ST

pub type MapGdRegionInfoParam = ParamStruct<MAP_GD_REGION_ID_PARAM_ST>;
impl Param for ParamStruct<MAP_GD_REGION_ID_PARAM_ST> {
    const NAME: &'static str = "MapGdRegionInfoParam";
    const TYPE_NAME: &'static str = "MAP_GD_REGION_ID_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::MapGdRegionInfoParam::MapGdRegionInfoParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MapGdRegionInfoParam>(), 32)
    }
}
