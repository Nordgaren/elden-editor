/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/WEATHER_LOT_PARAM_ST.rs");

/// Type: WEATHER_LOT_PARAM_ST

pub type WeatherLotParam = ParamStruct<WEATHER_LOT_PARAM_ST>;
impl Param for ParamStruct<WEATHER_LOT_PARAM_ST> {
    const NAME: &'static str = "WeatherLotParam";
    const TYPE_NAME: &'static str = "WEATHER_LOT_PARAM_ST";
    const VERSION: u16 = 5;
}

#[cfg(test)]
mod tests {
    use crate::param::WeatherLotParam::WeatherLotParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<WeatherLotParam>(), 82)
    }
}
