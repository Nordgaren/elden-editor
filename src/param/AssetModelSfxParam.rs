/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/ASSET_MODEL_SFX_PARAM_ST.rs");

/// Type: ASSET_MODEL_SFX_PARAM_ST

pub type AssetModelSfxParam = ParamStruct<ASSET_MODEL_SFX_PARAM_ST>;
impl Param for ParamStruct<ASSET_MODEL_SFX_PARAM_ST> {
    const NAME: &'static str = "AssetModelSfxParam";
    const TYPE_NAME: &'static str = "ASSET_MODEL_SFX_PARAM_ST";
    const VERSION: u16 = 0;
}

#[cfg(test)]
mod tests {
    use crate::param::AssetModelSfxParam::AssetModelSfxParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<AssetModelSfxParam>(), 128)
    }
}
