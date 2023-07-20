/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::CUTSCENE_GPARAM_TIME_PARAM_ST::CUTSCENE_GPARAM_TIME_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: CUTSCENE_GPARAM_TIME_PARAM_ST

pub struct CutsceneGparamTimeParam {
    _data: CUTSCENE_GPARAM_TIME_PARAM_ST,
}
impl Param for CutsceneGparamTimeParam {
    type Def = CUTSCENE_GPARAM_TIME_PARAM_ST;
    const NAME: &'static str = "CutsceneGparamTimeParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for CutsceneGparamTimeParam {
    type Target = CUTSCENE_GPARAM_TIME_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for CutsceneGparamTimeParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::CutsceneGparamTimeParam::CutsceneGparamTimeParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<CutsceneGparamTimeParam>(), 16)
    }
}
