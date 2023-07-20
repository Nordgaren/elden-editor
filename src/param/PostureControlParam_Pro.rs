/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::POSTURE_CONTROL_PARAM_PRO_ST::POSTURE_CONTROL_PARAM_PRO_ST;

/// Type: POSTURE_CONTROL_PARAM_PRO_ST

pub struct PostureControlParam_Pro {
	_data: POSTURE_CONTROL_PARAM_PRO_ST
}
impl Param for PostureControlParam_Pro {
	type Def = POSTURE_CONTROL_PARAM_PRO_ST;
	const NAME: &'static str = "PostureControlParam_Pro";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for PostureControlParam_Pro {
	type Target = POSTURE_CONTROL_PARAM_PRO_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for PostureControlParam_Pro {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::PostureControlParam_Pro::PostureControlParam_Pro;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<PostureControlParam_Pro>(), 80)
	}
}
