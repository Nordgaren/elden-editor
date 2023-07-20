/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::TALK_PARAM_ST::TALK_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: TALK_PARAM_ST

pub struct TalkParam {
    _data: TALK_PARAM_ST,
}
impl Param for TalkParam {
    type Def = TALK_PARAM_ST;
    const NAME: &'static str = "TalkParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for TalkParam {
    type Target = TALK_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for TalkParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::TalkParam::TalkParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<TalkParam>(), 96)
    }
}
