/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CHARMAKEMENUTOP_PARAM_ST.rs");

/// Type: CHARMAKEMENUTOP_PARAM_ST

pub type CharMakeMenuTopParam = ParamStruct<CHARMAKEMENUTOP_PARAM_ST>;
impl Param for ParamStruct<CHARMAKEMENUTOP_PARAM_ST> {
    const NAME: &'static str = "CharMakeMenuTopParam";
    const TYPE_NAME: &'static str = "CHARMAKEMENUTOP_PARAM_ST";
    const VERSION: u16 = 3;
}

#[cfg(test)]
mod tests {
    use crate::param::CharMakeMenuTopParam::CharMakeMenuTopParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<CharMakeMenuTopParam>(), 48)
    }
}
