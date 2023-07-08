/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/PLAY_REGION_PARAM_ST.rs");

/// Type: PLAY_REGION_PARAM_ST

pub type PlayRegionParam = ParamStruct<PLAY_REGION_PARAM_ST>;
impl Param for ParamStruct<PLAY_REGION_PARAM_ST> {
    const NAME: &'static str = "PlayRegionParam";
    const TYPE_NAME: &'static str = "PLAY_REGION_PARAM_ST";
    const VERSION: u16 = 9;
}

#[cfg(test)]
mod tests {
    use crate::param::PlayRegionParam::PlayRegionParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<PlayRegionParam>(), 272)
    }
}
