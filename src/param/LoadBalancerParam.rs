/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::LOAD_BALANCER_PARAM_ST::LOAD_BALANCER_PARAM_ST;

/// Type: LOAD_BALANCER_PARAM_ST

pub struct LoadBalancerParam {
	_data: LOAD_BALANCER_PARAM_ST
}
impl Param for LoadBalancerParam {
	type Def = LOAD_BALANCER_PARAM_ST;
	const NAME: &'static str = "LoadBalancerParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for LoadBalancerParam {
	type Target = LOAD_BALANCER_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for LoadBalancerParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::LoadBalancerParam::LoadBalancerParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<LoadBalancerParam>(), 80)
	}
}
