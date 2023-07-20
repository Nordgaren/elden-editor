/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST {

	/// NAME: Grid can be constructed Height Min [m] - グリッド構築可能高さMin[m]
	/// DESC: Minimum height that can be built in the grid [m]. (LOD 2 units) - グリッド構築可能高さ最小値[m]。(LOD2単位)
	pub GridEnableCreateHeightMin:f32,

	/// NAME: Grid can be constructed Height Max [m] - グリッド構築可能高さMax[m]
	/// DESC: Maximum height that can be constructed in the grid [m]. (LOD 2 units) - グリッド構築可能高さ最大値[m]。(LOD2単位)
	pub GridEnableCreateHeightMax:f32,

	/// NAME: Reserve - リザーブ
	/// DESC: Reserve - リザーブ
	pub Reserve:[u8;24],
}

impl Paramdef for MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST {
const NAME: &'static str = "MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST";
const VERSION: u16 = 1;
}
