/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::ASSET_GEOMETORY_PARAM_ST::ASSET_GEOMETORY_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: ASSET_GEOMETORY_PARAM_ST

pub struct AssetEnvironmentGeometryParam {
    _data: ASSET_GEOMETORY_PARAM_ST,
}
impl Param for AssetEnvironmentGeometryParam {
    type Def = ASSET_GEOMETORY_PARAM_ST;
    const NAME: &'static str = "AssetEnvironmentGeometryParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for AssetEnvironmentGeometryParam {
    type Target = ASSET_GEOMETORY_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for AssetEnvironmentGeometryParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::AssetEnvironmentGeometryParam::AssetEnvironmentGeometryParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<AssetEnvironmentGeometryParam>(), 320)
    }
}
