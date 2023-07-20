/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::EQUIP_PARAM_ACCESSORY_ST::EQUIP_PARAM_ACCESSORY_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: EQUIP_PARAM_ACCESSORY_ST

pub struct EquipParamAccessory {
    _data: EQUIP_PARAM_ACCESSORY_ST,
}
impl Param for EquipParamAccessory {
    type Def = EQUIP_PARAM_ACCESSORY_ST;
    const NAME: &'static str = "EquipParamAccessory";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for EquipParamAccessory {
    type Target = EQUIP_PARAM_ACCESSORY_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for EquipParamAccessory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::EquipParamAccessory::EquipParamAccessory;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<EquipParamAccessory>(), 96)
    }
}
