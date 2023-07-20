/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST::LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST;

/// Type: LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST

pub struct LoadBalancerDrawDistScaleParam_xb1 {
	_data: LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST
}
impl Param for LoadBalancerDrawDistScaleParam_xb1 {
	type Def = LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST;
	const NAME: &'static str = "LoadBalancerDrawDistScaleParam_xb1";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for LoadBalancerDrawDistScaleParam_xb1 {
	type Target = LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for LoadBalancerDrawDistScaleParam_xb1 {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::LoadBalancerDrawDistScaleParam_xb1::LoadBalancerDrawDistScaleParam_xb1;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<LoadBalancerDrawDistScaleParam_xb1>(), 128)
	}
}
