/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST.rs");

/// Type: NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST

pub type NpcAiBehaviorProbability = ParamStruct<NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST>;
impl Param for ParamStruct<NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST> {
    const NAME: &'static str = "NpcAiBehaviorProbability";
    const TYPE_NAME: &'static str = "NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST";
    const VERSION: u16 = 2;
}

#[cfg(test)]
mod tests {
    use crate::param::NpcAiBehaviorProbability::NpcAiBehaviorProbability;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<NpcAiBehaviorProbability>(), 400)
    }
}
