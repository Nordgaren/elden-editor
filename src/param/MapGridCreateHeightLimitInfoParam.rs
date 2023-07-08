/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST.rs");

/// Type: MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST

pub type MapGridCreateHeightLimitInfoParam =
    ParamStruct<MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST>;
impl Param for ParamStruct<MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST> {
    const NAME: &'static str = "MapGridCreateHeightLimitInfoParam";
    const TYPE_NAME: &'static str = "MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::MapGridCreateHeightLimitInfoParam::MapGridCreateHeightLimitInfoParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MapGridCreateHeightLimitInfoParam>(), 32)
    }
}
