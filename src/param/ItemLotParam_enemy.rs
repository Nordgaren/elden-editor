/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::param::defs::ITEMLOT_PARAM_ST::ITEMLOT_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: ITEMLOT_PARAM_ST

pub struct ItemLotParam_enemy {
    _data: ITEMLOT_PARAM_ST,
}

impl Param for ItemLotParam_enemy {
    type Def = ITEMLOT_PARAM_ST;
    const NAME: &'static str = "ItemLotParam_enemy";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for ItemLotParam_enemy {
    type Target = ITEMLOT_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for ItemLotParam_enemy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::ItemLotParam_enemy::ItemLotParam_enemy;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<ItemLotParam_enemy>(), 152)
    }
}
