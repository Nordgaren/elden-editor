/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/CUTSCENE_GPARAM_WEATHER_PARAM_ST.rs");

/// Type: CUTSCENE_GPARAM_WEATHER_PARAM_ST

pub type CutsceneGparamWeatherParam = ParamStruct<CUTSCENE_GPARAM_WEATHER_PARAM_ST>;
impl Param for ParamStruct<CUTSCENE_GPARAM_WEATHER_PARAM_ST> {
    const NAME: &'static str = "CutsceneGparamWeatherParam";
    const TYPE_NAME: &'static str = "CUTSCENE_GPARAM_WEATHER_PARAM_ST";
    const VERSION: u16 = 6;
}

#[cfg(test)]
mod tests {
    use crate::param::CutsceneGparamWeatherParam::CutsceneGparamWeatherParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<CutsceneGparamWeatherParam>(), 96)
    }
}
