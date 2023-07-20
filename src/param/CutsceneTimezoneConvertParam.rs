/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::CUTSCENE_TIMEZONE_CONVERT_PARAM_ST::CUTSCENE_TIMEZONE_CONVERT_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: CUTSCENE_TIMEZONE_CONVERT_PARAM_ST

pub struct CutsceneTimezoneConvertParam {
    _data: CUTSCENE_TIMEZONE_CONVERT_PARAM_ST,
}
impl Param for CutsceneTimezoneConvertParam {
    type Def = CUTSCENE_TIMEZONE_CONVERT_PARAM_ST;
    const NAME: &'static str = "CutsceneTimezoneConvertParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for CutsceneTimezoneConvertParam {
    type Target = CUTSCENE_TIMEZONE_CONVERT_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for CutsceneTimezoneConvertParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::CutsceneTimezoneConvertParam::CutsceneTimezoneConvertParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<CutsceneTimezoneConvertParam>(), 8)
    }
}
