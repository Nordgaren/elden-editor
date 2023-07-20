/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct HIT_EFFECT_SFX_PARAM_ST {

	/// NAME: Slash: Standard - 斬撃：標準
	/// DESC: Slash: Standard - 斬撃：標準
	pub Slash_Normal:i32,

	/// NAME: Slash: Small - 斬撃：小
	/// DESC: Slash: Small - 斬撃：小
	pub Slash_S:i32,

	/// NAME: Slash: Large - 斬撃：大
	/// DESC: Slash: Large - 斬撃：大
	pub Slash_L:i32,

	/// NAME: Slash: Designation 1 - 斬撃：指定1
	/// DESC: Slash: Designation 1 - 斬撃：指定1
	pub Slash_Specific1:i32,

	/// NAME: Slash: Designation 2 - 斬撃：指定2
	/// DESC: Slash: Designation 2 - 斬撃：指定2
	pub Slash_Specific2:i32,

	/// NAME: Batter: Standard - 打撃：標準
	/// DESC: Batter: Standard - 打撃：標準
	pub Blow_Normal:i32,

	/// NAME: Batter: Small - 打撃：小
	/// DESC: Batter: Small - 打撃：小
	pub Blow_S:i32,

	/// NAME: Batter: Large - 打撃：大
	/// DESC: Batter: Large - 打撃：大
	pub Blow_L:i32,

	/// NAME: Batter: Designation 1 - 打撃：指定1
	/// DESC: Batter: Designation 1 - 打撃：指定1
	pub Blow_Specific1:i32,

	/// NAME: Batter: Designation 2 - 打撃：指定2
	/// DESC: Batter: Designation 2 - 打撃：指定2
	pub Blow_Specific2:i32,

	/// NAME: Piercing: Standard - 刺突：標準
	/// DESC: Piercing: Standard - 刺突：標準
	pub Thrust_Normal:i32,

	/// NAME: Piercing: Small - 刺突：小
	/// DESC: Piercing: Small - 刺突：小
	pub Thrust_S:i32,

	/// NAME: Piercing: Large - 刺突：大
	/// DESC: Piercing: Large - 刺突：大
	pub Thrust_L:i32,

	/// NAME: Piercing: Designation 1 - 刺突：指定1
	/// DESC: Piercing: Designation 1 - 刺突：指定1
	pub Thrust_Specific1:i32,

	/// NAME: Piercing: Designation 2 - 刺突：指定2
	/// DESC: Piercing: Designation 2 - 刺突：指定2
	pub Thrust_Specific2:i32,

	/// NAME: Non-attribute: standard - 無属性：標準
	/// DESC: Non-attribute: standard - 無属性：標準
	pub Neutral_Normal:i32,

	/// NAME: Non-attribute: small - 無属性：小
	/// DESC: Non-attribute: small - 無属性：小
	pub Neutral_S:i32,

	/// NAME: Non-attribute: Large - 無属性：大
	/// DESC: Non-attribute: Large - 無属性：大
	pub Neutral_L:i32,

	/// NAME: Non-attribute: Designation 1 - 無属性：指定1
	/// DESC: Non-attribute: Designation 1 - 無属性：指定1
	pub Neutral_Specific1:i32,

	/// NAME: Non-attribute: Designation 2 - 無属性：指定2
	/// DESC: Non-attribute: Designation 2 - 無属性：指定2
	pub Neutral_Specific2:i32,
}

impl Paramdef for HIT_EFFECT_SFX_PARAM_ST {
const NAME: &'static str = "HIT_EFFECT_SFX_PARAM_ST";
const VERSION: u16 = 1;
}
