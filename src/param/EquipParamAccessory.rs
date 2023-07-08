/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/EQUIP_PARAM_ACCESSORY_ST.rs");

/// Type: EQUIP_PARAM_ACCESSORY_ST

pub type EquipParamAccessory = ParamStruct<EQUIP_PARAM_ACCESSORY_ST>;
impl Param for ParamStruct<EQUIP_PARAM_ACCESSORY_ST> {
    const NAME: &'static str = "EquipParamAccessory";
    const TYPE_NAME: &'static str = "EQUIP_PARAM_ACCESSORY_ST";
    const VERSION: u16 = 2;
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
