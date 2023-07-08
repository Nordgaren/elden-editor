/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/NPC_THINK_PARAM_ST.rs");

/// Type: NPC_THINK_PARAM_ST

pub type NpcThinkParam = ParamStruct<NPC_THINK_PARAM_ST>;
impl Param for ParamStruct<NPC_THINK_PARAM_ST> {
    const NAME: &'static str = "NpcThinkParam";
    const TYPE_NAME: &'static str = "NPC_THINK_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::NpcThinkParam::NpcThinkParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<NpcThinkParam>(), 228)
    }
}
