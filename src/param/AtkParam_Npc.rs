/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/ATK_PARAM_ST.rs");

/// Type: ATK_PARAM_ST

pub type AtkParam_Npc = ParamStruct<ATK_PARAM_ST>;
impl Param for ParamStruct<ATK_PARAM_ST> {
    const NAME: &'static str = "AtkParam_Npc";
    const TYPE_NAME: &'static str = "ATK_PARAM_ST";
    const VERSION: u16 = 4;
}

#[cfg(test)]
mod tests {
    use crate::param::AtkParam_Npc::AtkParam_Npc;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<AtkParam_Npc>(), 456)
    }
}
