/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SOUND_CUTSCENE_PARAM_ST.rs");

/// Type: SOUND_CUTSCENE_PARAM_ST

pub type SoundCutsceneParam = ParamStruct<SOUND_CUTSCENE_PARAM_ST>;
impl Param for ParamStruct<SOUND_CUTSCENE_PARAM_ST> {
    const NAME: &'static str = "SoundCutsceneParam";
    const TYPE_NAME: &'static str = "SOUND_CUTSCENE_PARAM_ST";
    const VERSION: u16 = 6;
}

#[cfg(test)]
mod tests {
    use crate::param::SoundCutsceneParam::SoundCutsceneParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SoundCutsceneParam>(), 36)
    }
}
