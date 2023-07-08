/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/ASSET_MATERIAL_SFX_PARAM_ST.rs");

/// Type: ASSET_MATERIAL_SFX_PARAM_ST

pub type AssetMaterialSfxParam = ParamStruct<ASSET_MATERIAL_SFX_PARAM_ST>;
impl Param for ParamStruct<ASSET_MATERIAL_SFX_PARAM_ST> {
    const NAME: &'static str = "AssetMaterialSfxParam";
    const TYPE_NAME: &'static str = "ASSET_MATERIAL_SFX_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::AssetMaterialSfxParam::AssetMaterialSfxParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<AssetMaterialSfxParam>(), 128)
    }
}
