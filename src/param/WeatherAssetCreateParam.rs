/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::WEATHER_ASSET_CREATE_PARAM_ST::WEATHER_ASSET_CREATE_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: WEATHER_ASSET_CREATE_PARAM_ST

pub struct WeatherAssetCreateParam {
    _data: WEATHER_ASSET_CREATE_PARAM_ST,
}
impl Param for WeatherAssetCreateParam {
    type Def = WEATHER_ASSET_CREATE_PARAM_ST;
    const NAME: &'static str = "WeatherAssetCreateParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for WeatherAssetCreateParam {
    type Target = WEATHER_ASSET_CREATE_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for WeatherAssetCreateParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
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
