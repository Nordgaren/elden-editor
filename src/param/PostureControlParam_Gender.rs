/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::param::defs::POSTURE_CONTROL_PARAM_GENDER_ST::POSTURE_CONTROL_PARAM_GENDER_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: POSTURE_CONTROL_PARAM_GENDER_ST

pub struct PostureControlParam_Gender {
    _data: POSTURE_CONTROL_PARAM_GENDER_ST,
}
impl Param for PostureControlParam_Gender {
    type Def = POSTURE_CONTROL_PARAM_GENDER_ST;
    const NAME: &'static str = "PostureControlParam_Gender";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for PostureControlParam_Gender {
    type Target = POSTURE_CONTROL_PARAM_GENDER_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for PostureControlParam_Gender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::PostureControlParam_Gender::PostureControlParam_Gender;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<PostureControlParam_Gender>(), 64)
    }
}
