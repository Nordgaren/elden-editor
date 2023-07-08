/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SOUND_COMMON_INGAME_PARAM_ST.rs");

/// Type: SOUND_COMMON_INGAME_PARAM_ST

pub type SoundCommonIngameParam = ParamStruct<SOUND_COMMON_INGAME_PARAM_ST>;
impl Param for ParamStruct<SOUND_COMMON_INGAME_PARAM_ST> {
    const NAME: &'static str = "SoundCommonIngameParam";
    const TYPE_NAME: &'static str = "SOUND_COMMON_INGAME_PARAM_ST";
    const VERSION: u16 = 0;
}

#[cfg(test)]
mod tests {
    use crate::param::SoundCommonIngameParam::SoundCommonIngameParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SoundCommonIngameParam>(), 64)
    }
}
