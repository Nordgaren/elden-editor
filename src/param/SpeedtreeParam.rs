/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::SPEEDTREE_MODEL_PARAM_ST::SPEEDTREE_MODEL_PARAM_ST;

/// Type: SPEEDTREE_MODEL_PARAM_ST

pub struct SpeedtreeParam {
	_data: SPEEDTREE_MODEL_PARAM_ST
}
impl Param for SpeedtreeParam {
	type Def = SPEEDTREE_MODEL_PARAM_ST;
	const NAME: &'static str = "SpeedtreeParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for SpeedtreeParam {
	type Target = SPEEDTREE_MODEL_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for SpeedtreeParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::SpeedtreeParam::SpeedtreeParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<SpeedtreeParam>(), 40)
	}
}
