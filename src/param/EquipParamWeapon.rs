/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::EQUIP_PARAM_WEAPON_ST::EQUIP_PARAM_WEAPON_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: EQUIP_PARAM_WEAPON_ST

pub struct EquipParamWeapon {
    _data: EQUIP_PARAM_WEAPON_ST,
}

impl Param for EquipParamWeapon {
    type Def = EQUIP_PARAM_WEAPON_ST;
    const NAME: &'static str = "EquipParamWeapon";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for EquipParamWeapon {
    type Target = EQUIP_PARAM_WEAPON_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for EquipParamWeapon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::EquipParamWeapon::EquipParamWeapon;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<EquipParamWeapon>(), 664)
    }
}
