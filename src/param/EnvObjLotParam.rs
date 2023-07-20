/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::ENV_OBJ_LOT_PARAM_ST::ENV_OBJ_LOT_PARAM_ST;

/// Type: ENV_OBJ_LOT_PARAM_ST

pub struct EnvObjLotParam {
	_data: ENV_OBJ_LOT_PARAM_ST
}
impl Param for EnvObjLotParam {
	type Def = ENV_OBJ_LOT_PARAM_ST;
	const NAME: &'static str = "EnvObjLotParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for EnvObjLotParam {
	type Target = ENV_OBJ_LOT_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for EnvObjLotParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::EnvObjLotParam::EnvObjLotParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<EnvObjLotParam>(), 64)
	}
}
