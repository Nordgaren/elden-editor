/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/WHITE_SIGN_COOL_TIME_PARAM_ST.rs");

/// Type: WHITE_SIGN_COOL_TIME_PARAM_ST

pub type WhiteSignCoolTimeParam = ParamStruct<WHITE_SIGN_COOL_TIME_PARAM_ST>;
impl Param for ParamStruct<WHITE_SIGN_COOL_TIME_PARAM_ST> {
    const NAME: &'static str = "WhiteSignCoolTimeParam";
    const TYPE_NAME: &'static str = "WHITE_SIGN_COOL_TIME_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::WhiteSignCoolTimeParam::WhiteSignCoolTimeParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<WhiteSignCoolTimeParam>(), 16)
    }
}
