/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/GRASS_TYPE_PARAM_ST.rs");

/// Type: GRASS_TYPE_PARAM_ST

pub type GrassTypeParam_Lv1 = ParamStruct<GRASS_TYPE_PARAM_ST>;
impl Param for ParamStruct<GRASS_TYPE_PARAM_ST> {
    const NAME: &'static str = "GrassTypeParam_Lv1";
    const TYPE_NAME: &'static str = "GRASS_TYPE_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::GrassTypeParam_Lv1::GrassTypeParam_Lv1;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<GrassTypeParam_Lv1>(), 276)
    }
}
