/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/ITEMLOT_PARAM_ST.rs");

/// Type: ITEMLOT_PARAM_ST

pub type ItemLotParam_map = ParamStruct<ITEMLOT_PARAM_ST>;
impl Param for ParamStruct<ITEMLOT_PARAM_ST> {
    const NAME: &'static str = "ItemLotParam_map";
    const TYPE_NAME: &'static str = "ITEMLOT_PARAM_ST";
    const VERSION: u16 = 4;
}

#[cfg(test)]
mod tests {
    use crate::param::ItemLotParam_map::ItemLotParam_map;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<ItemLotParam_map>(), 152)
    }
}
