/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::SE_MATERIAL_CONVERT_PARAM_ST::SE_MATERIAL_CONVERT_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: SE_MATERIAL_CONVERT_PARAM_ST

pub struct SeMaterialConvertParam {
    _data: SE_MATERIAL_CONVERT_PARAM_ST,
}
impl Param for SeMaterialConvertParam {
    type Def = SE_MATERIAL_CONVERT_PARAM_ST;
    const NAME: &'static str = "SeMaterialConvertParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for SeMaterialConvertParam {
    type Target = SE_MATERIAL_CONVERT_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for SeMaterialConvertParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::SeMaterialConvertParam::SeMaterialConvertParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SeMaterialConvertParam>(), 4)
    }
}
