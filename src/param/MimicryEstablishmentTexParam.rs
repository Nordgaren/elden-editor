/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MIMICRY_ESTABLISHMENT_TEX_PARAM_ST.rs");

/// Type: MIMICRY_ESTABLISHMENT_TEX_PARAM_ST

pub type MimicryEstablishmentTexParam = ParamStruct<MIMICRY_ESTABLISHMENT_TEX_PARAM_ST>;
impl Param for ParamStruct<MIMICRY_ESTABLISHMENT_TEX_PARAM_ST> {
    const NAME: &'static str = "MimicryEstablishmentTexParam";
    const TYPE_NAME: &'static str = "MIMICRY_ESTABLISHMENT_TEX_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::MimicryEstablishmentTexParam::MimicryEstablishmentTexParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MimicryEstablishmentTexParam>(), 16)
    }
}
