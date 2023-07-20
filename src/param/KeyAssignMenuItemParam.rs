/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::CS_KEY_ASSIGN_MENUITEM_PARAM::CS_KEY_ASSIGN_MENUITEM_PARAM;

/// Type: CS_KEY_ASSIGN_MENUITEM_PARAM

pub struct KeyAssignMenuItemParam {
	_data: CS_KEY_ASSIGN_MENUITEM_PARAM
}
impl Param for KeyAssignMenuItemParam {
	type Def = CS_KEY_ASSIGN_MENUITEM_PARAM;
	const NAME: &'static str = "KeyAssignMenuItemParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for KeyAssignMenuItemParam {
	type Target = CS_KEY_ASSIGN_MENUITEM_PARAM;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for KeyAssignMenuItemParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::KeyAssignMenuItemParam::KeyAssignMenuItemParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<KeyAssignMenuItemParam>(), 24)
	}
}
