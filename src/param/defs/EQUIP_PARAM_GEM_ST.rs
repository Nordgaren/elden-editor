/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct EQUIP_PARAM_GEM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Icon ID - アイコンID
	/// DESC: Menu icon ID - メニュー用アイコンID
	pub iconId:u16,

	/// NAME: Magic stone rank - 魔石ランク
	/// DESC: Magic stone rank - 魔石ランク
	pub rank:i8,

	/// NAME: Sort item type ID - ソートアイテム種別ID
	/// DESC: Sort item type ID. In the sort "Item type order", the same ID is displayed together as the same group. - ソートアイテム種別ID。ソート「アイテム種別順」にて、同じIDは同じグループとしてまとめて表示されます
	pub sortGroupId:u8,

	/// NAME: Resident special effect ID00 - 常駐特殊効果ID00
	/// DESC: Special effect ID00 - 特殊効果ID00
	pub spEffectId0:i32,

	/// NAME: Resident special effect ID01 - 常駐特殊効果ID01
	/// DESC: Special effect ID01 - 特殊効果ID01
	pub spEffectId1:i32,

	/// NAME: Resident special effect ID02 - 常駐特殊効果ID02
	/// DESC: Special effect ID02 - 特殊効果ID02
	pub spEffectId2:i32,

	/// NAME: Item acquisition tutorial Judgment flag ID - アイテム入手チュートリアル判定フラグID
	/// DESC: Event flag ID for the tutorial when you first get the item. Flag ON when item is obtained. - 初めてアイテム入手した時のチュートリアル用のイベントフラグID。アイテム入手時にフラグON。
	pub itemGetTutorialFlagId:u32,

	/// NAME: Change destination arts parameter ID - 変化先アーツパラメータID
	/// DESC: ID of the destination arts parameter - 変化先アーツパラメータのID
	pub swordArtsParamId:i32,

	/// NAME: Installation price - 装着価格
	/// DESC: Installation price - 装着価格
	pub mountValue:i32,

	/// NAME: Sale price - 売却価格
	/// DESC: Sale price - 売却価格
	pub sellValue:i32,

	/// NAME: Selling price - 販売価格
	/// DESC: Selling price - 販売価格
	pub saleValue:i32,

	/// NAME: Sort ID - ソートID
	/// DESC: Sort ID (-1: Do not collect) - ソートID(-1:集めない)
	pub sortId:i32,

	/// NAME: Comptrophy SEQ number - コンプトロフィーSEQ番号
	/// DESC: SEQ number of complete trophy - コンプリート系トロフィのSEQ番号
	pub compTrophySedId:i16,

	/// NAME: Trophy SEQ number - トロフィーSEQ番号
	/// DESC: Trophy SEQ number - トロフィーのSEQ番号
	pub trophySeqId:i16,

	/// NAME: 0 - 0
	/// DESC: Configurable weapon attribute ID 0 - 設定可能武器属性ID0
	pub Bitfield2:u8,

	/// NAME: 8 - 8
	/// DESC: Configurable Weapon Attribute ID 8 - 設定可能武器属性ID8
	pub Bitfield3:u8,

	/// NAME: Rarity - レア度
	/// DESC: Rarity used in item acquisition logs - アイテム取得ログで使うレア度
	pub rarity:u8,

	/// NAME: 16 16 - 16
	/// DESC: Configurable Weapon Attribute ID 16 - 設定可能武器属性ID16
	pub Bitfield4:u8,

	/// NAME: Can you throw it away - 捨てれるか
	/// DESC: Can you throw away the item? TRUE = thrown away - アイテムを捨てれるか？TRUE=捨てれる
	pub Bitfield5:u8,

	/// NAME: Default weapon attribute ID - デフォルト武器属性ID
	/// DESC: Default weapon attribute ID. Can be installed even with unopened weapon attributes - デフォルト武器属性ID。開放されてない武器属性でも装着可能になる
	pub defaultWepAttr:u8,

	/// NAME: pad2 - pad2
	/// DESC: pad2 - pad2
	pub pad2:[u8;2],

	/// NAME: dagger - 短剣
	/// DESC: Can it be attached to "Weapon type: Dagger"? No input becomes x - 「武器種別：短剣」に装着可能か。未入力は×になる
	pub Bitfield6:u8,

	/// NAME: Sword - 刺剣
	/// DESC: Can it be attached to "Weapon type: Sword"? No input becomes x - 「武器種別：刺剣」に装着可能か。未入力は×になる
	pub Bitfield7:u8,

	/// NAME: Long spear - 長槍
	/// DESC: Can it be attached to "Weapon Type: Long Spear"? No input becomes x - 「武器種別：長槍」に装着可能か。未入力は×になる
	pub Bitfield8:u8,

	/// NAME: Small bow - 小弓
	/// DESC: Can it be attached to "Weapon type: Small bow"? No input becomes x - 「武器種別：小弓」に装着可能か。未入力は×になる
	pub Bitfield9:u8,

	/// NAME: Small shield - 小盾
	/// DESC: Can it be attached to "Weapon Type: Small Shield"? No input becomes x - 「武器種別：小盾」に装着可能か。未入力は×になる
	pub Bitfield10:u8,

	/// NAME: Reserved area (weapon type that can be installed) - 予約領域（装着可能な武器種別か）
	/// DESC: Reserved area for each type of weapon that can be installed (64 bits in total) - 装着可能な武器種別かの予約領域（全部で64bit分確保）
	pub reserved2_canMountWep:[u8;3],

	/// NAME: Effect text ID 00 - 効果テキストID00
	/// DESC: Effect text ID 00 (Gem_Effect). Magic stone effect text to display in status - 効果テキストID00(Gem_Effect)。ステータスに表示する魔石の効果テキスト
	pub spEffectMsgId0:i32,

	/// NAME: Effect text ID 01 - 効果テキストID01
	/// DESC: Effect text ID 01 (Gem_Effect). Magic stone effect text to display in status - 効果テキストID01(Gem_Effect)。ステータスに表示する魔石の効果テキスト
	pub spEffectMsgId1:i32,

	/// NAME: Special effect ID00 on attack hit - 攻撃ヒット時特殊効果ID00
	/// DESC: Special effect parameter ID for attack hit - 攻撃ヒット時用の特殊効果パラメータID
	pub spEffectId_forAtk0:i32,

	/// NAME: Special effect ID01 on attack hit - 攻撃ヒット時特殊効果ID01
	/// DESC: Special effect parameter ID for attack hit - 攻撃ヒット時用の特殊効果パラメータID
	pub spEffectId_forAtk1:i32,

	/// NAME: Special effect ID02 on attack hit - 攻撃ヒット時特殊効果ID02
	/// DESC: Special effect parameter ID for attack hit - 攻撃ヒット時用の特殊効果パラメータID
	pub spEffectId_forAtk2:i32,

	/// NAME: Corresponding weapon type overwrite text ID - 対応武器種別上書きテキストID
	/// DESC: Corresponding Weapon Type Overwrite Text ID (-1: Do not overwrite) [Menu Text] - 対応武器種別上書きテキストID(-1:上書きしない)[MenuText]
	pub mountWepTextId:i32,

	/// NAME: pad6 - pad6
	pub pad6:[u8;8],
}

