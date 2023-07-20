/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::BUDDY_PARAM_ST::BUDDY_PARAM_ST;

/// Type: BUDDY_PARAM_ST

pub struct BuddyParam {
	_data: BUDDY_PARAM_ST
}
impl Param for BuddyParam {
	type Def = BUDDY_PARAM_ST;
	const NAME: &'static str = "BuddyParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for BuddyParam {
	type Target = BUDDY_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for BuddyParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::BuddyParam::BuddyParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<BuddyParam>(), 160)
	}
}
