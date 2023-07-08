/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/ENV_OBJ_LOT_PARAM_ST.rs");

/// Type: ENV_OBJ_LOT_PARAM_ST

pub type EnvObjLotParam = ParamStruct<ENV_OBJ_LOT_PARAM_ST>;
impl Param for ParamStruct<ENV_OBJ_LOT_PARAM_ST> {
    const NAME: &'static str = "EnvObjLotParam";
    const TYPE_NAME: &'static str = "ENV_OBJ_LOT_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::EnvObjLotParam::EnvObjLotParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<EnvObjLotParam>(), 64)
    }
}
