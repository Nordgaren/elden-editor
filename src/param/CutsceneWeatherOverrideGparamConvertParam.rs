/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST::CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST

pub struct CutsceneWeatherOverrideGparamConvertParam {
    _data: CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST,
}
impl Param for CutsceneWeatherOverrideGparamConvertParam {
    type Def = CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST;
    const NAME: &'static str = "CutsceneWeatherOverrideGparamConvertParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for CutsceneWeatherOverrideGparamConvertParam {
    type Target = CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for CutsceneWeatherOverrideGparamConvertParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::CutsceneWeatherOverrideGparamConvertParam::CutsceneWeatherOverrideGparamConvertParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<CutsceneWeatherOverrideGparamConvertParam>(), 4)
    }
}
