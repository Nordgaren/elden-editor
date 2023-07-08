/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MATERIAL_EX_PARAM_ST.rs");

/// Type: MATERIAL_EX_PARAM_ST

pub type MaterialExParam = ParamStruct<MATERIAL_EX_PARAM_ST>;
impl Param for ParamStruct<MATERIAL_EX_PARAM_ST> {
    const NAME: &'static str = "MaterialExParam";
    const TYPE_NAME: &'static str = "MATERIAL_EX_PARAM_ST";
    const VERSION: u16 = 4;
}

#[cfg(test)]
mod tests {
    use crate::param::MaterialExParam::MaterialExParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MaterialExParam>(), 96)
    }
}
