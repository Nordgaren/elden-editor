/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::GPARAM_REF_SETTINGS_PARAM_ST::GPARAM_REF_SETTINGS_PARAM_ST;

/// Type: GPARAM_REF_SETTINGS_PARAM_ST

pub struct GparamRefSettings {
	_data: GPARAM_REF_SETTINGS_PARAM_ST
}
impl Param for GparamRefSettings {
	type Def = GPARAM_REF_SETTINGS_PARAM_ST;
	const NAME: &'static str = "GparamRefSettings";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for GparamRefSettings {
	type Target = GPARAM_REF_SETTINGS_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for GparamRefSettings {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::GparamRefSettings::GparamRefSettings;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<GparamRefSettings>(), 32)
	}
}
