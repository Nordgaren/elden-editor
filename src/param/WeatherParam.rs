/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/WEATHER_PARAM_ST.rs");

/// Type: WEATHER_PARAM_ST

pub type WeatherParam = ParamStruct<WEATHER_PARAM_ST>;
impl Param for ParamStruct<WEATHER_PARAM_ST> {
    const NAME: &'static str = "WeatherParam";
    const TYPE_NAME: &'static str = "WEATHER_PARAM_ST";
    const VERSION: u16 = 3;
}

#[cfg(test)]
mod tests {
    use crate::param::WeatherParam::WeatherParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<WeatherParam>(), 68)
    }
}
