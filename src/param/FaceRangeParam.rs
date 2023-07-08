/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/FACE_RANGE_PARAM_ST.rs");

/// Type: FACE_RANGE_PARAM_ST

pub type FaceRangeParam = ParamStruct<FACE_RANGE_PARAM_ST>;
impl Param for ParamStruct<FACE_RANGE_PARAM_ST> {
    const NAME: &'static str = "FaceRangeParam";
    const TYPE_NAME: &'static str = "FACE_RANGE_PARAM_ST";
    const VERSION: u16 = 3;
}

#[cfg(test)]
mod tests {
    use crate::param::FaceRangeParam::FaceRangeParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<FaceRangeParam>(), 824)
    }
}
