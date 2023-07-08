/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/MENUPROPERTY_LAYOUT.rs");

/// Type: MENUPROPERTY_LAYOUT

pub type MenuPropertyLayoutParam = ParamStruct<MENUPROPERTY_LAYOUT>;
impl Param for ParamStruct<MENUPROPERTY_LAYOUT> {
    const NAME: &'static str = "MenuPropertyLayoutParam";
    const TYPE_NAME: &'static str = "MENUPROPERTY_LAYOUT";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::MenuPropertyLayoutParam::MenuPropertyLayoutParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MenuPropertyLayoutParam>(), 32)
    }
}
