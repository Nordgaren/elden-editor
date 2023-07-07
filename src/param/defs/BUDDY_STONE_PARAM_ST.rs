/* This file was automatically generated from XML paramdefs. */
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct BUDDY_STONE_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Conversation character entity ID - 会話キャラエンティティID
	/// DESC: Used as a foreign key when referencing from a conversation. - 会話から参照する時の外部キーとして使う。
	pub talkChrEntityId:u32,

	/// NAME: Defeat Target List Entity ID - 撃破対象リストエンティティID
	/// DESC: Entity ID of the character / group to be defeated by the buddy when summoned from this stele - この石碑から召喚した際に、バディの撃破対象になるキャラ/グループのエンティティID
	pub eliminateTargetEntityId:u32,

	/// NAME: Summoned event flag ID - 召喚済みイベントフラグID
	/// DESC: Flag ID that stands when summoned from a stone monument. When this flag is set, the stone monument cannot be summoned. - 一度石碑から召喚した際に立つフラグID。このフラグが立っていると、石碑が召喚不可になる。
	pub summonedEventFlagId:u32,

	/// NAME: Is it special? - スペシャルか
	/// DESC: Is the stone monument an SP stone monument or a general-purpose stone monument? A bool that distinguishes. - 石碑がSP石碑か汎用石碑か？を区別するbool。
	pub Bitfield2:u8,

	/// NAME: Padding - パディング
	pub pad2:[u8;3],

	/// NAME: Buddy ID - バディID
	/// DESC: ID of the buddy parameter. If "Special" is ○, this buddy ID will be summoned. - バディパラメータのID。「スペシャルか」が○の場合、このバディIDが召喚される。
	pub buddyId:i32,

	/// NAME: Special effects ID for doping - ドーピング用特殊効果ID
	/// DESC: Special effect ID applied to the buddy when summoning the buddy. - バディ召喚時に、バディにかかる特殊効果ID。
	pub dopingSpEffectId:i32,

	/// NAME: Buddy activation distance [m] - バディ起動距離[m]
	/// DESC: If there is at least one character to be defeated in this range, you can summon a buddy at that stone monument. - 撃破対象のキャラがこの範囲に1体でも居れば、その石碑でバディ召喚が可能になる
	pub activateRange:u16,

	/// NAME: Buddy homecoming distance overwrite [m] - バディ帰巣距離上書き[m]
	/// DESC: Buddy's homecoming distance can be overridden - バディの帰巣距離を上書きできる
	pub overwriteReturnRange:i16,

	/// NAME: Launch range overwrite area entity ID - 起動範囲上書き領域エンティティID
	/// DESC: The range where the buddy can be summoned can be overwritten in the area of the specified entity ID. - バディを召喚できる範囲を、指定エンティティIDの領域で上書きできる
	pub overwriteActivateRegionEntityId:u32,

	/// NAME: Warning range overwrite area entity ID - 警告範囲上書き領域エンティティID
	/// DESC: Warning area entity ID - 警告領域エンティティID
	pub warnRegionEntityId:u32,

	/// NAME: Padding - パディング
	pub pad3:[u8;24],
}

impl BUDDY_STONE_PARAM_ST {
	/// Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	/// Bitfield1
	pub fn get_disableParam_NT(&self) -> bool {
		&self.Bitfield1 & 0x1 != 0
	}

	/// Bitfield1
	pub fn set_disableParam_NT(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x1
		} else {
			self.Bitfield1 &= 0xFE
		}
	}
	/// Reserve for package output 1 - パッケージ出力用リザーブ1
	/// Bitfield1
	pub fn get_disableParamReserve1(&self) -> u8 {
		&self.Bitfield1 & 0xFE
	}

	/// Bitfield1 MAX: 127
	pub fn set_disableParamReserve1(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0xFE;
			let newVal = &self.Bitfield1 & 0x1 | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0x1
		}
	}	/// Is the stone monument an SP stone monument or a general-purpose stone monument? A bool that distinguishes. - 石碑がSP石碑か汎用石碑か？を区別するbool。
	/// Bitfield2
	pub fn get_isSpecial(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_isSpecial(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// 
	/// Bitfield2
	pub fn get_pad1(&self) -> u8 {
		&self.Bitfield2 & 0xFE
	}

	/// Bitfield2 MAX: 127
	pub fn set_pad1(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0xFE;
			let newVal = &self.Bitfield2 & 0x1 | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0x1
		}
	}
}
