/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::SOUND_CUTSCENE_PARAM_ST::SOUND_CUTSCENE_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: SOUND_CUTSCENE_PARAM_ST

pub struct SoundCutsceneParam {
    _data: SOUND_CUTSCENE_PARAM_ST,
}
impl Param for SoundCutsceneParam {
    type Def = SOUND_CUTSCENE_PARAM_ST;
    const NAME: &'static str = "SoundCutsceneParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for SoundCutsceneParam {
    type Target = SOUND_CUTSCENE_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for SoundCutsceneParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
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
