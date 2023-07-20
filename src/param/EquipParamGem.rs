/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::EQUIP_PARAM_GEM_ST::EQUIP_PARAM_GEM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: EQUIP_PARAM_GEM_ST

pub struct EquipParamGem {
    _data: EQUIP_PARAM_GEM_ST,
}
impl Param for EquipParamGem {
    type Def = EQUIP_PARAM_GEM_ST;
    const NAME: &'static str = "EquipParamGem";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for EquipParamGem {
    type Target = EQUIP_PARAM_GEM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for EquipParamGem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::EquipParamGem::EquipParamGem;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<EquipParamGem>(), 96)
    }
}
