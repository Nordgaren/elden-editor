/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::CHARMAKEMENU_LISTITEM_PARAM_ST::CHARMAKEMENU_LISTITEM_PARAM_ST;

/// Type: CHARMAKEMENU_LISTITEM_PARAM_ST

pub struct CharMakeMenuListItemParam {
	_data: CHARMAKEMENU_LISTITEM_PARAM_ST
}
impl Param for CharMakeMenuListItemParam {
	type Def = CHARMAKEMENU_LISTITEM_PARAM_ST;
	const NAME: &'static str = "CharMakeMenuListItemParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for CharMakeMenuListItemParam {
	type Target = CHARMAKEMENU_LISTITEM_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for CharMakeMenuListItemParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::CharMakeMenuListItemParam::CharMakeMenuListItemParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<CharMakeMenuListItemParam>(), 16)
	}
}
