/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/FACE_PARAM_ST.rs");

/// Type: FACE_PARAM_ST

pub type FaceParam = ParamStruct<FACE_PARAM_ST>;
impl Param for ParamStruct<FACE_PARAM_ST> {
    const NAME: &'static str = "FaceParam";
    const TYPE_NAME: &'static str = "FACE_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::FaceParam::FaceParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<FaceParam>(), 240)
    }
}
