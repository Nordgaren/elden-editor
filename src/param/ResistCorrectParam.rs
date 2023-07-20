/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::RESIST_CORRECT_PARAM_ST::RESIST_CORRECT_PARAM_ST;

/// Type: RESIST_CORRECT_PARAM_ST

pub struct ResistCorrectParam {
	_data: RESIST_CORRECT_PARAM_ST
}
impl Param for ResistCorrectParam {
	type Def = RESIST_CORRECT_PARAM_ST;
	const NAME: &'static str = "ResistCorrectParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for ResistCorrectParam {
	type Target = RESIST_CORRECT_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for ResistCorrectParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ResistCorrectParam::ResistCorrectParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ResistCorrectParam>(), 40)
	}
}
