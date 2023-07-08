/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST.rs");

/// Type: SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST

pub type SoundAssetSoundObjEnableDistParam =
    ParamStruct<SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST>;
impl Param for ParamStruct<SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST> {
    const NAME: &'static str = "SoundAssetSoundObjEnableDistParam";
    const TYPE_NAME: &'static str = "SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::SoundAssetSoundObjEnableDistParam::SoundAssetSoundObjEnableDistParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SoundAssetSoundObjEnableDistParam>(), 4)
    }
}
