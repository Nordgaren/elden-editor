/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/LOCK_CAM_PARAM_ST.rs");

/// Type: LOCK_CAM_PARAM_ST

pub type LockCamParam = ParamStruct<LOCK_CAM_PARAM_ST>;
impl Param for ParamStruct<LOCK_CAM_PARAM_ST> {
    const NAME: &'static str = "LockCamParam";
    const TYPE_NAME: &'static str = "LOCK_CAM_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::LockCamParam::LockCamParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<LockCamParam>(), 128)
    }
}
