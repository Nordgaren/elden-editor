/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/WEATHER_ASSET_CREATE_PARAM_ST.rs");

/// Type: WEATHER_ASSET_CREATE_PARAM_ST

pub type WeatherAssetCreateParam = ParamStruct<WEATHER_ASSET_CREATE_PARAM_ST>;
impl Param for ParamStruct<WEATHER_ASSET_CREATE_PARAM_ST> {
    const NAME: &'static str = "WeatherAssetCreateParam";
    const TYPE_NAME: &'static str = "WEATHER_ASSET_CREATE_PARAM_ST";
    const VERSION: u16 = 1;
}

#[cfg(test)]
mod tests {
    use crate::param::WeatherAssetCreateParam::WeatherAssetCreateParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<WeatherAssetCreateParam>(), 64)
    }
}
