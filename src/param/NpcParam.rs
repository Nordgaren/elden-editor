/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/NPC_PARAM_ST.rs");

/// Type: NPC_PARAM_ST

pub type NpcParam = ParamStruct<NPC_PARAM_ST>;
impl Param for ParamStruct<NPC_PARAM_ST> {
	const NAME: &'static str = "NpcParam";
	const TYPE_NAME: &'static str = "NPC_PARAM_ST";
	const VERSION: u16 = 9;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::NpcParam::NpcParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<NpcParam>(), 736)
	}
}
