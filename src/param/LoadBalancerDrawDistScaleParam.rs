/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::defs::LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST::LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST;
use crate::param::traits::*;
use std::ops::{Deref, DerefMut};

/// Type: LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST

pub struct LoadBalancerDrawDistScaleParam {
    _data: LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST,
}
impl Param for LoadBalancerDrawDistScaleParam {
    type Def = LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST;
    const NAME: &'static str = "LoadBalancerDrawDistScaleParam";
    fn data(&self) -> &Self::Def {
        &self._data
    }
    fn data_mut(&mut self) -> &mut Self::Def {
        &mut self._data
    }
}

impl Deref for LoadBalancerDrawDistScaleParam {
    type Target = LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST;
    fn deref(&self) -> &Self::Target {
        self.data()
    }
}
impl DerefMut for LoadBalancerDrawDistScaleParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::param::LoadBalancerDrawDistScaleParam::LoadBalancerDrawDistScaleParam;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<LoadBalancerDrawDistScaleParam>(), 128)
    }
}
