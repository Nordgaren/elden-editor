/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::DECAL_PARAM_ST::DECAL_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: DECAL_PARAM_ST

pub struct DecalParam {
    _data: DECAL_PARAM_ST,
}
impl Param for DecalParam {
    type Def = DECAL_PARAM_ST;
    const NAME: &'static str = "DecalParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for DecalParam {
    type Target = DECAL_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for DecalParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::DecalParam::DecalParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<DecalParam>(), 248)
    }
}
