/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::FACE_RANGE_PARAM_ST::FACE_RANGE_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: FACE_RANGE_PARAM_ST

pub struct FaceRangeParam {
    _data: FACE_RANGE_PARAM_ST,
}
impl Param for FaceRangeParam {
    type Def = FACE_RANGE_PARAM_ST;
    const NAME: &'static str = "FaceRangeParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for FaceRangeParam {
    type Target = FACE_RANGE_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for FaceRangeParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::FaceRangeParam::FaceRangeParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<FaceRangeParam>(), 824)
    }
}
