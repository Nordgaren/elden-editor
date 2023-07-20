/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::PLAY_REGION_PARAM_ST::PLAY_REGION_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: PLAY_REGION_PARAM_ST

pub struct PlayRegionParam {
    _data: PLAY_REGION_PARAM_ST,
}
impl Param for PlayRegionParam {
    type Def = PLAY_REGION_PARAM_ST;
    const NAME: &'static str = "PlayRegionParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for PlayRegionParam {
    type Target = PLAY_REGION_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for PlayRegionParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::PlayRegionParam::PlayRegionParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<PlayRegionParam>(), 272)
    }
}
