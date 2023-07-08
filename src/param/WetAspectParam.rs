/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/WET_ASPECT_PARAM_ST.rs");

/// Type: WET_ASPECT_PARAM_ST

pub type WetAspectParam = ParamStruct<WET_ASPECT_PARAM_ST>;
impl Param for ParamStruct<WET_ASPECT_PARAM_ST> {
    const NAME: &'static str = "WetAspectParam";
    const TYPE_NAME: &'static str = "WET_ASPECT_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::WetAspectParam::WetAspectParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<WetAspectParam>(), 32)
    }
}
