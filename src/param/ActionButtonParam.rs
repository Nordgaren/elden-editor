/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/ACTIONBUTTON_PARAM_ST.rs");

/// Type: ACTIONBUTTON_PARAM_ST

pub type ActionButtonParam = ParamStruct<ACTIONBUTTON_PARAM_ST>;
impl Param for ParamStruct<ACTIONBUTTON_PARAM_ST> {
    const NAME: &'static str = "ActionButtonParam";
    const TYPE_NAME: &'static str = "ACTIONBUTTON_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::ActionButtonParam::ActionButtonParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<ActionButtonParam>(), 100)
    }
}
