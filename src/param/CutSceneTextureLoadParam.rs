/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::CUTSCENE_TEXTURE_LOAD_PARAM_ST::CUTSCENE_TEXTURE_LOAD_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: CUTSCENE_TEXTURE_LOAD_PARAM_ST

pub struct CutSceneTextureLoadParam {
    _data: CUTSCENE_TEXTURE_LOAD_PARAM_ST,
}
impl Param for CutSceneTextureLoadParam {
    type Def = CUTSCENE_TEXTURE_LOAD_PARAM_ST;
    const NAME: &'static str = "CutSceneTextureLoadParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for CutSceneTextureLoadParam {
    type Target = CUTSCENE_TEXTURE_LOAD_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for CutSceneTextureLoadParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::CutSceneTextureLoadParam::CutSceneTextureLoadParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<CutSceneTextureLoadParam>(), 260)
    }
}
