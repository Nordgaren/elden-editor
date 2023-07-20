/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::GRASS_LOD_RANGE_PARAM_ST::GRASS_LOD_RANGE_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: GRASS_LOD_RANGE_PARAM_ST

pub struct GrassLodRangeParam {
    _data: GRASS_LOD_RANGE_PARAM_ST,
}
impl Param for GrassLodRangeParam {
    type Def = GRASS_LOD_RANGE_PARAM_ST;
    const NAME: &'static str = "GrassLodRangeParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for GrassLodRangeParam {
    type Target = GRASS_LOD_RANGE_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for GrassLodRangeParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::GrassLodRangeParam::GrassLodRangeParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<GrassLodRangeParam>(), 24)
    }
}
