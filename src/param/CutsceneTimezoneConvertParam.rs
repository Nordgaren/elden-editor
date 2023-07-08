/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CUTSCENE_TIMEZONE_CONVERT_PARAM_ST.rs");

/// Type: CUTSCENE_TIMEZONE_CONVERT_PARAM_ST

pub type CutsceneTimezoneConvertParam = ParamStruct<CUTSCENE_TIMEZONE_CONVERT_PARAM_ST>;
impl Param for ParamStruct<CUTSCENE_TIMEZONE_CONVERT_PARAM_ST> {
    const NAME: &'static str = "CutsceneTimezoneConvertParam";
    const TYPE_NAME: &'static str = "CUTSCENE_TIMEZONE_CONVERT_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::CutsceneTimezoneConvertParam::CutsceneTimezoneConvertParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<CutsceneTimezoneConvertParam>(), 8)
    }
}
