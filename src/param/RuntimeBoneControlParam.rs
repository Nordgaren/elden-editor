/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/RUNTIME_BONE_CONTROL_PARAM_ST.rs");

/// Type: RUNTIME_BONE_CONTROL_PARAM_ST

pub type RuntimeBoneControlParam = ParamStruct<RUNTIME_BONE_CONTROL_PARAM_ST>;
impl Param for ParamStruct<RUNTIME_BONE_CONTROL_PARAM_ST> {
    const NAME: &'static str = "RuntimeBoneControlParam";
    const TYPE_NAME: &'static str = "RUNTIME_BONE_CONTROL_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::RuntimeBoneControlParam::RuntimeBoneControlParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<RuntimeBoneControlParam>(), 112)
    }
}
