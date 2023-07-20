/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::NETWORK_MSG_PARAM_ST::NETWORK_MSG_PARAM_ST;

/// Type: NETWORK_MSG_PARAM_ST

pub struct NetworkMsgParam {
	_data: NETWORK_MSG_PARAM_ST
}
impl Param for NetworkMsgParam {
	type Def = NETWORK_MSG_PARAM_ST;
	const NAME: &'static str = "NetworkMsgParam";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for NetworkMsgParam {
	type Target = NETWORK_MSG_PARAM_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for NetworkMsgParam {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::NetworkMsgParam::NetworkMsgParam;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<NetworkMsgParam>(), 192)
	}
}
