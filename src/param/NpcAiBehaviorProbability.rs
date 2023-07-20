/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST::NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST;

/// Type: NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST

pub struct NpcAiBehaviorProbability {
	_data: NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST
}
impl Param for NpcAiBehaviorProbability {
	type Def = NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST;
	const NAME: &'static str = "NpcAiBehaviorProbability";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for NpcAiBehaviorProbability {
	type Target = NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for NpcAiBehaviorProbability {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::NpcAiBehaviorProbability::NpcAiBehaviorProbability;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<NpcAiBehaviorProbability>(), 400)
	}
}
