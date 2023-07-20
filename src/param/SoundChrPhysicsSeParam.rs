/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::SOUND_CHR_PHYSICS_SE_PARAM_ST::SOUND_CHR_PHYSICS_SE_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: SOUND_CHR_PHYSICS_SE_PARAM_ST

pub struct SoundChrPhysicsSeParam {
    _data: SOUND_CHR_PHYSICS_SE_PARAM_ST,
}
impl Param for SoundChrPhysicsSeParam {
    type Def = SOUND_CHR_PHYSICS_SE_PARAM_ST;
    const NAME: &'static str = "SoundChrPhysicsSeParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for SoundChrPhysicsSeParam {
    type Target = SOUND_CHR_PHYSICS_SE_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for SoundChrPhysicsSeParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
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
