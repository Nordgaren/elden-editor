/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::SE_ACTIVATION_RANGE_PARAM_ST::SE_ACTIVATION_RANGE_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: SE_ACTIVATION_RANGE_PARAM_ST

pub struct SeActivationRangeParam {
    _data: SE_ACTIVATION_RANGE_PARAM_ST,
}
impl Param for SeActivationRangeParam {
    type Def = SE_ACTIVATION_RANGE_PARAM_ST;
    const NAME: &'static str = "SeActivationRangeParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for SeActivationRangeParam {
    type Target = SE_ACTIVATION_RANGE_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for SeActivationRangeParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::SeActivationRangeParam::SeActivationRangeParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SeActivationRangeParam>(), 4)
    }
}
