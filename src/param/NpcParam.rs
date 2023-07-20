/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::NPC_PARAM_ST::NPC_PARAM_ST;

/// Type: NPC_PARAM_ST

pub struct NpcParam {
	_data: NPC_PARAM_ST
}
impl Param for NpcParam {
	type Def = NPC_PARAM_ST;
	const NAME: &'static str = "NpcParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for NpcParam {
	type Target = NPC_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for NpcParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
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
