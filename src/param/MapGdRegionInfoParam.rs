/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::MAP_GD_REGION_ID_PARAM_ST::MAP_GD_REGION_ID_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: MAP_GD_REGION_ID_PARAM_ST

pub struct MapGdRegionInfoParam {
    _data: MAP_GD_REGION_ID_PARAM_ST,
}
impl Param for MapGdRegionInfoParam {
    type Def = MAP_GD_REGION_ID_PARAM_ST;
    const NAME: &'static str = "MapGdRegionInfoParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for MapGdRegionInfoParam {
    type Target = MAP_GD_REGION_ID_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for MapGdRegionInfoParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::MapGdRegionInfoParam::MapGdRegionInfoParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MapGdRegionInfoParam>(), 32)
    }
}
