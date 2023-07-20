/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::ACTIONBUTTON_PARAM_ST::ACTIONBUTTON_PARAM_ST;

/// Type: ACTIONBUTTON_PARAM_ST

pub struct ActionButtonParam {
	_data: ACTIONBUTTON_PARAM_ST
}
impl Param for ActionButtonParam {
	type Def = ACTIONBUTTON_PARAM_ST;
	const NAME: &'static str = "ActionButtonParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}


impl Deref for ActionButtonParam {
	type Target = ACTIONBUTTON_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for ActionButtonParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::ActionButtonParam::ActionButtonParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<ActionButtonParam>(), 100)
	}
}
