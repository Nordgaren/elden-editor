/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/WORLD_MAP_LEGACY_CONV_PARAM_ST.rs");

/// Type: WORLD_MAP_LEGACY_CONV_PARAM_ST

pub type WorldMapLegacyConvParam = ParamStruct<WORLD_MAP_LEGACY_CONV_PARAM_ST>;
impl Param for ParamStruct<WORLD_MAP_LEGACY_CONV_PARAM_ST> {
    const NAME: &'static str = "WorldMapLegacyConvParam";
    const TYPE_NAME: &'static str = "WORLD_MAP_LEGACY_CONV_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::WorldMapLegacyConvParam::WorldMapLegacyConvParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<WorldMapLegacyConvParam>(), 48)
    }
}
