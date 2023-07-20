/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::CUTSCENE_GPARAM_WEATHER_PARAM_ST::CUTSCENE_GPARAM_WEATHER_PARAM_ST;

/// Type: CUTSCENE_GPARAM_WEATHER_PARAM_ST

pub struct CutsceneGparamWeatherParam {
	_data: CUTSCENE_GPARAM_WEATHER_PARAM_ST
}
impl Param for CutsceneGparamWeatherParam {
	type Def = CUTSCENE_GPARAM_WEATHER_PARAM_ST;
	const NAME: &'static str = "CutsceneGparamWeatherParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for CutsceneGparamWeatherParam {
	type Target = CUTSCENE_GPARAM_WEATHER_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for CutsceneGparamWeatherParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::CutsceneGparamWeatherParam::CutsceneGparamWeatherParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<CutsceneGparamWeatherParam>(), 96)
	}
}
