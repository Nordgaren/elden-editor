/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::SOUND_AUTO_REVERB_SELECT_PARAM_ST::SOUND_AUTO_REVERB_SELECT_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: SOUND_AUTO_REVERB_SELECT_PARAM_ST

pub struct SoundAutoReverbSelectParam {
    _data: SOUND_AUTO_REVERB_SELECT_PARAM_ST,
}
impl Param for SoundAutoReverbSelectParam {
    type Def = SOUND_AUTO_REVERB_SELECT_PARAM_ST;
    const NAME: &'static str = "SoundAutoReverbSelectParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for SoundAutoReverbSelectParam {
    type Target = SOUND_AUTO_REVERB_SELECT_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for SoundAutoReverbSelectParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::SoundAutoReverbSelectParam::SoundAutoReverbSelectParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SoundAutoReverbSelectParam>(), 32)
    }
}
