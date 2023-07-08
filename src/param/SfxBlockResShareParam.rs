/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SFX_BLOCK_RES_SHARE_PARAM.rs");

/// Type: SFX_BLOCK_RES_SHARE_PARAM

pub type SfxBlockResShareParam = ParamStruct<SFX_BLOCK_RES_SHARE_PARAM>;
impl Param for ParamStruct<SFX_BLOCK_RES_SHARE_PARAM> {
    const NAME: &'static str = "SfxBlockResShareParam";
    const TYPE_NAME: &'static str = "SFX_BLOCK_RES_SHARE_PARAM";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::SfxBlockResShareParam::SfxBlockResShareParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SfxBlockResShareParam>(), 4)
    }
}
