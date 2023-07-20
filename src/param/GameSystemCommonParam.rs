/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::GAME_SYSTEM_COMMON_PARAM_ST::GAME_SYSTEM_COMMON_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: GAME_SYSTEM_COMMON_PARAM_ST

pub struct GameSystemCommonParam {
    _data: GAME_SYSTEM_COMMON_PARAM_ST,
}
impl Param for GameSystemCommonParam {
    type Def = GAME_SYSTEM_COMMON_PARAM_ST;
    const NAME: &'static str = "GameSystemCommonParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for GameSystemCommonParam {
    type Target = GAME_SYSTEM_COMMON_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for GameSystemCommonParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::GameSystemCommonParam::GameSystemCommonParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<GameSystemCommonParam>(), 880)
    }
}
