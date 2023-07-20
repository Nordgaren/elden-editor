/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::MOVE_PARAM_ST::MOVE_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: MOVE_PARAM_ST

pub struct MoveParam {
    _data: MOVE_PARAM_ST,
}
impl Param for MoveParam {
    type Def = MOVE_PARAM_ST;
    const NAME: &'static str = "MoveParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for MoveParam {
    type Target = MOVE_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for MoveParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::MoveParam::MoveParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MoveParam>(), 144)
    }
}