impl Paramdef for EQUIP_PARAM_GEM_ST {
const NAME: &'static str = "EQUIP_PARAM_GEM_ST";
const VERSION: u16 = 3;
}
impl EQUIP_PARAM_GEM_ST {
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
	}	/// Configurable weapon attribute ID 0 - 設定可能武器属性ID0
	/// Bitfield2
	pub fn get_configurableWepAttr00(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_configurableWepAttr00(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// Configurable Weapon Attribute ID 1 - 設定可能武器属性ID1
	/// Bitfield2
	pub fn get_configurableWepAttr01(&self) -> bool {
		&self.Bitfield2 & 0x2 != 0
	}

	/// Bitfield2
	pub fn set_configurableWepAttr01(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x2
		} else {
			self.Bitfield2 &= 0xFD
		}
	}
	/// Configurable Weapon Attribute ID 2 - 設定可能武器属性ID2
	/// Bitfield2
	pub fn get_configurableWepAttr02(&self) -> bool {
		&self.Bitfield2 & 0x4 != 0
	}

	/// Bitfield2
	pub fn set_configurableWepAttr02(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x4
		} else {
			self.Bitfield2 &= 0xFB
		}
	}
	/// Configurable Weapon Attribute ID 3 - 設定可能武器属性ID3
	/// Bitfield2
	pub fn get_configurableWepAttr03(&self) -> bool {
		&self.Bitfield2 & 0x8 != 0
	}

	/// Bitfield2
	pub fn set_configurableWepAttr03(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x8
		} else {
			self.Bitfield2 &= 0xF7
		}
	}
	/// Configurable Weapon Attribute ID 4 - 設定可能武器属性ID4
	/// Bitfield2
	pub fn get_configurableWepAttr04(&self) -> bool {
		&self.Bitfield2 & 0x10 != 0
	}

	/// Bitfield2
	pub fn set_configurableWepAttr04(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x10
		} else {
			self.Bitfield2 &= 0xEF
		}
	}
	/// Configurable Weapon Attribute ID 5 - 設定可能武器属性ID5
	/// Bitfield2
	pub fn get_configurableWepAttr05(&self) -> bool {
		&self.Bitfield2 & 0x20 != 0
	}

	/// Bitfield2
	pub fn set_configurableWepAttr05(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x20
		} else {
			self.Bitfield2 &= 0xDF
		}
	}
	/// Configurable Weapon Attribute ID 6 - 設定可能武器属性ID6
	/// Bitfield2
	pub fn get_configurableWepAttr06(&self) -> bool {
		&self.Bitfield2 & 0x40 != 0
	}

	/// Bitfield2
	pub fn set_configurableWepAttr06(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x40
		} else {
			self.Bitfield2 &= 0xBF
		}
	}
	/// Configurable Weapon Attribute ID 7 - 設定可能武器属性ID7
	/// Bitfield2
	pub fn get_configurableWepAttr07(&self) -> bool {
		&self.Bitfield2 & 0x80 != 0
	}

	/// Bitfield2
	pub fn set_configurableWepAttr07(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x80
		} else {
			self.Bitfield2 &= 0x7F
		}
	}
	/// Configurable Weapon Attribute ID 8 - 設定可能武器属性ID8
	/// Bitfield3
	pub fn get_configurableWepAttr08(&self) -> bool {
		&self.Bitfield3 & 0x1 != 0
	}

	/// Bitfield3
	pub fn set_configurableWepAttr08(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x1
		} else {
			self.Bitfield3 &= 0xFE
		}
	}
	/// Configurable Weapon Attribute ID 9 - 設定可能武器属性ID9
	/// Bitfield3
	pub fn get_configurableWepAttr09(&self) -> bool {
		&self.Bitfield3 & 0x2 != 0
	}

	/// Bitfield3
	pub fn set_configurableWepAttr09(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x2
		} else {
			self.Bitfield3 &= 0xFD
		}
	}
	/// Configurable Weapon Attribute ID 10 - 設定可能武器属性ID10
	/// Bitfield3
	pub fn get_configurableWepAttr10(&self) -> bool {
		&self.Bitfield3 & 0x4 != 0
	}

	/// Bitfield3
	pub fn set_configurableWepAttr10(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x4
		} else {
			self.Bitfield3 &= 0xFB
		}
	}
	/// Configurable Weapon Attribute ID 11 - 設定可能武器属性ID11
	/// Bitfield3
	pub fn get_configurableWepAttr11(&self) -> bool {
		&self.Bitfield3 & 0x8 != 0
	}

	/// Bitfield3
	pub fn set_configurableWepAttr11(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x8
		} else {
			self.Bitfield3 &= 0xF7
		}
	}
	/// Configurable Weapon Attribute ID 12 - 設定可能武器属性ID12
	/// Bitfield3
	pub fn get_configurableWepAttr12(&self) -> bool {
		&self.Bitfield3 & 0x10 != 0
	}

	/// Bitfield3
	pub fn set_configurableWepAttr12(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x10
		} else {
			self.Bitfield3 &= 0xEF
		}
	}
	/// Configurable Weapon Attribute ID 13 - 設定可能武器属性ID13
	/// Bitfield3
	pub fn get_configurableWepAttr13(&self) -> bool {
		&self.Bitfield3 & 0x20 != 0
	}

	/// Bitfield3
	pub fn set_configurableWepAttr13(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x20
		} else {
			self.Bitfield3 &= 0xDF
		}
	}
	/// Configurable Weapon Attribute ID 14 - 設定可能武器属性ID14
	/// Bitfield3
	pub fn get_configurableWepAttr14(&self) -> bool {
		&self.Bitfield3 & 0x40 != 0
	}

	/// Bitfield3
	pub fn set_configurableWepAttr14(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x40
		} else {
			self.Bitfield3 &= 0xBF
		}
	}
	/// Configurable Weapon Attribute ID 15 - 設定可能武器属性ID15
	/// Bitfield3
	pub fn get_configurableWepAttr15(&self) -> bool {
		&self.Bitfield3 & 0x80 != 0
	}

	/// Bitfield3
	pub fn set_configurableWepAttr15(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x80
		} else {
			self.Bitfield3 &= 0x7F
		}
	}
	/// Configurable Weapon Attribute ID 16 - 設定可能武器属性ID16
	/// Bitfield4
	pub fn get_configurableWepAttr16(&self) -> bool {
		&self.Bitfield4 & 0x1 != 0
	}

	/// Bitfield4
	pub fn set_configurableWepAttr16(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x1
		} else {
			self.Bitfield4 &= 0xFE
		}
	}
	/// Configurable Weapon Attribute ID 17 - 設定可能武器属性ID17
	/// Bitfield4
	pub fn get_configurableWepAttr17(&self) -> bool {
		&self.Bitfield4 & 0x2 != 0
	}

	/// Bitfield4
	pub fn set_configurableWepAttr17(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x2
		} else {
			self.Bitfield4 &= 0xFD
		}
	}
	/// Configurable Weapon Attribute ID 18 - 設定可能武器属性ID18
	/// Bitfield4
	pub fn get_configurableWepAttr18(&self) -> bool {
		&self.Bitfield4 & 0x4 != 0
	}

	/// Bitfield4
	pub fn set_configurableWepAttr18(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x4
		} else {
			self.Bitfield4 &= 0xFB
		}
	}
	/// Configurable Weapon Attribute ID 19 - 設定可能武器属性ID19
	/// Bitfield4
	pub fn get_configurableWepAttr19(&self) -> bool {
		&self.Bitfield4 & 0x8 != 0
	}

	/// Bitfield4
	pub fn set_configurableWepAttr19(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x8
		} else {
			self.Bitfield4 &= 0xF7
		}
	}
	/// Configurable Weapon Attribute ID 20 - 設定可能武器属性ID20
	/// Bitfield4
	pub fn get_configurableWepAttr20(&self) -> bool {
		&self.Bitfield4 & 0x10 != 0
	}

	/// Bitfield4
	pub fn set_configurableWepAttr20(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x10
		} else {
			self.Bitfield4 &= 0xEF
		}
	}
	/// Configurable Weapon Attribute ID 21 - 設定可能武器属性ID21
	/// Bitfield4
	pub fn get_configurableWepAttr21(&self) -> bool {
		&self.Bitfield4 & 0x20 != 0
	}

	/// Bitfield4
	pub fn set_configurableWepAttr21(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x20
		} else {
			self.Bitfield4 &= 0xDF
		}
	}
	/// Configurable Weapon Attribute ID 22 - 設定可能武器属性ID22
	/// Bitfield4
	pub fn get_configurableWepAttr22(&self) -> bool {
		&self.Bitfield4 & 0x40 != 0
	}

	/// Bitfield4
	pub fn set_configurableWepAttr22(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x40
		} else {
			self.Bitfield4 &= 0xBF
		}
	}
	/// Configurable Weapon Attribute ID 23 - 設定可能武器属性ID23
	/// Bitfield4
	pub fn get_configurableWepAttr23(&self) -> bool {
		&self.Bitfield4 & 0x80 != 0
	}

	/// Bitfield4
	pub fn set_configurableWepAttr23(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x80
		} else {
			self.Bitfield4 &= 0x7F
		}
	}
	/// Can you throw away the item? TRUE = thrown away - アイテムを捨てれるか？TRUE=捨てれる
	/// Bitfield5
	pub fn get_isDiscard(&self) -> bool {
		&self.Bitfield5 & 0x1 != 0
	}

	/// Bitfield5
	pub fn set_isDiscard(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x1
		} else {
			self.Bitfield5 &= 0xFE
		}
	}
	/// Can I put the item on the spot? TRUE = can be placed - アイテムをその場に置けるか？TRUE=置ける
	/// Bitfield5
	pub fn get_isDrop(&self) -> bool {
		&self.Bitfield5 & 0x2 != 0
	}

	/// Bitfield5
	pub fn set_isDrop(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x2
		} else {
			self.Bitfield5 &= 0xFD
		}
	}
	/// Can you leave it in the warehouse? - 倉庫に預けれるか
	/// Bitfield5
	pub fn get_isDeposit(&self) -> bool {
		&self.Bitfield5 & 0x4 != 0
	}

	/// Bitfield5
	pub fn set_isDeposit(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x4
		} else {
			self.Bitfield5 &= 0xFB
		}
	}
	/// Is multi-drop sharing prohibited? - マルチドロップ共有禁止か
	/// Bitfield5
	pub fn get_disableMultiDropShare(&self) -> bool {
		&self.Bitfield5 & 0x8 != 0
	}

	/// Bitfield5
	pub fn set_disableMultiDropShare(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x8
		} else {
			self.Bitfield5 &= 0xF7
		}
	}
	/// Whether to display it in the item acquisition dialog when acquiring an item (not entered: new only) - アイテム取得時にアイテム取得ダイアログに表示するか（未入力: newのみ）
	/// Bitfield5
	pub fn get_showDialogCondType(&self) -> u8 {
		&self.Bitfield5 & 0x30
	}

	/// Bitfield5 MAX: 3
	pub fn set_showDialogCondType(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 4) & 0x30;
			let newVal = &self.Bitfield5 & 0xCF | val;
			self.Bitfield5 = newVal
		} else {
			self.Bitfield5 &= 0xCF
		}
	}	/// Whether to display in the item acquisition log when acquiring the item (not entered: ○) - アイテム取得時にアイテム取得ログに表示するか（未入力: ○）
	/// Bitfield5
	pub fn get_showLogCondType(&self) -> bool {
		&self.Bitfield5 & 0x40 != 0
	}

	/// Bitfield5
	pub fn set_showLogCondType(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x40
		} else {
			self.Bitfield5 &= 0xBF
		}
	}
	/// pad - pad
	/// Bitfield5
	pub fn get_pad(&self) -> bool {
		&self.Bitfield5 & 0x80 != 0
	}

	/// Bitfield5
	pub fn set_pad(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x80
		} else {
			self.Bitfield5 &= 0x7F
		}
	}
	/// Can it be attached to "Weapon type: Dagger"? No input becomes x - 「武器種別：短剣」に装着可能か。未入力は×になる
	/// Bitfield6
	pub fn get_canMountWep_Dagger(&self) -> bool {
		&self.Bitfield6 & 0x1 != 0
	}

	/// Bitfield6
	pub fn set_canMountWep_Dagger(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x1
		} else {
			self.Bitfield6 &= 0xFE
		}
	}
	/// Can it be attached to "Weapon type: Straight sword"? No input becomes x - 「武器種別：直剣」に装着可能か。未入力は×になる
	/// Bitfield6
	pub fn get_canMountWep_SwordNormal(&self) -> bool {
		&self.Bitfield6 & 0x2 != 0
	}

	/// Bitfield6
	pub fn set_canMountWep_SwordNormal(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x2
		} else {
			self.Bitfield6 &= 0xFD
		}
	}
	/// Can it be attached to "Weapon Type: Large Sword"? No input becomes x - 「武器種別：大剣」に装着可能か。未入力は×になる
	/// Bitfield6
	pub fn get_canMountWep_SwordLarge(&self) -> bool {
		&self.Bitfield6 & 0x4 != 0
	}

	/// Bitfield6
	pub fn set_canMountWep_SwordLarge(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x4
		} else {
			self.Bitfield6 &= 0xFB
		}
	}
	/// Can it be attached to "Weapon Type: Oversized Sword"? No input becomes x - 「武器種別：特大剣」に装着可能か。未入力は×になる
	/// Bitfield6
	pub fn get_canMountWep_SwordGigantic(&self) -> bool {
		&self.Bitfield6 & 0x8 != 0
	}

	/// Bitfield6
	pub fn set_canMountWep_SwordGigantic(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x8
		} else {
			self.Bitfield6 &= 0xF7
		}
	}
	/// Can it be attached to "Weapon Type: Curly Sword"? No input becomes x - 「武器種別：曲剣 」に装着可能か。未入力は×になる
	/// Bitfield6
	pub fn get_canMountWep_SaberNormal(&self) -> bool {
		&self.Bitfield6 & 0x10 != 0
	}

	/// Bitfield6
	pub fn set_canMountWep_SaberNormal(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x10
		} else {
			self.Bitfield6 &= 0xEF
		}
	}
	/// Can it be attached to "Weapon Type: Omagari Sword"? No input becomes x - 「武器種別：大曲剣」に装着可能か。未入力は×になる
	/// Bitfield6
	pub fn get_canMountWep_SaberLarge(&self) -> bool {
		&self.Bitfield6 & 0x20 != 0
	}

	/// Bitfield6
	pub fn set_canMountWep_SaberLarge(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x20
		} else {
			self.Bitfield6 &= 0xDF
		}
	}
	/// Can it be attached to "Weapon type: Sword"? No input becomes x - 「武器種別：刀」に装着可能か。未入力は×になる
	/// Bitfield6
	pub fn get_canMountWep_katana(&self) -> bool {
		&self.Bitfield6 & 0x40 != 0
	}

	/// Bitfield6
	pub fn set_canMountWep_katana(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x40
		} else {
			self.Bitfield6 &= 0xBF
		}
	}
	/// Can it be attached to "Weapon type: Double-edged sword"? No input becomes x - 「武器種別：両刃剣」に装着可能か。未入力は×になる
	/// Bitfield6
	pub fn get_canMountWep_SwordDoubleEdge(&self) -> bool {
		&self.Bitfield6 & 0x80 != 0
	}

	/// Bitfield6
	pub fn set_canMountWep_SwordDoubleEdge(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x80
		} else {
			self.Bitfield6 &= 0x7F
		}
	}
	/// Can it be attached to "Weapon type: Sword"? No input becomes x - 「武器種別：刺剣」に装着可能か。未入力は×になる
	/// Bitfield7
	pub fn get_canMountWep_SwordPierce(&self) -> bool {
		&self.Bitfield7 & 0x1 != 0
	}

	/// Bitfield7
	pub fn set_canMountWep_SwordPierce(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x1
		} else {
			self.Bitfield7 &= 0xFE
		}
	}
	/// Can it be attached to "Weapon Type: Large Sword"? No input becomes x - 「武器種別：大刺剣」に装着可能か。未入力は×になる
	/// Bitfield7
	pub fn get_canMountWep_RapierHeavy(&self) -> bool {
		&self.Bitfield7 & 0x2 != 0
	}

	/// Bitfield7
	pub fn set_canMountWep_RapierHeavy(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x2
		} else {
			self.Bitfield7 &= 0xFD
		}
	}
	/// Can it be attached to "Weapon Type: Ax"? No input becomes x - 「武器種別：斧」に装着可能か。未入力は×になる
	/// Bitfield7
	pub fn get_canMountWep_AxeNormal(&self) -> bool {
		&self.Bitfield7 & 0x4 != 0
	}

	/// Bitfield7
	pub fn set_canMountWep_AxeNormal(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x4
		} else {
			self.Bitfield7 &= 0xFB
		}
	}
	/// Can it be attached to "Weapon Type: Large Ax"? No input becomes x - 「武器種別：大斧」に装着可能か。未入力は×になる
	/// Bitfield7
	pub fn get_canMountWep_AxeLarge(&self) -> bool {
		&self.Bitfield7 & 0x8 != 0
	}

	/// Bitfield7
	pub fn set_canMountWep_AxeLarge(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x8
		} else {
			self.Bitfield7 &= 0xF7
		}
	}
	/// Can it be attached to "Weapon type: Hammer"? No input becomes x - 「武器種別：槌」に装着可能か。未入力は×になる
	/// Bitfield7
	pub fn get_canMountWep_HammerNormal(&self) -> bool {
		&self.Bitfield7 & 0x10 != 0
	}

	/// Bitfield7
	pub fn set_canMountWep_HammerNormal(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x10
		} else {
			self.Bitfield7 &= 0xEF
		}
	}
	/// Can it be attached to "Weapon Type: Gavel"? No input becomes x - 「武器種別：大槌」に装着可能か。未入力は×になる
	/// Bitfield7
	pub fn get_canMountWep_HammerLarge(&self) -> bool {
		&self.Bitfield7 & 0x20 != 0
	}

	/// Bitfield7
	pub fn set_canMountWep_HammerLarge(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x20
		} else {
			self.Bitfield7 &= 0xDF
		}
	}
	/// Can it be attached to "Weapon Type: Frail"? No input becomes x - 「武器種別：フレイル」に装着可能か。未入力は×になる
	/// Bitfield7
	pub fn get_canMountWep_Flail(&self) -> bool {
		&self.Bitfield7 & 0x40 != 0
	}

	/// Bitfield7
	pub fn set_canMountWep_Flail(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x40
		} else {
			self.Bitfield7 &= 0xBF
		}
	}
	/// Can it be attached to "Weapon Type: Spear"? No input becomes x - 「武器種別：槍」に装着可能か。未入力は×になる
	/// Bitfield7
	pub fn get_canMountWep_SpearNormal(&self) -> bool {
		&self.Bitfield7 & 0x80 != 0
	}

	/// Bitfield7
	pub fn set_canMountWep_SpearNormal(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x80
		} else {
			self.Bitfield7 &= 0x7F
		}
	}
	/// Can it be attached to "Weapon Type: Long Spear"? No input becomes x - 「武器種別：長槍」に装着可能か。未入力は×になる
	/// Bitfield8
	pub fn get_canMountWep_SpearLarge(&self) -> bool {
		&self.Bitfield8 & 0x1 != 0
	}

	/// Bitfield8
	pub fn set_canMountWep_SpearLarge(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x1
		} else {
			self.Bitfield8 &= 0xFE
		}
	}
	/// Can it be attached to "Weapon Type: Large Spear"? No input becomes x - 「武器種別：大槍」に装着可能か。未入力は×になる
	/// Bitfield8
	pub fn get_canMountWep_SpearHeavy(&self) -> bool {
		&self.Bitfield8 & 0x2 != 0
	}

	/// Bitfield8
	pub fn set_canMountWep_SpearHeavy(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x2
		} else {
			self.Bitfield8 &= 0xFD
		}
	}
	/// Can it be attached to "Weapon Type: Halberd"? No input becomes x - 「武器種別：斧槍」に装着可能か。未入力は×になる
	/// Bitfield8
	pub fn get_canMountWep_SpearAxe(&self) -> bool {
		&self.Bitfield8 & 0x4 != 0
	}

	/// Bitfield8
	pub fn set_canMountWep_SpearAxe(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x4
		} else {
			self.Bitfield8 &= 0xFB
		}
	}
	/// Can it be attached to "Weapon type: Sickle"? No input becomes x - 「武器種別：鎌」に装着可能か。未入力は×になる
	/// Bitfield8
	pub fn get_canMountWep_Sickle(&self) -> bool {
		&self.Bitfield8 & 0x8 != 0
	}

	/// Bitfield8
	pub fn set_canMountWep_Sickle(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x8
		} else {
			self.Bitfield8 &= 0xF7
		}
	}
	/// Can it be attached to "Weapon type: Fist"? No input becomes x - 「武器種別：拳」に装着可能か。未入力は×になる
	/// Bitfield8
	pub fn get_canMountWep_Knuckle(&self) -> bool {
		&self.Bitfield8 & 0x10 != 0
	}

	/// Bitfield8
	pub fn set_canMountWep_Knuckle(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x10
		} else {
			self.Bitfield8 &= 0xEF
		}
	}
	/// Can it be attached to "Weapon type: Claw"? No input becomes x - 「武器種別：爪」に装着可能か。未入力は×になる
	/// Bitfield8
	pub fn get_canMountWep_Claw(&self) -> bool {
		&self.Bitfield8 & 0x20 != 0
	}

	/// Bitfield8
	pub fn set_canMountWep_Claw(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x20
		} else {
			self.Bitfield8 &= 0xDF
		}
	}
	/// Can it be attached to "Weapon type: Whip"? No input becomes x - 「武器種別：ムチ」に装着可能か。未入力は×になる
	/// Bitfield8
	pub fn get_canMountWep_Whip(&self) -> bool {
		&self.Bitfield8 & 0x40 != 0
	}

	/// Bitfield8
	pub fn set_canMountWep_Whip(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x40
		} else {
			self.Bitfield8 &= 0xBF
		}
	}
	/// Can it be attached to "Weapon type: Oversized ax mallet"? No input becomes x - 「武器種別：特大斧槌」に装着可能か。未入力は×になる
	/// Bitfield8
	pub fn get_canMountWep_AxhammerLarge(&self) -> bool {
		&self.Bitfield8 & 0x80 != 0
	}

	/// Bitfield8
	pub fn set_canMountWep_AxhammerLarge(&mut self, state: bool) {
		if state {
			self.Bitfield8 |= 0x80
		} else {
			self.Bitfield8 &= 0x7F
		}
	}
	/// Can it be attached to "Weapon type: Small bow"? No input becomes x - 「武器種別：小弓」に装着可能か。未入力は×になる
	/// Bitfield9
	pub fn get_canMountWep_BowSmall(&self) -> bool {
		&self.Bitfield9 & 0x1 != 0
	}

	/// Bitfield9
	pub fn set_canMountWep_BowSmall(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x1
		} else {
			self.Bitfield9 &= 0xFE
		}
	}
	/// Can it be attached to "Weapon Type: Bow"? No input becomes x - 「武器種別：弓」に装着可能か。未入力は×になる
	/// Bitfield9
	pub fn get_canMountWep_BowNormal(&self) -> bool {
		&self.Bitfield9 & 0x2 != 0
	}

	/// Bitfield9
	pub fn set_canMountWep_BowNormal(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x2
		} else {
			self.Bitfield9 &= 0xFD
		}
	}
	/// Can it be attached to "Weapon Type: Large Bow"? No input becomes x - 「武器種別：大弓」に装着可能か。未入力は×になる
	/// Bitfield9
	pub fn get_canMountWep_BowLarge(&self) -> bool {
		&self.Bitfield9 & 0x4 != 0
	}

	/// Bitfield9
	pub fn set_canMountWep_BowLarge(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x4
		} else {
			self.Bitfield9 &= 0xFB
		}
	}
	/// Can it be attached to "Weapon Type: Crossbow"? No input becomes x - 「武器種別：クロスボウ」に装着可能か。未入力は×になる
	/// Bitfield9
	pub fn get_canMountWep_ClossBow(&self) -> bool {
		&self.Bitfield9 & 0x8 != 0
	}

	/// Bitfield9
	pub fn set_canMountWep_ClossBow(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x8
		} else {
			self.Bitfield9 &= 0xF7
		}
	}
	/// Can it be attached to "Weapon Type: Varistor"? No input becomes x - 「武器種別：バリスタ」に装着可能か。未入力は×になる
	/// Bitfield9
	pub fn get_canMountWep_Ballista(&self) -> bool {
		&self.Bitfield9 & 0x10 != 0
	}

	/// Bitfield9
	pub fn set_canMountWep_Ballista(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x10
		} else {
			self.Bitfield9 &= 0xEF
		}
	}
	/// Can it be attached to "Weapon type: Wand"? No input becomes x - 「武器種別：杖」に装着可能か。未入力は×になる
	/// Bitfield9
	pub fn get_canMountWep_Staff(&self) -> bool {
		&self.Bitfield9 & 0x20 != 0
	}

	/// Bitfield9
	pub fn set_canMountWep_Staff(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x20
		} else {
			self.Bitfield9 &= 0xDF
		}
	}
	/// Can it be attached to "Weapon type: Tattoo"? No input becomes x - 「武器種別：入れ墨」に装着可能か。未入力は×になる
	/// Bitfield9
	pub fn get_canMountWep_Sorcery(&self) -> bool {
		&self.Bitfield9 & 0x40 != 0
	}

	/// Bitfield9
	pub fn set_canMountWep_Sorcery(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x40
		} else {
			self.Bitfield9 &= 0xBF
		}
	}
	/// Can it be attached to "Weapon Type: Holy Mark"? No input becomes x - 「武器種別：聖印」に装着可能か。未入力は×になる
	/// Bitfield9
	pub fn get_canMountWep_Talisman(&self) -> bool {
		&self.Bitfield9 & 0x80 != 0
	}

	/// Bitfield9
	pub fn set_canMountWep_Talisman(&mut self, state: bool) {
		if state {
			self.Bitfield9 |= 0x80
		} else {
			self.Bitfield9 &= 0x7F
		}
	}
	/// Can it be attached to "Weapon Type: Small Shield"? No input becomes x - 「武器種別：小盾」に装着可能か。未入力は×になる
	/// Bitfield10
	pub fn get_canMountWep_ShieldSmall(&self) -> bool {
		&self.Bitfield10 & 0x1 != 0
	}

	/// Bitfield10
	pub fn set_canMountWep_ShieldSmall(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x1
		} else {
			self.Bitfield10 &= 0xFE
		}
	}
	/// Can it be attached to "Weapon Type: Middle Shield"? No input becomes x - 「武器種別：中盾」に装着可能か。未入力は×になる
	/// Bitfield10
	pub fn get_canMountWep_ShieldNormal(&self) -> bool {
		&self.Bitfield10 & 0x2 != 0
	}

	/// Bitfield10
	pub fn set_canMountWep_ShieldNormal(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x2
		} else {
			self.Bitfield10 &= 0xFD
		}
	}
	/// Can it be attached to "Weapon Type: Large Shield"? No input becomes x - 「武器種別：大盾」に装着可能か。未入力は×になる
	/// Bitfield10
	pub fn get_canMountWep_ShieldLarge(&self) -> bool {
		&self.Bitfield10 & 0x4 != 0
	}

	/// Bitfield10
	pub fn set_canMountWep_ShieldLarge(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x4
		} else {
			self.Bitfield10 &= 0xFB
		}
	}
	/// Can it be attached to "Weapon type: Torch"? No input becomes x - 「武器種別：松明」に装着可能か。未入力は×になる
	/// Bitfield10
	pub fn get_canMountWep_Torch(&self) -> bool {
		&self.Bitfield10 & 0x8 != 0
	}

	/// Bitfield10
	pub fn set_canMountWep_Torch(&mut self, state: bool) {
		if state {
			self.Bitfield10 |= 0x8
		} else {
			self.Bitfield10 &= 0xF7
		}
	}
	/// Reserved area for each type of weapon that can be installed (64 bits in total) - 装着可能な武器種別かの予約領域（全部で64bit分確保）
	/// Bitfield10
	pub fn get_reserved_canMountWep(&self) -> u8 {
		&self.Bitfield10 & 0xF0
	}

	/// Bitfield10 MAX: 15
	pub fn set_reserved_canMountWep(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 4) & 0xF0;
			let newVal = &self.Bitfield10 & 0xF | val;
			self.Bitfield10 = newVal
		} else {
			self.Bitfield10 &= 0xF
		}
	}
}
