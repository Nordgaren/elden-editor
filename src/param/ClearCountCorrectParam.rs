/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::CLEAR_COUNT_CORRECT_PARAM_ST::CLEAR_COUNT_CORRECT_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: CLEAR_COUNT_CORRECT_PARAM_ST

pub struct ClearCountCorrectParam {
    _data: CLEAR_COUNT_CORRECT_PARAM_ST,
}
impl Param for ClearCountCorrectParam {
    type Def = CLEAR_COUNT_CORRECT_PARAM_ST;
    const NAME: &'static str = "ClearCountCorrectParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for ClearCountCorrectParam {
    type Target = CLEAR_COUNT_CORRECT_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for ClearCountCorrectParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::ClearCountCorrectParam::ClearCountCorrectParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<ClearCountCorrectParam>(), 128)
    }
}
