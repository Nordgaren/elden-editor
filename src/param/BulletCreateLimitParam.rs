/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/BULLET_CREATE_LIMIT_PARAM_ST.rs");

/// Type: BULLET_CREATE_LIMIT_PARAM_ST

pub type BulletCreateLimitParam = ParamStruct<BULLET_CREATE_LIMIT_PARAM_ST>;
impl Param for ParamStruct<BULLET_CREATE_LIMIT_PARAM_ST> {
    const NAME: &'static str = "BulletCreateLimitParam";
    const TYPE_NAME: &'static str = "BULLET_CREATE_LIMIT_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::BulletCreateLimitParam::BulletCreateLimitParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<BulletCreateLimitParam>(), 32)
    }
}
