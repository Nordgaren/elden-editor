/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::CUTSCENE_TIMEZONE_CONVERT_PARAM_ST::CUTSCENE_TIMEZONE_CONVERT_PARAM_ST;

/// Type: CUTSCENE_TIMEZONE_CONVERT_PARAM_ST

pub struct CutsceneTimezoneConvertParam {
	_data: CUTSCENE_TIMEZONE_CONVERT_PARAM_ST
}
impl Param for CutsceneTimezoneConvertParam {
	type Def = CUTSCENE_TIMEZONE_CONVERT_PARAM_ST;
	const NAME: &'static str = "CutsceneTimezoneConvertParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for CutsceneTimezoneConvertParam {
	type Target = CUTSCENE_TIMEZONE_CONVERT_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for CutsceneTimezoneConvertParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::CutsceneTimezoneConvertParam::CutsceneTimezoneConvertParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<CutsceneTimezoneConvertParam>(), 8)
	}
}
