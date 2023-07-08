/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/NETWORK_AREA_PARAM_ST.rs");

/// Type: NETWORK_AREA_PARAM_ST

pub type NetworkAreaParam = ParamStruct<NETWORK_AREA_PARAM_ST>;
impl Param for ParamStruct<NETWORK_AREA_PARAM_ST> {
    const NAME: &'static str = "NetworkAreaParam";
    const TYPE_NAME: &'static str = "NETWORK_AREA_PARAM_ST";
    const VERSION: u16 = 4;
}

#[cfg(test)]
mod tests {
    use crate::param::NetworkAreaParam::NetworkAreaParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<NetworkAreaParam>(), 28)
    }
}
