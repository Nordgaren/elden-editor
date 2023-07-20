/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::WORLD_MAP_PLACE_NAME_PARAM_ST::WORLD_MAP_PLACE_NAME_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: WORLD_MAP_PLACE_NAME_PARAM_ST

pub struct WorldMapPlaceNameParam {
    _data: WORLD_MAP_PLACE_NAME_PARAM_ST,
}
impl Param for WorldMapPlaceNameParam {
    type Def = WORLD_MAP_PLACE_NAME_PARAM_ST;
    const NAME: &'static str = "WorldMapPlaceNameParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for WorldMapPlaceNameParam {
    type Target = WORLD_MAP_PLACE_NAME_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for WorldMapPlaceNameParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::WorldMapPlaceNameParam::WorldMapPlaceNameParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<WorldMapPlaceNameParam>(), 32)
    }
}
