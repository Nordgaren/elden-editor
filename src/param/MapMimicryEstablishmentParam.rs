/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MAP_MIMICRY_ESTABLISHMENT_PARAM_ST.rs");

/// Type: MAP_MIMICRY_ESTABLISHMENT_PARAM_ST

pub type MapMimicryEstablishmentParam = ParamStruct<MAP_MIMICRY_ESTABLISHMENT_PARAM_ST>;
impl Param for ParamStruct<MAP_MIMICRY_ESTABLISHMENT_PARAM_ST> {
    const NAME: &'static str = "MapMimicryEstablishmentParam";
    const TYPE_NAME: &'static str = "MAP_MIMICRY_ESTABLISHMENT_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::MapMimicryEstablishmentParam::MapMimicryEstablishmentParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MapMimicryEstablishmentParam>(), 64)
    }
}
