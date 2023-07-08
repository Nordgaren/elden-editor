/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/HIT_MTRL_PARAM_ST.rs");

/// Type: HIT_MTRL_PARAM_ST

pub type HitMtrlParam = ParamStruct<HIT_MTRL_PARAM_ST>;
impl Param for ParamStruct<HIT_MTRL_PARAM_ST> {
    const NAME: &'static str = "HitMtrlParam";
    const TYPE_NAME: &'static str = "HIT_MTRL_PARAM_ST";
    const VERSION: u16 = 3;
}

#[cfg(test)]
mod tests {
    use crate::param::HitMtrlParam::HitMtrlParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<HitMtrlParam>(), 100)
    }
}
