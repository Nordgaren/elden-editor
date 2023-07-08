/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/TUTORIAL_PARAM_ST.rs");

/// Type: TUTORIAL_PARAM_ST

pub type TutorialParam = ParamStruct<TUTORIAL_PARAM_ST>;
impl Param for ParamStruct<TUTORIAL_PARAM_ST> {
    const NAME: &'static str = "TutorialParam";
    const TYPE_NAME: &'static str = "TUTORIAL_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::TutorialParam::TutorialParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<TutorialParam>(), 32)
    }
}
