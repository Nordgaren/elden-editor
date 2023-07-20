/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::AI_SOUND_PARAM_ST::AI_SOUND_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: AI_SOUND_PARAM_ST

pub struct AiSoundParam {
    _data: AI_SOUND_PARAM_ST,
}
impl Param for AiSoundParam {
    type Def = AI_SOUND_PARAM_ST;
    const NAME: &'static str = "AiSoundParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for AiSoundParam {
    type Target = AI_SOUND_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for AiSoundParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
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
