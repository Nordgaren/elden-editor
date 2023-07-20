/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::RANDOM_APPEAR_PARAM_ST::RANDOM_APPEAR_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: RANDOM_APPEAR_PARAM_ST

pub struct RandomAppearParam {
    _data: RANDOM_APPEAR_PARAM_ST,
}
impl Param for RandomAppearParam {
    type Def = RANDOM_APPEAR_PARAM_ST;
    const NAME: &'static str = "RandomAppearParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for RandomAppearParam {
    type Target = RANDOM_APPEAR_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for RandomAppearParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::RandomAppearParam::RandomAppearParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<RandomAppearParam>(), 13)
    }
}
