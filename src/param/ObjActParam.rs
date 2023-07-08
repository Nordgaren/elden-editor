/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/OBJ_ACT_PARAM_ST.rs");

/// Type: OBJ_ACT_PARAM_ST

pub type ObjActParam = ParamStruct<OBJ_ACT_PARAM_ST>;
impl Param for ParamStruct<OBJ_ACT_PARAM_ST> {
    const NAME: &'static str = "ObjActParam";
    const TYPE_NAME: &'static str = "OBJ_ACT_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::ObjActParam::ObjActParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<ObjActParam>(), 96)
    }
}
