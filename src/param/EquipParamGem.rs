/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/EQUIP_PARAM_GEM_ST.rs");

/// Type: EQUIP_PARAM_GEM_ST

pub type EquipParamGem = ParamStruct<EQUIP_PARAM_GEM_ST>;
impl Param for ParamStruct<EQUIP_PARAM_GEM_ST> {
    const NAME: &'static str = "EquipParamGem";
    const TYPE_NAME: &'static str = "EQUIP_PARAM_GEM_ST";
    const VERSION: u16 = 3;
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
