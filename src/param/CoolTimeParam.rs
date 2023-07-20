/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::COOL_TIME_PARAM_ST::COOL_TIME_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: COOL_TIME_PARAM_ST

pub struct CoolTimeParam {
    _data: COOL_TIME_PARAM_ST,
}
impl Param for CoolTimeParam {
    type Def = COOL_TIME_PARAM_ST;
    const NAME: &'static str = "CoolTimeParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for CoolTimeParam {
    type Target = COOL_TIME_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for CoolTimeParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::CoolTimeParam::CoolTimeParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<CoolTimeParam>(), 32)
    }
}
