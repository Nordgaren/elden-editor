/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/RANDOM_APPEAR_PARAM_ST.rs");

/// Type: RANDOM_APPEAR_PARAM_ST

pub type RandomAppearParam = ParamStruct<RANDOM_APPEAR_PARAM_ST>;
impl Param for ParamStruct<RANDOM_APPEAR_PARAM_ST> {
    const NAME: &'static str = "RandomAppearParam";
    const TYPE_NAME: &'static str = "RANDOM_APPEAR_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::RandomAppearParam::RandomAppearParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<RandomAppearParam>(), 13)
    }
}
