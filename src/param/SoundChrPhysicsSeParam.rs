/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SOUND_CHR_PHYSICS_SE_PARAM_ST.rs");

/// Type: SOUND_CHR_PHYSICS_SE_PARAM_ST

pub type SoundChrPhysicsSeParam = ParamStruct<SOUND_CHR_PHYSICS_SE_PARAM_ST>;
impl Param for ParamStruct<SOUND_CHR_PHYSICS_SE_PARAM_ST> {
    const NAME: &'static str = "SoundChrPhysicsSeParam";
    const TYPE_NAME: &'static str = "SOUND_CHR_PHYSICS_SE_PARAM_ST";
    const VERSION: u16 = 4;
}

#[cfg(test)]
mod tests {
    use crate::param::SoundChrPhysicsSeParam::SoundChrPhysicsSeParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SoundChrPhysicsSeParam>(), 56)
    }
}
