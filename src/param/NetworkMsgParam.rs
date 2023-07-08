/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/NETWORK_MSG_PARAM_ST.rs");

/// Type: NETWORK_MSG_PARAM_ST

pub type NetworkMsgParam = ParamStruct<NETWORK_MSG_PARAM_ST>;
impl Param for ParamStruct<NETWORK_MSG_PARAM_ST> {
    const NAME: &'static str = "NetworkMsgParam";
    const TYPE_NAME: &'static str = "NETWORK_MSG_PARAM_ST";
    const VERSION: u16 = 3;
}

#[cfg(test)]
mod tests {
    use crate::param::NetworkMsgParam::NetworkMsgParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<NetworkMsgParam>(), 192)
    }
}
