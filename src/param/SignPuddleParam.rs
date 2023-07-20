/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::SIGN_PUDDLE_PARAM_ST::SIGN_PUDDLE_PARAM_ST;

/// Type: SIGN_PUDDLE_PARAM_ST

pub struct SignPuddleParam {
	_data: SIGN_PUDDLE_PARAM_ST
}
impl Param for SignPuddleParam {
	type Def = SIGN_PUDDLE_PARAM_ST;
	const NAME: &'static str = "SignPuddleParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for SignPuddleParam {
	type Target = SIGN_PUDDLE_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for SignPuddleParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::SignPuddleParam::SignPuddleParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<SignPuddleParam>(), 32)
	}
}
