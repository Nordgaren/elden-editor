/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/WORLD_MAP_POINT_PARAM_ST.rs");

/// Type: WORLD_MAP_POINT_PARAM_ST

pub type WorldMapPointParam = ParamStruct<WORLD_MAP_POINT_PARAM_ST>;
impl Param for ParamStruct<WORLD_MAP_POINT_PARAM_ST> {
    const NAME: &'static str = "WorldMapPointParam";
    const TYPE_NAME: &'static str = "WORLD_MAP_POINT_PARAM_ST";
    const VERSION: u16 = 6;
}

#[cfg(test)]
mod tests {
    use crate::param::WorldMapPointParam::WorldMapPointParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<WorldMapPointParam>(), 256)
    }
}
