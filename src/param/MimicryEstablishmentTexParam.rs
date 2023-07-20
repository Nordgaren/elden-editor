/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::MIMICRY_ESTABLISHMENT_TEX_PARAM_ST::MIMICRY_ESTABLISHMENT_TEX_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: MIMICRY_ESTABLISHMENT_TEX_PARAM_ST

pub struct MimicryEstablishmentTexParam {
    _data: MIMICRY_ESTABLISHMENT_TEX_PARAM_ST,
}
impl Param for MimicryEstablishmentTexParam {
    type Def = MIMICRY_ESTABLISHMENT_TEX_PARAM_ST;
    const NAME: &'static str = "MimicryEstablishmentTexParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for MimicryEstablishmentTexParam {
    type Target = MIMICRY_ESTABLISHMENT_TEX_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for MimicryEstablishmentTexParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::MimicryEstablishmentTexParam::MimicryEstablishmentTexParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MimicryEstablishmentTexParam>(), 16)
    }
}
