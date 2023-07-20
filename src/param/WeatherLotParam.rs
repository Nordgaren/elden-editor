/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::WEATHER_LOT_PARAM_ST::WEATHER_LOT_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: WEATHER_LOT_PARAM_ST

pub struct WeatherLotParam {
    _data: WEATHER_LOT_PARAM_ST,
}
impl Param for WeatherLotParam {
    type Def = WEATHER_LOT_PARAM_ST;
    const NAME: &'static str = "WeatherLotParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for WeatherLotParam {
    type Target = WEATHER_LOT_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for WeatherLotParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
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
