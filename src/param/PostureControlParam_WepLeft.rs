/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::param::defs::POSTURE_CONTROL_PARAM_WEP_LEFT_ST::POSTURE_CONTROL_PARAM_WEP_LEFT_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: POSTURE_CONTROL_PARAM_WEP_LEFT_ST

pub struct PostureControlParam_WepLeft {
    _data: POSTURE_CONTROL_PARAM_WEP_LEFT_ST,
}
impl Param for PostureControlParam_WepLeft {
    type Def = POSTURE_CONTROL_PARAM_WEP_LEFT_ST;
    const NAME: &'static str = "PostureControlParam_WepLeft";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for PostureControlParam_WepLeft {
    type Target = POSTURE_CONTROL_PARAM_WEP_LEFT_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for PostureControlParam_WepLeft {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::PostureControlParam_WepLeft::PostureControlParam_WepLeft;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<PostureControlParam_WepLeft>(), 32)
    }
}
