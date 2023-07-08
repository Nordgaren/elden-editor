/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM.rs");

/// Type: LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM

pub type LegacyDistantViewPartsReplaceParam = ParamStruct<LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM>;
impl Param for ParamStruct<LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM> {
    const NAME: &'static str = "LegacyDistantViewPartsReplaceParam";
    const TYPE_NAME: &'static str = "LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM";
    const VERSION: u16 = 4;
}

#[cfg(test)]
mod tests {
    use crate::param::LegacyDistantViewPartsReplaceParam::LegacyDistantViewPartsReplaceParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<LegacyDistantViewPartsReplaceParam>(), 64)
    }
}
