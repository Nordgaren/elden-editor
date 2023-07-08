/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST.rs");

/// Type: SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST

pub type SoundAutoReverbEvaluationDistParam =
    ParamStruct<SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST>;
impl Param for ParamStruct<SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST> {
    const NAME: &'static str = "SoundAutoReverbEvaluationDistParam";
    const TYPE_NAME: &'static str = "SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::SoundAutoReverbEvaluationDistParam::SoundAutoReverbEvaluationDistParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SoundAutoReverbEvaluationDistParam>(), 20)
    }
}
