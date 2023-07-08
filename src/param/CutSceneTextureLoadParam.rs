/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CUTSCENE_TEXTURE_LOAD_PARAM_ST.rs");

/// Type: CUTSCENE_TEXTURE_LOAD_PARAM_ST

pub type CutSceneTextureLoadParam = ParamStruct<CUTSCENE_TEXTURE_LOAD_PARAM_ST>;
impl Param for ParamStruct<CUTSCENE_TEXTURE_LOAD_PARAM_ST> {
    const NAME: &'static str = "CutSceneTextureLoadParam";
    const TYPE_NAME: &'static str = "CUTSCENE_TEXTURE_LOAD_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::CutSceneTextureLoadParam::CutSceneTextureLoadParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<CutSceneTextureLoadParam>(), 260)
    }
}
