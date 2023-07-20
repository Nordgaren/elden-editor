/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::TOUGHNESS_PARAM_ST::TOUGHNESS_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: TOUGHNESS_PARAM_ST

pub struct ToughnessParam {
    _data: TOUGHNESS_PARAM_ST,
}
impl Param for ToughnessParam {
    type Def = TOUGHNESS_PARAM_ST;
    const NAME: &'static str = "ToughnessParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for ToughnessParam {
    type Target = TOUGHNESS_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for ToughnessParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::ToughnessParam::ToughnessParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<ToughnessParam>(), 32)
    }
}
