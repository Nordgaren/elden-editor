/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::SOUND_COMMON_INGAME_PARAM_ST::SOUND_COMMON_INGAME_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: SOUND_COMMON_INGAME_PARAM_ST

pub struct SoundCommonIngameParam {
    _data: SOUND_COMMON_INGAME_PARAM_ST,
}
impl Param for SoundCommonIngameParam {
    type Def = SOUND_COMMON_INGAME_PARAM_ST;
    const NAME: &'static str = "SoundCommonIngameParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for SoundCommonIngameParam {
    type Target = SOUND_COMMON_INGAME_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for SoundCommonIngameParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
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
