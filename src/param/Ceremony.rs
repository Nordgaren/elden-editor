/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CEREMONY_PARAM_ST.rs");

/// Type: CEREMONY_PARAM_ST

pub type Ceremony = ParamStruct<CEREMONY_PARAM_ST>;
impl Param for ParamStruct<CEREMONY_PARAM_ST> {
    const NAME: &'static str = "Ceremony";
    const TYPE_NAME: &'static str = "CEREMONY_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::Ceremony::Ceremony;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<Ceremony>(), 36)
    }
}
