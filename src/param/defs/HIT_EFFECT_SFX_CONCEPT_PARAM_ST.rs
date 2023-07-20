/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct HIT_EFFECT_SFX_CONCEPT_PARAM_ST {

	/// NAME: Iron: Concept 1 - 鉄製：概念１
	/// DESC: Iron: Concept 1 - 鉄製：概念１
	pub atkIron_1:i16,

	/// NAME: Iron: Concept 2 - 鉄製：概念２
	/// DESC: Iron: Concept 2 - 鉄製：概念２
	pub atkIron_2:i16,

	/// NAME: Made of leather: Concept 1 - 革製：概念１
	/// DESC: Leather: Concept 1 - 革：概念１
	pub atkLeather_1:i16,

	/// NAME: Made of leather: Concept 2 - 革製：概念２
	/// DESC: Leather: Concept 2 - 革：概念２
	pub atkLeather_2:i16,

	/// NAME: Wooden: Concept 1 - 木製：概念１
	/// DESC: Wooden: Concept 1 - 木製：概念１
	pub atkWood_1:i16,

	/// NAME: Wooden: Concept 2 - 木製：概念２
	/// DESC: Wooden: Concept 2 - 木製：概念２
	pub atkWood_2:i16,

	/// NAME: Meat: Concept 1 - 肉：概念１
	/// DESC: Meat: Concept 1 - 肉：概念１
	pub atkBody_1:i16,

	/// NAME: Meat: Concept 2 - 肉：概念２
	/// DESC: Meat: Concept 2 - 肉：概念２
	pub atkBody_2:i16,

	/// NAME: Stone: Concept 1 - 石製：概念１
	/// DESC: Corrosion: Concept 1 - 蝕：概念１
	pub atkStone_1:i16,

	/// NAME: Stone: Concept 2 - 石製：概念２
	/// DESC: Corrosion: Concept 2 - 蝕：概念２
	pub atkStone_2:i16,

	/// NAME: pad - pad
	pub pad:[u8;4],

	/// NAME: None: Concept 1 - なし：概念１
	/// DESC: None: Concept 1 - なし：概念１
	pub atkNone_1:i16,

	/// NAME: None: Concept 2 - なし：概念２
	/// DESC: None: Concept 2 - なし：概念２
	pub atkNone_2:i16,

	/// NAME: Reserved area - 予約領域
	pub reserve:[u8;52],
}

impl Paramdef for HIT_EFFECT_SFX_CONCEPT_PARAM_ST {
const NAME: &'static str = "HIT_EFFECT_SFX_CONCEPT_PARAM_ST";
const VERSION: u16 = 2;
}
