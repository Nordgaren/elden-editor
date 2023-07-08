/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/AUTO_CREATE_ENV_SOUND_PARAM_ST.rs");

/// Type: AUTO_CREATE_ENV_SOUND_PARAM_ST

pub type AutoCreateEnvSoundParam = ParamStruct<AUTO_CREATE_ENV_SOUND_PARAM_ST>;
impl Param for ParamStruct<AUTO_CREATE_ENV_SOUND_PARAM_ST> {
    const NAME: &'static str = "AutoCreateEnvSoundParam";
    const TYPE_NAME: &'static str = "AUTO_CREATE_ENV_SOUND_PARAM_ST";
    const VERSION: u16 = 0;
}

#[cfg(test)]
mod tests {
    use crate::param::AutoCreateEnvSoundParam::AutoCreateEnvSoundParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<AutoCreateEnvSoundParam>(), 32)
    }
}
