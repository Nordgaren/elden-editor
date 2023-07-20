/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::MAGIC_PARAM_ST::MAGIC_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: MAGIC_PARAM_ST

pub struct Magic {
    _data: MAGIC_PARAM_ST,
}
impl Param for Magic {
    type Def = MAGIC_PARAM_ST;
    const NAME: &'static str = "Magic";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for Magic {
    type Target = MAGIC_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for Magic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::Magic::Magic;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<Magic>(), 168)
    }
}
