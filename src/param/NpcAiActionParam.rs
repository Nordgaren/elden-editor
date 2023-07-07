/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/NPC_AI_ACTION_PARAM_ST.rs");

/// Type: NPC_AI_ACTION_PARAM_ST

pub type NpcAiActionParam = ParamStruct<NPC_AI_ACTION_PARAM_ST>;
impl Param for ParamStruct<NPC_AI_ACTION_PARAM_ST> {
	const NAME: &'static str = "NpcAiActionParam";
	const TYPE_NAME: &'static str = "NPC_AI_ACTION_PARAM_ST";
	const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::NpcAiActionParam::NpcAiActionParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<NpcAiActionParam>(), 16)
	}
}
