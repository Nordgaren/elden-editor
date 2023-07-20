/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST::MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST

pub struct MapGridCreateHeightLimitInfoParam {
    _data: MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST,
}
impl Param for MapGridCreateHeightLimitInfoParam {
    type Def = MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST;
    const NAME: &'static str = "MapGridCreateHeightLimitInfoParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for MapGridCreateHeightLimitInfoParam {
    type Target = MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for MapGridCreateHeightLimitInfoParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::MapGridCreateHeightLimitInfoParam::MapGridCreateHeightLimitInfoParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MapGridCreateHeightLimitInfoParam>(), 32)
    }
}
