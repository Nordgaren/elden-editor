/* This file was automatically generated from regulation data. */
#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use crate::param::traits::*;
use crate::param::defs::CACL_CORRECT_GRAPH_ST::CACL_CORRECT_GRAPH_ST;

/// Type: CACL_CORRECT_GRAPH_ST

pub struct CalcCorrectGraph {
	_data: CACL_CORRECT_GRAPH_ST
}
impl Param for CalcCorrectGraph {
	type Def = CACL_CORRECT_GRAPH_ST;
	const NAME: &'static str = "CalcCorrectGraph";
	fn data(&self) -> &Self::Def {
	&self._data
	}
	fn data_mut(&mut self) -> &mut Self::Def {
	&mut self._data
	}
}

impl Deref for CalcCorrectGraph {
	type Target = CACL_CORRECT_GRAPH_ST;
	fn deref(&self) -> &Self::Target {
		self.data()
	}
}
impl DerefMut for CalcCorrectGraph {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data_mut()
	}
}

#[cfg(test)]
mod tests {
	use std::mem::size_of;
	use crate::param::CalcCorrectGraph::CalcCorrectGraph;

	#[test]
	fn size_check() {
		assert_eq!(size_of::<CalcCorrectGraph>(), 80)
	}
}
