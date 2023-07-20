/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::CEREMONY_PARAM_ST::CEREMONY_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: CEREMONY_PARAM_ST

pub struct Ceremony {
    _data: CEREMONY_PARAM_ST,
}
impl Param for Ceremony {
    type Def = CEREMONY_PARAM_ST;
    const NAME: &'static str = "Ceremony";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for Ceremony {
    type Target = CEREMONY_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for Ceremony {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::Ceremony::Ceremony;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<Ceremony>(), 36)
    }
}
