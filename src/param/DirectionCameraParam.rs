/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::DIRECTION_CAMERA_PARAM_ST::DIRECTION_CAMERA_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: DIRECTION_CAMERA_PARAM_ST

pub struct DirectionCameraParam {
    _data: DIRECTION_CAMERA_PARAM_ST,
}
impl Param for DirectionCameraParam {
    type Def = DIRECTION_CAMERA_PARAM_ST;
    const NAME: &'static str = "DirectionCameraParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for DirectionCameraParam {
    type Target = DIRECTION_CAMERA_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for DirectionCameraParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::DirectionCameraParam::DirectionCameraParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<DirectionCameraParam>(), 16)
    }
}
