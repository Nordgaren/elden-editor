/* This file was automatically generated from XML paramdefs. */
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CHARACTER_INIT_PARAM {

	/// NAME: MP recovery speed basic value [s] - ＭＰ回復速度基本値[s]
	/// DESC: Time until MP recovers 1 point (first decimal place) - ＭＰが、1ポイント回復するまでの時間（小数点第一位）
	pub baseRec_mp:f32,

	/// NAME: Stamina recovery speed basic value [s] - スタミナ回復速度基本値[s]
	/// DESC: Time to recover 1 point of stamina (first decimal place) - スタミナが、1ポイント回復するまでの時間（小数点第一位）
	pub baseRec_sp:f32,

	/// NAME: Fall damage reduction correction [%] - 落下ダメージ軽減補正[%]
	/// DESC: Damage reduction amount (%) that can be a cushion when scolded by another character from above (first decimal place) - 他のキャラクターに上からのしかかれたときに、クッションとなりえるダメージ軽減量（％）（小数点第一位）
	pub red_Falldam:f32,

	/// NAME: Early soul - 初期ソウル
	/// DESC: The amount of soul you have at the beginning - 初期に所持しているソウル量
	pub soul:i32,

	/// NAME: Right-handed weapon slot 1 - 右手武器スロット1
	/// DESC: Weapon ID of equipment parameter (right hand slot 1) - 装備品パラメータの武器ＩＤ(右手スロット１)
	pub equip_Wep_Right:i32,

	/// NAME: Right-handed weapon slot 2 - 右手武器スロット2
	/// DESC: Weapon ID of equipment parameter (right hand slot 2) - 装備品パラメータの武器ＩＤ(右手スロット２)
	pub equip_Subwep_Right:i32,

	/// NAME: Left-handed weapon slot 1 - 左手武器スロット1
	/// DESC: Weapon ID of equipment parameter (left hand slot 1) - 装備品パラメータの武器ＩＤ(左手スロット１)
	pub equip_Wep_Left:i32,

	/// NAME: Left-handed weapon slot 2 - 左手武器スロット2
	/// DESC: Weapon ID of equipment parameter (left hand slot 2) - 装備品パラメータの武器ＩＤ(左手スロット２)
	pub equip_Subwep_Left:i32,

	/// NAME: Head armor - 頭防具
	/// DESC: Equipment parameter armor ID (head armor) - 装備品パラメータの防具ＩＤ(頭防具)
	pub equip_Helm:i32,

	/// NAME: Torso armor - 胴体防具
	/// DESC: Equipment parameter armor ID (torso armor) - 装備品パラメータの防具ＩＤ(胴体防具)
	pub equip_Armer:i32,

	/// NAME: Arm armor - 腕防具
	/// DESC: Equipment parameter armor ID (arm armor) - 装備品パラメータの防具ＩＤ(腕防具)
	pub equip_Gaunt:i32,

	/// NAME: Leg armor - 脚防具
	/// DESC: Equipment parameter armor ID (leg armor) - 装備品パラメータの防具ＩＤ(脚防具)
	pub equip_Leg:i32,

	/// NAME: arrow - 矢
	/// DESC: Weapon ID (arrow) of equipment parameter - 装備品パラメータの武器ＩＤ(矢)
	pub equip_Arrow:i32,

	/// NAME: bolt - ボルト
	/// DESC: Weapon ID (bolt) of equipment parameter - 装備品パラメータの武器ＩＤ(ボルト)
	pub equip_Bolt:i32,

	/// NAME: Spare arrow - 予備矢
	/// DESC: Weapon ID of equipment parameter (arrow reserve) - 装備品パラメータの武器ＩＤ(矢予備)
	pub equip_SubArrow:i32,

	/// NAME: Spare bolt - 予備ボルト
	/// DESC: Weapon ID of equipment parameter (spare bolt) - 装備品パラメータの武器ＩＤ(ボルト予備)
	pub equip_SubBolt:i32,

	/// NAME: Ornaments 1 - 装飾品1
	/// DESC: Equipment parameter decoration ID01 - 装備品パラメータの装飾品ＩＤ01
	pub equip_Accessory01:i32,

	/// NAME: Ornament 2 - 装飾品2
	/// DESC: Equipment parameter decoration ID02 - 装備品パラメータの装飾品ＩＤ02
	pub equip_Accessory02:i32,

	/// NAME: Ornaments 3 - 装飾品3
	/// DESC: Equipment parameter decoration ID03 - 装備品パラメータの装飾品ＩＤ03
	pub equip_Accessory03:i32,

	/// NAME: Ornament 4 - 装飾品4
	/// DESC: Equipment parameter decoration ID04 - 装備品パラメータの装飾品ＩＤ04
	pub equip_Accessory04:i32,

	/// NAME: pad - pad
	/// DESC: pad - pad
	pub pad8:[u8;4],

	/// NAME: Material ID1 for Elixir - エリクサー用素材ID1
	/// DESC: Material ID1 for Elixir - エリクサー用素材ID1
	pub elixir_material00:i32,

	/// NAME: Material ID2 for Elixir - エリクサー用素材ID2
	/// DESC: Material ID2 for Elixir - エリクサー用素材ID2
	pub elixir_material01:i32,

	/// NAME: Material ID3 for Elixir - エリクサー用素材ID3
	/// DESC: Material ID3 for Elixir - エリクサー用素材ID3
	pub elixir_material02:i32,

	/// NAME: Magic / miracle 1 - 魔法・奇跡1
	/// DESC: Initial placement magic / miracle ID01 - 初期配置の魔法・奇跡ID01
	pub equip_Spell_01:i32,

	/// NAME: Magic / miracle 2 - 魔法・奇跡2
	/// DESC: Initial placement magic / miracle ID02 - 初期配置の魔法・奇跡ID02
	pub equip_Spell_02:i32,

	/// NAME: Magic / miracle 3 - 魔法・奇跡3
	/// DESC: Initial placement magic / miracle ID03 - 初期配置の魔法・奇跡ID03
	pub equip_Spell_03:i32,

	/// NAME: Magic / miracle 4 - 魔法・奇跡4
	/// DESC: Initial placement magic / miracle ID04 - 初期配置の魔法・奇跡ID04
	pub equip_Spell_04:i32,

	/// NAME: Magic / miracle 5 - 魔法・奇跡5
	/// DESC: Initial placement magic / miracle ID05 - 初期配置の魔法・奇跡ID05
	pub equip_Spell_05:i32,

	/// NAME: Magic / miracle 6 - 魔法・奇跡6
	/// DESC: Initial placement magic / miracle ID06 - 初期配置の魔法・奇跡ID06
	pub equip_Spell_06:i32,

	/// NAME: Magic / miracle 7 - 魔法・奇跡7
	/// DESC: Initial placement magic / miracle ID07 - 初期配置の魔法・奇跡ID07
	pub equip_Spell_07:i32,

	/// NAME: Item 01 - アイテム01
	/// DESC: Initial possession item ID 01 - 初期所持のアイテムID01
	pub item_01:i32,

	/// NAME: Item 02 - アイテム02
	/// DESC: Initial possession item ID 02 - 初期所持のアイテムID02
	pub item_02:i32,

	/// NAME: Item 03 - アイテム03
	/// DESC: Initial possession item ID 03 - 初期所持のアイテムID03
	pub item_03:i32,

	/// NAME: Item 04 - アイテム04
	/// DESC: Initial possession item ID 04 - 初期所持のアイテムID04
	pub item_04:i32,

	/// NAME: Item 05 - アイテム05
	/// DESC: Initial possession item ID 05 - 初期所持のアイテムID05
	pub item_05:i32,

	/// NAME: Item 06 - アイテム06
	/// DESC: Initial possession item ID 06 - 初期所持のアイテムID06
	pub item_06:i32,

	/// NAME: Item 07 - アイテム07
	/// DESC: Initial possession item ID 07 - 初期所持のアイテムID07
	pub item_07:i32,

	/// NAME: Item 08 - アイテム08
	/// DESC: Initial possession item ID 08 - 初期所持のアイテムID08
	pub item_08:i32,

	/// NAME: Item 09 - アイテム09
	/// DESC: Initial possession item ID 09 - 初期所持のアイテムID09
	pub item_09:i32,

	/// NAME: Item 10 - アイテム10
	/// DESC: Initial possession item ID 10 - 初期所持のアイテムID10
	pub item_10:i32,

	/// NAME: Facegen Parameter ID - フェイスジェンパラメータID
	/// DESC: Facegen parameter ID used by NPC players. Not used by normal players. - NPCプレイヤーで使用するフェイスジェンパラメータID。通常プレイヤーでは使用しません。
	pub npcPlayerFaceGenId:i32,

	/// NAME: NPC player thinking ID - NPCプレイヤーの思考ID
	/// DESC: NPC Thinking Parameter ID used by NPC players. Not used by normal players. - NPCプレイヤーで使用するNPC思考パラメータID。通常プレイヤーでは使用しません。
	pub npcPlayerThinkId:i32,

	/// NAME: HP basic value - ＨＰ基本値
	/// DESC: Basic value of HP (actually, it is corrected by the calculation formula) - ＨＰの基本値（実際は、計算式で補正される）
	pub baseHp:u16,

	/// NAME: MP basic value - ＭＰ基本値
	/// DESC: Basic value of MP (actually, it is corrected by the calculation formula) - ＭＰの基本値（実際は、計算式で補正される）
	pub baseMp:u16,

	/// NAME: Stamina base value - スタミナ基本値
	/// DESC: Basic value of stamina (actually, it is corrected by the formula) - スタミナの基本値（実際は、計算式で補正される）
	pub baseSp:u16,

	/// NAME: Number of arrows possessed - 矢の所持数
	/// DESC: Initial number of arrows - 矢の初期所持数
	pub arrowNum:u16,

	/// NAME: Number of bolts - ボルトの所持数
	/// DESC: Initial number of bolts - ボルトの初期所持数
	pub boltNum:u16,

	/// NAME: Number of spare arrows possessed - 予備矢の所持数
	/// DESC: Initial number of arrows - 矢の初期所持数
	pub subArrowNum:u16,

	/// NAME: Number of spare bolts - 予備ボルトの所持数
	/// DESC: Initial number of bolts - ボルトの初期所持数
	pub subBoltNum:u16,

	/// NAME: pad - pad
	pub pad4:[u8;6],

	/// NAME: Soul Lv - ソウルLv
	/// DESC: Initial Lv - 初期Lv
	pub soulLv:i16,

	/// NAME: Physical fitness - 体力
	/// DESC: Basic value of physical fitness - 体力の基本値
	pub baseVit:u8,

	/// NAME: spirit - 精神
	/// DESC: Basic value of spirit - 精神の基本値
	pub baseWil:u8,

	/// NAME: Robust - 頑強
	/// DESC: Robust base value - 頑強の基本値
	pub baseEnd:u8,

	/// NAME: Strength - 筋力
	/// DESC: Basic value of muscle strength - 筋力の基本値
	pub baseStr:u8,

	/// NAME: Agility - 俊敏
	/// DESC: Basic value of agility - 俊敏の基本値
	pub baseDex:u8,

	/// NAME: Magical power - 魔力
	/// DESC: Basic value of magic power - 魔力の基本値
	pub baseMag:u8,

	/// NAME: faith - 信仰
	/// DESC: Basic value of faith - 信仰の基本値
	pub baseFai:u8,

	/// NAME: luck - 運
	/// DESC: Basic value of luck - 運の基本値
	pub baseLuc:u8,

	/// NAME: Human nature - 人間性
	/// DESC: Basic value of human nature - 人間性の基本値
	pub baseHeroPoint:u8,

	/// NAME: Endurance - 耐久力
	/// DESC: Basic value of endurance - 耐久力の基本値
	pub baseDurability:u8,

	/// NAME: Number of possession of item 01 - アイテム01の所持数
	/// DESC: Initial possession number of items 01 - 初期所持のアイテム個数01
	pub itemNum_01:u8,

	/// NAME: Number of possession of item 02 - アイテム02の所持数
	/// DESC: Initial possession number of items 02 - 初期所持のアイテム個数02
	pub itemNum_02:u8,

	/// NAME: Number of items 03 - アイテム03の所持数
	/// DESC: Initial possession number of items 03 - 初期所持のアイテム個数03
	pub itemNum_03:u8,

	/// NAME: Item quantity 04 - アイテム個数04
	/// DESC: Initial possession number of items 04 - 初期所持のアイテム個数04
	pub itemNum_04:u8,

	/// NAME: Item quantity 05 - アイテム個数05
	/// DESC: Initial possession number of items 05 - 初期所持のアイテム個数05
	pub itemNum_05:u8,

	/// NAME: Item quantity 06 - アイテム個数06
	/// DESC: Initial possession number of items 06 - 初期所持のアイテム個数06
	pub itemNum_06:u8,

	/// NAME: Item quantity 07 - アイテム個数07
	/// DESC: Initial possession number of items 07 - 初期所持のアイテム個数07
	pub itemNum_07:u8,

	/// NAME: Item quantity 08 - アイテム個数08
	/// DESC: Initial possession number of items 08 - 初期所持のアイテム個数08
	pub itemNum_08:u8,

	/// NAME: Item quantity 09 - アイテム個数09
	/// DESC: Initial possession number of items 09 - 初期所持のアイテム個数09
	pub itemNum_09:u8,

	/// NAME: Item quantity 10 - アイテム個数10
	/// DESC: Initial possession number of items 10 - 初期所持のアイテム個数10
	pub itemNum_10:u8,

	/// NAME: pad - pad
	pub pad5:[u8;5],

	/// NAME: Gesture ID 0 - ジェスチャーID0
	/// DESC: Gesture 0th (0 if you want to play EzState gesture 0) - ジェスチャー0番目(EzStateのジェスチャー0を再生したいなら0)
	pub gestureId0:i8,

	/// NAME: Gesture ID1 - ジェスチャーID1
	/// DESC: Gesture 1st (0 if you want to play EzState gesture 0) - ジェスチャー1番目(EzStateのジェスチャー0を再生したいなら0)
	pub gestureId1:i8,

	/// NAME: Gesture ID 2 - ジェスチャーID2
	/// DESC: Gesture 2nd (0 if you want to play EzState's gesture 0) - ジェスチャー2番目(EzStateのジェスチャー0を再生したいなら0)
	pub gestureId2:i8,

	/// NAME: Gesture ID 3 - ジェスチャーID3
	/// DESC: Gesture 3rd (0 if you want to play EzState gesture 0) - ジェスチャー3番目(EzStateのジェスチャー0を再生したいなら0)
	pub gestureId3:i8,

	/// NAME: Gesture ID 4 - ジェスチャーID4
	/// DESC: Gesture 4th (0 if you want to play EzState's gesture 0) - ジェスチャー4番目(EzStateのジェスチャー0を再生したいなら0)
	pub gestureId4:i8,

	/// NAME: Gesture ID 5 - ジェスチャーID5
	/// DESC: Gesture 5th (0 if you want to play EzState gesture 0) - ジェスチャー5番目(EzStateのジェスチャー0を再生したいなら0)
	pub gestureId5:i8,

	/// NAME: Gesture ID 6 - ジェスチャーID6
	/// DESC: Gesture 6th (0 if you want to play EzState gesture 0) - ジェスチャー6番目(EzStateのジェスチャー0を再生したいなら0)
	pub gestureId6:i8,

	/// NAME: NPC type of NPC player - NPCプレイヤーのNPCタイプ
	/// DESC: NPC type used by NPC players. Not used by normal players. - NPCプレイヤーで使用するNPCタイプ。通常プレイヤーでは使用しません。
	pub npcPlayerType:u8,

	/// NAME: NPC player drawing type - NPCプレイヤーの描画タイプ
	/// DESC: Drawing type used by NPC players. Not used by normal players. - NPCプレイヤーで使用する描画タイプ。通常プレイヤーでは使用しません。
	pub npcPlayerDrawType:i8,

	/// NAME: Gender of NPC players - NPCプレイヤーの性別
	/// DESC: The gender used by NPC players. It is not reflected to normal players. - NPCプレイヤーで使用する性別です。通常プレイヤーには反映しません。
	pub npcPlayerSex:u8,

	/// NAME: Pledge - 誓約
	/// DESC: Pledge type (None: 0) - 誓約タイプ(なし：0)
	pub Bitfield1:u8,

	/// NAME: pad - pad
	pub pad6:[u8;2],

	/// NAME: Right-handed weapon slot 1 equipment type - 右手武器スロット1装備タイプ
	/// DESC: Right-handed weapon slot 1 parameter reference - 右手武器スロット１のパラメータ参照先
	pub wepParamType_Right1:u8,

	/// NAME: Right-handed weapon slot 2 equipment type - 右手武器スロット2装備タイプ
	/// DESC: Right-handed weapon slot 2 parameter reference - 右手武器スロット２のパラメータ参照先
	pub wepParamType_Right2:u8,

	/// NAME: Right-handed weapon slot 3 equipment type - 右手武器スロット3装備タイプ
	/// DESC: Right-handed weapon slot 3 parameter reference - 右手武器スロット３のパラメータ参照先
	pub wepParamType_Right3:u8,

	/// NAME: Left-handed weapon slot 1 equipment type - 左手武器スロット1装備タイプ
	/// DESC: Parameter reference for left-handed weapon slot 1 - 左手武器スロット１のパラメータ参照先
	pub wepParamType_Left1:u8,

	/// NAME: Left-handed weapon slot 2 equipment type - 左手武器スロット2装備タイプ
	/// DESC: Parameter reference for left-handed weapon slot 2 - 左手武器スロット２のパラメータ参照先
	pub wepParamType_Left2:u8,

	/// NAME: Left-handed weapon slot 3 equipment type - 左手武器スロット3装備タイプ
	/// DESC: Parameter reference for left-handed weapon slot 3 - 左手武器スロット３のパラメータ参照先
	pub wepParamType_Left3:u8,

	/// NAME: pad - pad
	pub pad2:[u8;26],

	/// NAME: Right-handed weapon slot 3 - 右手武器スロット3
	/// DESC: Weapon ID of equipment parameter (right hand slot 3) - 装備品パラメータの武器ＩＤ(右手スロット３)
	pub equip_Subwep_Right3:i32,

	/// NAME: Left-handed weapon slot 3 - 左手武器スロット3
	/// DESC: Weapon ID of equipment parameter (left hand slot 3) - 装備品パラメータの武器ＩＤ(左手スロット３)
	pub equip_Subwep_Left3:i32,

	/// NAME: pad - pad
	pub pad3:[u8;4],

	/// NAME: Second item 01 - 第二アイテム01
	/// DESC: Item ID 01 of the second shortcut initial possession - 第二ショートカット初期所持のアイテムID01
	pub secondaryItem_01:i32,

	/// NAME: Second item 02 - 第二アイテム02
	/// DESC: Item ID 02 of the second shortcut initial possession - 第二ショートカット初期所持のアイテムID02
	pub secondaryItem_02:i32,

	/// NAME: Second item 03 - 第二アイテム03
	/// DESC: Item ID 03 of the second shortcut initial possession - 第二ショートカット初期所持のアイテムID03
	pub secondaryItem_03:i32,

	/// NAME: Second item 04 - 第二アイテム04
	/// DESC: Item ID 04 of the second shortcut initial possession - 第二ショートカット初期所持のアイテムID04
	pub secondaryItem_04:i32,

	/// NAME: Second item 05 - 第二アイテム05
	/// DESC: Item ID 05 of the second shortcut initial possession - 第二ショートカット初期所持のアイテムID05
	pub secondaryItem_05:i32,

	/// NAME: Second item 06 - 第二アイテム06
	/// DESC: Item ID 06 of the second shortcut initial possession - 第二ショートカット初期所持のアイテムID06
	pub secondaryItem_06:i32,

	/// NAME: Number of possessions of the second item 01 - 第二アイテム01の所持数
	/// DESC: 2nd Shortcut Initial possession number of items 01 - 第二ショートカット初期所持のアイテム個数01
	pub secondaryItemNum_01:u8,

	/// NAME: Number of possessions of the second item 02 - 第二アイテム02の所持数
	/// DESC: 2nd Shortcut Initial possession number of items 02 - 第二ショートカット初期所持のアイテム個数02
	pub secondaryItemNum_02:u8,

	/// NAME: Number of possessions of the second item 03 - 第二アイテム03の所持数
	/// DESC: 2nd Shortcut Initial possession number of items 03 - 第二ショートカット初期所持のアイテム個数03
	pub secondaryItemNum_03:u8,

	/// NAME: Number of possessions of the second item 04 - 第二アイテム04の所持数
	/// DESC: 2nd Shortcut Initial possession number of items 04 - 第二ショートカット初期所持のアイテム個数04
	pub secondaryItemNum_04:u8,

	/// NAME: Number of possessions of the second item 05 - 第二アイテム05の所持数
	/// DESC: 2nd Shortcut Initial possession number of items 05 - 第二ショートカット初期所持のアイテム個数05
	pub secondaryItemNum_05:u8,

	/// NAME: Number of possessions of the second item 06 - 第二アイテム06の所持数
	/// DESC: 2nd Shortcut Initial possession number of items 06 - 第二ショートカット初期所持のアイテム個数06
	pub secondaryItemNum_06:u8,

	/// NAME: HP Est Bottle Possession Limit - HPエスト瓶 所持限界数
	/// DESC: HP Est Bottle Possession Limit - HPエスト瓶 所持限界数
	pub HpEstMax:i8,

	/// NAME: MP Est Bottle Possession Limit - MPエスト瓶 所持限界数
	/// DESC: MP Est Bottle Possession Limit - MPエスト瓶 所持限界数
	pub MpEstMax:i8,

	/// NAME: pad - pad
	pub pad7:[u8;5],

	/// NAME: Voice type - 声タイプ
	/// DESC: Voice type - 声タイプ
	pub voiceType:u8,

	/// NAME: Reserved area - 予約領域
	/// DESC: Reserved area - 予約領域
	pub reserve:[u8;6],
}

impl CHARACTER_INIT_PARAM {
	/// Pledge type (None: 0) - 誓約タイプ(なし：0)
	/// Bitfield1
	pub fn get_vowType(&self) -> u8 {
		&self.Bitfield1 & 0x78
	}

	/// Bitfield1 MAX: 15
	pub fn set_vowType(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 3) & 0x78;
			let newVal = &self.Bitfield1 & 0x87 | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0x87
		}
	}	/// Is it a send / receive target (for copy NPCs)? - 送受信対象か（コピーNPC用）
	/// Bitfield1
	pub fn get_isSyncTarget(&self) -> bool {
		&self.Bitfield1 & 0x10 != 0
	}

	/// Bitfield1
	pub fn set_isSyncTarget(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x10
		} else {
			self.Bitfield1 &= 0xEF
		}
	}
	/// 
	/// Bitfield1
	pub fn get_pad(&self) -> u8 {
		&self.Bitfield1 & 0xE0
	}

	/// Bitfield1 MAX: 7
	pub fn set_pad(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 5) & 0xE0;
			let newVal = &self.Bitfield1 & 0x1F | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0x1F
		}
	}
}
