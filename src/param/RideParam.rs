/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/RIDE_PARAM_ST.rs");

/// Type: RIDE_PARAM_ST

pub type RideParam = ParamStruct<RIDE_PARAM_ST>;
impl Param for ParamStruct<RIDE_PARAM_ST> {
    const NAME: &'static str = "RideParam";
    const TYPE_NAME: &'static str = "RIDE_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::RideParam::RideParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<RideParam>(), 64)
    }
}
