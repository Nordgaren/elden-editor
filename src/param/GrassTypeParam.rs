/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::GRASS_TYPE_PARAM_ST::GRASS_TYPE_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: GRASS_TYPE_PARAM_ST

pub struct GrassTypeParam {
    _data: GRASS_TYPE_PARAM_ST,
}
impl Param for GrassTypeParam {
    type Def = GRASS_TYPE_PARAM_ST;
    const NAME: &'static str = "GrassTypeParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for GrassTypeParam {
    type Target = GRASS_TYPE_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for GrassTypeParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::GrassTypeParam::GrassTypeParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<GrassTypeParam>(), 276)
    }
}
