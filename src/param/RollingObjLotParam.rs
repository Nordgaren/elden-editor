/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/ROLLING_OBJ_LOT_PARAM_ST.rs");

/// Type: ROLLING_OBJ_LOT_PARAM_ST

pub type RollingObjLotParam = ParamStruct<ROLLING_OBJ_LOT_PARAM_ST>;
impl Param for ParamStruct<ROLLING_OBJ_LOT_PARAM_ST> {
    const NAME: &'static str = "RollingObjLotParam";
    const TYPE_NAME: &'static str = "ROLLING_OBJ_LOT_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::RollingObjLotParam::RollingObjLotParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<RollingObjLotParam>(), 64)
    }
}
