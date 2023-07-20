/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::ACTIONBUTTON_PARAM_ST::ACTIONBUTTON_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: ACTIONBUTTON_PARAM_ST

pub struct ActionButtonParam {
    _data: ACTIONBUTTON_PARAM_ST,
}
impl Param for ActionButtonParam {
    type Def = ACTIONBUTTON_PARAM_ST;
    const NAME: &'static str = "ActionButtonParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for ActionButtonParam {
    type Target = ACTIONBUTTON_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for ActionButtonParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::ActionButtonParam::ActionButtonParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<ActionButtonParam>(), 100)
    }
}
