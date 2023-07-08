/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/KEY_ASSIGN_PARAM_ST.rs");

/// Type: KEY_ASSIGN_PARAM_ST

pub type KeyAssignParam_TypeB = ParamStruct<KEY_ASSIGN_PARAM_ST>;
impl Param for ParamStruct<KEY_ASSIGN_PARAM_ST> {
    const NAME: &'static str = "KeyAssignParam_TypeB";
    const TYPE_NAME: &'static str = "KEY_ASSIGN_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::KeyAssignParam_TypeB::KeyAssignParam_TypeB;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<KeyAssignParam_TypeB>(), 32)
    }
}
