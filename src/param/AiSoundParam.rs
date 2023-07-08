/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/AI_SOUND_PARAM_ST.rs");

/// Type: AI_SOUND_PARAM_ST

pub type AiSoundParam = ParamStruct<AI_SOUND_PARAM_ST>;
impl Param for ParamStruct<AI_SOUND_PARAM_ST> {
    const NAME: &'static str = "AiSoundParam";
    const TYPE_NAME: &'static str = "AI_SOUND_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::AiSoundParam::AiSoundParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<AiSoundParam>(), 32)
    }
}
