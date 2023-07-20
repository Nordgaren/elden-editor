/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::MAP_DEFAULT_INFO_PARAM_ST::MAP_DEFAULT_INFO_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: MAP_DEFAULT_INFO_PARAM_ST

pub struct MapDefaultInfoParam {
    _data: MAP_DEFAULT_INFO_PARAM_ST,
}
impl Param for MapDefaultInfoParam {
    type Def = MAP_DEFAULT_INFO_PARAM_ST;
    const NAME: &'static str = "MapDefaultInfoParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for MapDefaultInfoParam {
    type Target = MAP_DEFAULT_INFO_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for MapDefaultInfoParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::MapDefaultInfoParam::MapDefaultInfoParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MapDefaultInfoParam>(), 64)
    }
}
