/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::BULLET_PARAM_ST::BULLET_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: BULLET_PARAM_ST

pub struct Bullet {
    _data: BULLET_PARAM_ST,
}
impl Param for Bullet {
    type Def = BULLET_PARAM_ST;
    const NAME: &'static str = "Bullet";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for Bullet {
    type Target = BULLET_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for Bullet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::Bullet::Bullet;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<Bullet>(), 272)
    }
}
