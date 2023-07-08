/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/BEHAVIOR_PARAM_ST.rs");

/// Type: BEHAVIOR_PARAM_ST

pub type BehaviorParam_PC = ParamStruct<BEHAVIOR_PARAM_ST>;
impl Param for ParamStruct<BEHAVIOR_PARAM_ST> {
    const NAME: &'static str = "BehaviorParam_PC";
    const TYPE_NAME: &'static str = "BEHAVIOR_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::BehaviorParam_PC::BehaviorParam_PC;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<BehaviorParam_PC>(), 32)
    }
}
