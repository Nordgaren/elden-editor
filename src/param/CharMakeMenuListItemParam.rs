/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CHARMAKEMENU_LISTITEM_PARAM_ST.rs");

/// Type: CHARMAKEMENU_LISTITEM_PARAM_ST

pub type CharMakeMenuListItemParam = ParamStruct<CHARMAKEMENU_LISTITEM_PARAM_ST>;
impl Param for ParamStruct<CHARMAKEMENU_LISTITEM_PARAM_ST> {
    const NAME: &'static str = "CharMakeMenuListItemParam";
    const TYPE_NAME: &'static str = "CHARMAKEMENU_LISTITEM_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::CharMakeMenuListItemParam::CharMakeMenuListItemParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<CharMakeMenuListItemParam>(), 16)
    }
}
