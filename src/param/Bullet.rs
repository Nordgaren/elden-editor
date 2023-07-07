/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use crate::param::traits::*;

include!("defs/BULLET_PARAM_ST.rs");

/// Type: BULLET_PARAM_ST

pub type Bullet = ParamStruct<BULLET_PARAM_ST>;
impl Param for ParamStruct<BULLET_PARAM_ST> {
	const NAME: &'static str = "Bullet";
	const TYPE_NAME: &'static str = "BULLET_PARAM_ST";
	const VERSION: u16 = 4;
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::Bullet::Bullet;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<Bullet>(), 272)
	}
}
