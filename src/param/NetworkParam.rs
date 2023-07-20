/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::NETWORK_PARAM_ST::NETWORK_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: NETWORK_PARAM_ST

pub struct NetworkParam {
    _data: NETWORK_PARAM_ST,
}
impl Param for NetworkParam {
    type Def = NETWORK_PARAM_ST;
    const NAME: &'static str = "NetworkParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for NetworkParam {
    type Target = NETWORK_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for NetworkParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::NetworkParam::NetworkParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<NetworkParam>(), 632)
    }
}
