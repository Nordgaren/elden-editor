/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/EQUIP_PARAM_PROTECTOR_ST.rs");

/// Type: EQUIP_PARAM_PROTECTOR_ST

pub type EquipParamProtector = ParamStruct<EQUIP_PARAM_PROTECTOR_ST>;
impl Param for ParamStruct<EQUIP_PARAM_PROTECTOR_ST> {
    const NAME: &'static str = "EquipParamProtector";
    const TYPE_NAME: &'static str = "EQUIP_PARAM_PROTECTOR_ST";
    const VERSION: u16 = 6;
}

#[cfg(test)]
mod tests {
    use crate::param::EquipParamProtector::EquipParamProtector;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<EquipParamProtector>(), 416)
    }
}
