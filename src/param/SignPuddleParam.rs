/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::SIGN_PUDDLE_PARAM_ST::SIGN_PUDDLE_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: SIGN_PUDDLE_PARAM_ST

pub struct SignPuddleParam {
    _data: SIGN_PUDDLE_PARAM_ST,
}
impl Param for SignPuddleParam {
    type Def = SIGN_PUDDLE_PARAM_ST;
    const NAME: &'static str = "SignPuddleParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for SignPuddleParam {
    type Target = SIGN_PUDDLE_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for SignPuddleParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::SignPuddleParam::SignPuddleParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<SignPuddleParam>(), 32)
    }
}
