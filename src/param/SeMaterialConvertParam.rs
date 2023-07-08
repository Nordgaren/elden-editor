/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/SE_MATERIAL_CONVERT_PARAM_ST.rs");

/// Type: SE_MATERIAL_CONVERT_PARAM_ST

pub type SeMaterialConvertParam = ParamStruct<SE_MATERIAL_CONVERT_PARAM_ST>;
impl Param for ParamStruct<SE_MATERIAL_CONVERT_PARAM_ST> {
    const NAME: &'static str = "SeMaterialConvertParam";
    const TYPE_NAME: &'static str = "SE_MATERIAL_CONVERT_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::SeMaterialConvertParam::SeMaterialConvertParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SeMaterialConvertParam>(), 4)
    }
}
