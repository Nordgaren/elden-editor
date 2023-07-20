/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::WWISE_VALUE_TO_STR_CONVERT_PARAM_ST::WWISE_VALUE_TO_STR_CONVERT_PARAM_ST;

/// Type: WWISE_VALUE_TO_STR_CONVERT_PARAM_ST

pub struct WwiseValueToStrParam_Switch_OffensiveMaterial {
	_data: WWISE_VALUE_TO_STR_CONVERT_PARAM_ST
}
impl Param for WwiseValueToStrParam_Switch_OffensiveMaterial {
	type Def = WWISE_VALUE_TO_STR_CONVERT_PARAM_ST;
	const NAME: &'static str = "WwiseValueToStrParam_Switch_OffensiveMaterial";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for WwiseValueToStrParam_Switch_OffensiveMaterial {
	type Target = WWISE_VALUE_TO_STR_CONVERT_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for WwiseValueToStrParam_Switch_OffensiveMaterial {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::WwiseValueToStrParam_Switch_OffensiveMaterial::WwiseValueToStrParam_Switch_OffensiveMaterial;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<WwiseValueToStrParam_Switch_OffensiveMaterial>(), 36)
	}
}
