/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::TUTORIAL_PARAM_ST::TUTORIAL_PARAM_ST;

/// Type: TUTORIAL_PARAM_ST

pub struct TutorialParam {
	_data: TUTORIAL_PARAM_ST
}
impl Param for TutorialParam {
	type Def = TUTORIAL_PARAM_ST;
	const NAME: &'static str = "TutorialParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for TutorialParam {
	type Target = TUTORIAL_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for TutorialParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::TutorialParam::TutorialParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<TutorialParam>(), 32)
	}
}
