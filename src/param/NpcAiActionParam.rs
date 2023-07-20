/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::NPC_AI_ACTION_PARAM_ST::NPC_AI_ACTION_PARAM_ST;

/// Type: NPC_AI_ACTION_PARAM_ST

pub struct NpcAiActionParam {
	_data: NPC_AI_ACTION_PARAM_ST
}
impl Param for NpcAiActionParam {
	type Def = NPC_AI_ACTION_PARAM_ST;
	const NAME: &'static str = "NpcAiActionParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for NpcAiActionParam {
	type Target = NPC_AI_ACTION_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for NpcAiActionParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
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
