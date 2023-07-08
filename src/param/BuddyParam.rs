/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/BUDDY_PARAM_ST.rs");

/// Type: BUDDY_PARAM_ST

pub type BuddyParam = ParamStruct<BUDDY_PARAM_ST>;
impl Param for ParamStruct<BUDDY_PARAM_ST> {
    const NAME: &'static str = "BuddyParam";
    const TYPE_NAME: &'static str = "BUDDY_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::BuddyParam::BuddyParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<BuddyParam>(), 160)
    }
}
