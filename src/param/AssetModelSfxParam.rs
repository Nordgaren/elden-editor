/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::ASSET_MODEL_SFX_PARAM_ST::ASSET_MODEL_SFX_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: ASSET_MODEL_SFX_PARAM_ST

pub struct AssetModelSfxParam {
    _data: ASSET_MODEL_SFX_PARAM_ST,
}
impl Param for AssetModelSfxParam {
    type Def = ASSET_MODEL_SFX_PARAM_ST;
    const NAME: &'static str = "AssetModelSfxParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for AssetModelSfxParam {
    type Target = ASSET_MODEL_SFX_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for AssetModelSfxParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::AssetModelSfxParam::AssetModelSfxParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<AssetModelSfxParam>(), 128)
    }
}
