/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MENUPROPERTY_SPEC.rs");

/// Type: MENUPROPERTY_SPEC

pub type MenuPropertySpecParam = ParamStruct<MENUPROPERTY_SPEC>;
impl Param for ParamStruct<MENUPROPERTY_SPEC> {
    const NAME: &'static str = "MenuPropertySpecParam";
    const TYPE_NAME: &'static str = "MENUPROPERTY_SPEC";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::MenuPropertySpecParam::MenuPropertySpecParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MenuPropertySpecParam>(), 32)
    }
}
