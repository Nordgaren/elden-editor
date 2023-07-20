/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM::LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM;

/// Type: LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM

pub struct LegacyDistantViewPartsReplaceParam {
	_data: LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM
}
impl Param for LegacyDistantViewPartsReplaceParam {
	type Def = LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM;
	const NAME: &'static str = "LegacyDistantViewPartsReplaceParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for LegacyDistantViewPartsReplaceParam {
	type Target = LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for LegacyDistantViewPartsReplaceParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::LegacyDistantViewPartsReplaceParam::LegacyDistantViewPartsReplaceParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<LegacyDistantViewPartsReplaceParam>(), 64)
	}
}
