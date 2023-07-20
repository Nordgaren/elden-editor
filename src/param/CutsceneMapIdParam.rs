/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::CUTSCENE_MAP_ID_PARAM_ST::CUTSCENE_MAP_ID_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: CUTSCENE_MAP_ID_PARAM_ST

pub struct CutsceneMapIdParam {
    _data: CUTSCENE_MAP_ID_PARAM_ST,
}
impl Param for CutsceneMapIdParam {
    type Def = CUTSCENE_MAP_ID_PARAM_ST;
    const NAME: &'static str = "CutsceneMapIdParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for CutsceneMapIdParam {
    type Target = CUTSCENE_MAP_ID_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for CutsceneMapIdParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::CutsceneMapIdParam::CutsceneMapIdParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<CutsceneMapIdParam>(), 48)
    }
}
