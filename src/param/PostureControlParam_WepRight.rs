/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/POSTURE_CONTROL_PARAM_WEP_RIGHT_ST.rs");

/// Type: POSTURE_CONTROL_PARAM_WEP_RIGHT_ST

pub type PostureControlParam_WepRight = ParamStruct<POSTURE_CONTROL_PARAM_WEP_RIGHT_ST>;
impl Param for ParamStruct<POSTURE_CONTROL_PARAM_WEP_RIGHT_ST> {
    const NAME: &'static str = "PostureControlParam_WepRight";
    const TYPE_NAME: &'static str = "POSTURE_CONTROL_PARAM_WEP_RIGHT_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::PostureControlParam_WepRight::PostureControlParam_WepRight;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<PostureControlParam_WepRight>(), 112)
    }
}
