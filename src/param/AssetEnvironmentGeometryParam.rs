/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/ASSET_GEOMETORY_PARAM_ST.rs");

/// Type: ASSET_GEOMETORY_PARAM_ST

pub type AssetEnvironmentGeometryParam = ParamStruct<ASSET_GEOMETORY_PARAM_ST>;
impl Param for ParamStruct<ASSET_GEOMETORY_PARAM_ST> {
    const NAME: &'static str = "AssetEnvironmentGeometryParam";
    const TYPE_NAME: &'static str = "ASSET_GEOMETORY_PARAM_ST";
    const VERSION: u16 = 6;
}

#[cfg(test)]
mod tests {
    use crate::param::AssetEnvironmentGeometryParam::AssetEnvironmentGeometryParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<AssetEnvironmentGeometryParam>(), 320)
    }
}
