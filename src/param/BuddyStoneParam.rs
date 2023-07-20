/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::BUDDY_STONE_PARAM_ST::BUDDY_STONE_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: BUDDY_STONE_PARAM_ST

pub struct BuddyStoneParam {
    _data: BUDDY_STONE_PARAM_ST,
}
impl Param for BuddyStoneParam {
    type Def = BUDDY_STONE_PARAM_ST;
    const NAME: &'static str = "BuddyStoneParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for BuddyStoneParam {
    type Target = BUDDY_STONE_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for BuddyStoneParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::BuddyStoneParam::BuddyStoneParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<BuddyStoneParam>(), 64)
    }
}
