/* This file was automatically generated from XML paramdefs. */
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct EQUIP_PARAM_GOODS_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Call ID 0 (default) - 呼び出しID0(デフォルト)
	/// DESC: ID0 called from the item (default) - アイテムから呼び出されるID0(デフォルト)
	pub refId_default:i32,

	/// NAME: SFX variation ID - SFXバリエーションID
	/// DESC: Specify SFX variation (used to identify SFX in combination with TimeActEditor ID) - ＳＦＸのバリエーションを指定（TimeActEditorのＩＤと組み合わせて、ＳＦＸを特定するのに使用する）
	pub sfxVariationId:i32,

	/// NAME: Weight [kg] - 重量[kg]
	/// DESC: Weight [kg] - 重量[kg]
	pub weight:f32,

	/// NAME: Basic price - 基本価格
	/// DESC: Basic price - 基本価格
	pub basicPrice:i32,

	/// NAME: Sale price - 売却価格
	/// DESC: Selling price - 販売価格
	pub sellValue:i32,

	/// NAME: Action ID - 行動ID
	/// DESC: Set the effect that occurs when using a tool - 道具を使ったときに発生する効果を設定します
	pub behaviorId:i32,

	/// NAME: Replacement item ID - 差し替えアイテムID
	/// DESC: Item ID when replacing - 差し替えるときのアイテムID
	pub replaceItemId:i32,

	/// NAME: Sort ID - ソートID
	/// DESC: Sort ID (-1: Do not collect) - ソートID(-1:集めない)
	pub sortId:i32,

	/// NAME: Display replacement destination item ID - 表示差し替え先アイテムID
	/// DESC: Conditionally replace the display with another tool ID. The tool ID on the game data side does not change - 条件付きで表示を別の道具IDに差し替える。ゲームデータ側の道具IDは変わらない
	pub appearanceReplaceItemId:i32,

	/// NAME: YES / NO message ID - YES/NOメッセージID
	/// DESC: YesNo Message ID used when displaying the dialog - YesNoダイアログ表示時に使用するメッセージID
	pub yesNoDialogMessageId:i32,

	/// NAME: Usable condition_state change type - 使用可能条件_状態変化タイプ
	/// DESC: Allow to use only when the special effect of the set state change type is applied - 設定した状態変化タイプの特殊効果が掛かったときだけ、使用許可する
	pub useEnableSpEffectType:u16,

	/// NAME: Vase group ID - 壺グループID
	/// DESC: "Consumable items" for which 0 or more is set for the pot group ID can be possessed by the number of "pots" with the same pot group ID. - 壺グループIDに0以上が設定されている「消費アイテム」は同じ壺グループIDの「壺」の個数だけ所持可能
	pub potGroupId:i8,

	/// NAME: PAD - PAD
	/// DESC: Old (magic ID linked to a scroll) - 旧(巻物と紐づいた魔法ID)
	pub pad:[u8;1],

	/// NAME: Icon ID - アイコンID
	/// DESC: Menu icon ID - メニュー用アイコンID
	pub iconId:u16,

	/// NAME: Model ID - モデルID
	/// DESC: Model ID - モデルID
	pub modelId:u16,

	/// NAME: Shop level - ショップレベル
	/// DESC: Level that can be sold at the store - お店で販売できるレベル
	pub shopLv:i16,

	/// NAME: Comptrophy SEQ number - コンプトロフィーSEQ番号
	/// DESC: SEQ number of complete trophy - コンプリート系トロフィのSEQ番号
	pub compTrophySedId:i16,

	/// NAME: Trophy SEQ number - トロフィーSEQ番号
	/// DESC: Trophy SEQ number - トロフィーのSEQ番号
	pub trophySeqId:i16,

	/// NAME: Maximum number of possessions - 最大所持数
	/// DESC: Maximum number of possessions - 最大所持数
	pub maxNum:i16,

	/// NAME: Consumer nature - 消費人間性
	/// DESC: Consumer nature - 消費人間性
	pub consumeHeroPoint:u8,

	/// NAME: Skill over start value - 技量オーバー開始値
	/// DESC: Skill over start value - 技量オーバー開始値
	pub overDexterity:u8,

	/// NAME: Tool type - 道具のタイプ
	/// DESC: Types of tools - 道具の種類
	pub goodsType:u8,

	/// NAME: ID category - IDカテゴリ
	/// DESC: ↓ ID category [Attack, Projectile, Special] - ↓のIDのカテゴリ[攻撃、飛び道具、特殊]
	pub refCategory:u8,

	/// NAME: Special effects category - 特殊効果カテゴリ
	/// DESC: Since there are effects (enchantment weapons, etc.) whose parameters fluctuate depending on skills, magic, items, etc., set each action so that the determined effect can correspond to the effect such as "power up only weapon attack". Set "None" for items that do not need to be set, such as varistor. - スキルや、魔法、アイテムなどで、パラメータが変動する効果（エンチャントウェポンなど）があるので、│定した効果が、「武器攻撃のみをパワーアップする」といった効果に対応できるように行動ごとに設定するバリスタなど、設定の必要のないものは「なし」を設定する
	pub spEffectCategory:u8,

	/// NAME: pad - pad
	pub pad3:[u8;1],

	/// NAME: Animation when using tools - 道具使用時アニメ
	/// DESC: Set the animation to play when using the tool - 道具を使ったときに再生するアニメを設定します
	pub goodsUseAnim:u8,

	/// NAME: Do you want to open the menu - メニュー開くか
	/// DESC: Menu type that opens when using an item - アイテム使用時に開くメニュータイプ
	pub opmeMenuType:u8,

	/// NAME: Prohibition conditions_Special effects category - 使用禁止条件_特殊効果カテゴリ
	/// DESC: Specified to control whether it can be used by the special effect applied - かかっている特殊効果によって使用可能かを制御する為に指定
	pub useLimitCategory:u8,

	/// NAME: Replacement category - 差し替えカテゴリ
	/// DESC: Condition category to add to call ID - 呼び出しIDに加算しる条件カテゴリ
	pub replaceCategory:u8,

	/// NAME: Reserve - リザーブ
	pub reserve4:[u8;2],

	/// NAME: Can be used alive - 生存使用可
	/// DESC: Is it possible to use a surviving player? - 生存プレイヤー使用可能か
	pub Bitfield2:u8,

	/// NAME: Is it automatically equipped? - 自動装備するか？
	/// DESC: Will it be equipped automatically when picked up? - 拾った時に自動で装備するか？
	pub Bitfield3:u8,

	/// NAME: Is it a replenished item? - 補充済みアイテムか
	/// DESC: Used to determine replenished items - 補充済みアイテムを判別するのに使用します
	pub Bitfield4:u8,

	/// NAME: ID that synchronizes the number increase / decrease - 個数増減を同期させるID
	/// DESC: When the number of items is changed, the items with the same ID will be changed at the same time. 0: Not synchronized - アイテムの個数が変更された際に、同じIDを設定したアイテムも一緒に変更を行います。 0：同期しない
	pub syncNumVaryId:u8,

	/// NAME: Call ID1 - 呼び出しID1
	/// DESC: ID1 called from the item - アイテムから呼び出されるID1
	pub refId_1:i32,

	/// NAME: Reference Virtual Weapon ID - 参照仮想武器ID
	/// DESC: Weapon ID to refer to when using the tool - 道具使用時に参照する武器ID
	pub refVirtualWepId:i32,

	/// NAME: Item lottery ID_for map at the time of vagrant - ベイグラント時アイテム抽選ID_マップ用
	/// DESC: -1: No vagrant 0: No lottery 1 ~: With lottery - -1：ベイグラントなし 0：抽選なし 1～：抽選あり
	pub vagrantItemLotId:i32,

	/// NAME: Vagrant Bonus Enemy Drop Item Lottery ID_Map - ベイグラントボーナス敵ドロップアイテム抽選ID_マップ用
	/// DESC: -1: No drop 0: No lottery 1 ~: With lottery - -1：ドロップなし 0：抽選なし 1～：抽選あり
	pub vagrantBonusEneDropItemLotId:i32,

	/// NAME: Vagrant item Enemy drop item Lottery ID_for map - ベイグラントアイテム敵ドロップアイテム抽選ID_マップ用
	/// DESC: -1: No drop 0: No lottery 1 ~: With lottery - -1：ドロップなし 0：抽選なし 1～：抽選あり
	pub vagrantItemEneDropItemLotId:i32,

	/// NAME: Handheld SFXID - 手持ちSFXID
	/// DESC: SFXID until the effect is activated when you try to use the item - アイテムを使用しようとし、効果が発動するまでのSFXID
	pub castSfxId:i32,

	/// NAME: Activate SFXID - 発動SFXID
	/// DESC: SFX ID when the item is activated - アイテムが発動したときのSFXID
	pub fireSfxId:i32,

	/// NAME: Effect SFXID - 効果SFXID
	/// DESC: After the item is activated, the SFX ID in effect - アイテムが発動後、効果中のSFXID
	pub effectSfxId:i32,

	/// NAME: Can be used while the Great Rune is active - 大ルーン発動中使用可
	/// DESC: Can it be used with a large rune activated? - 大ルーン発動状態で使用可能か
	pub Bitfield5:u8,

	/// NAME: Replenishment type - 補充タイプ
	/// DESC: Replenishment item / Replenishment type when replenishing a replenished item. - 補充アイテム/補充済みアイテムを補充する際の補充タイプ。
	pub suppleType:u8,

	/// NAME: Automatic replenishment type - 自動補充タイプ
	/// DESC: Controls whether or not to automatically replenish and default settings - 自動補充する/しないの可否およびデフォルト設定をコントロールします
	pub autoReplenishType:u8,

	/// NAME: Can you put it on the spot - その場に置けるか
	/// DESC: Can I put the item on the spot? TRUE = can be placed - アイテムをその場に置けるか？TRUE=置ける
	pub Bitfield6:u8,

	/// NAME: Number of warehouses - 倉庫所持数
	/// DESC: Number of warehouses - 倉庫所持数
	pub maxRepositoryNum:i16,

	/// NAME: Sort item type ID - ソートアイテム種別ID
	/// DESC: Sort item type ID. In the sort "Item type order", the same ID is displayed together as the same group. - ソートアイテム種別ID。ソート「アイテム種別順」にて、同じIDは同じグループとしてまとめて表示されます
	pub sortGroupId:u8,

	/// NAME: Can it be used in an attack-prohibited area? - 攻撃禁止区域で使用できるか
	/// DESC: Can it be used in an attack-prohibited area? - 攻撃禁止区域で使用できるか
	pub Bitfield7:u8,

	/// NAME: Selling price - 販売価格
	/// DESC: Selling price - 販売価格
	pub saleValue:i32,

	/// NAME: Rarity - レア度
	/// DESC: Rarity used in item acquisition logs - アイテム取得ログで使うレア度
	pub rarity:u8,

	/// NAME: Is it a buddy item? - バディアイテムか
	/// DESC: Whether there are item usage restrictions for buddy items - バディアイテム用のアイテム使用制限がかかるかどうか
	pub useLimitSummonBuddy:u8,

	/// NAME: Use prohibition condition_state change type - 使用禁止条件_状態変化タイプ
	/// DESC: Specified to control whether it can be used depending on the state change type of the special effect applied - かかっている特殊効果の状態変化タイプによって使用可能かを制御する為に指定
	pub useLimitSpEffectType:u16,

	/// NAME: AI usage judgment ID - AI使用判断ID
	/// DESC: AI usage judgment ID - AI使用判断ID
	pub aiUseJudgeId:i32,

	/// NAME: MP consumption - 消費MP
	/// DESC: MP consumption - 消費MP
	pub consumeMP:i16,

	/// NAME: HP consumed - 消費HP
	/// DESC: HP consumed - 消費HP
	pub consumeHP:i16,

	/// NAME: Strengthening tool ID - 強化先道具ID
	/// DESC: Strengthening tool ID - 強化先道具ID
	pub reinforceGoodsId:i32,

	/// NAME: Material ID at the time of strengthening - 強化時素材ID
	/// DESC: Material ID at the time of strengthening - 強化時素材ID
	pub reinforceMaterialId:i32,

	/// NAME: Enhanced price - 強化価格
	/// DESC: Enhanced price - 強化価格
	pub reinforcePrice:i32,

	/// NAME: Pledge 0 usage level - 誓約0使用レベル
	/// DESC: Pledge 0 usage level - 誓約0使用レベル
	pub useLevel_vowType0:i8,

	/// NAME: Pledge 1 usage level - 誓約1使用レベル
	/// DESC: Pledge 1 usage level - 誓約1使用レベル
	pub useLevel_vowType1:i8,

	/// NAME: Pledge 2 usage level - 誓約2使用レベル
	/// DESC: Pledge 2 usage level - 誓約2使用レベル
	pub useLevel_vowType2:i8,

	/// NAME: Pledge 3 usage level - 誓約3使用レベル
	/// DESC: Pledge 3 usage level - 誓約3使用レベル
	pub useLevel_vowType3:i8,

	/// NAME: Pledge 4 usage level - 誓約4使用レベル
	/// DESC: Pledge 4 usage level - 誓約4使用レベル
	pub useLevel_vowType4:i8,

	/// NAME: Pledge 5 usage level - 誓約5使用レベル
	/// DESC: Pledge 5 usage level - 誓約5使用レベル
	pub useLevel_vowType5:i8,

	/// NAME: Pledge 6 usage level - 誓約6使用レベル
	/// DESC: Pledge 6 usage level - 誓約6使用レベル
	pub useLevel_vowType6:i8,

	/// NAME: Pledge 7 usage level - 誓約7使用レベル
	/// DESC: Pledge 7 usage level - 誓約7使用レベル
	pub useLevel_vowType7:i8,

	/// NAME: Pledge 8 usage level - 誓約8使用レベル
	/// DESC: Pledge 8 usage level - 誓約8使用レベル
	pub useLevel_vowType8:i8,

	/// NAME: Pledge 9 usage level - 誓約9使用レベル
	/// DESC: Pledge 9 usage level - 誓約9使用レベル
	pub useLevel_vowType9:i8,

	/// NAME: Pledge 10 usage level - 誓約10使用レベル
	/// DESC: Pledge 10 usage level - 誓約10使用レベル
	pub useLevel_vowType10:i8,

	/// NAME: Pledge 11 usage level - 誓約11使用レベル
	/// DESC: Pledge 11 usage level - 誓約11使用レベル
	pub useLevel_vowType11:i8,

	/// NAME: Pledge 12 usage level - 誓約12使用レベル
	/// DESC: Pledge 12 usage level - 誓約12使用レベル
	pub useLevel_vowType12:i8,

	/// NAME: Pledge 13 usage level - 誓約13使用レベル
	/// DESC: Pledge 13 usage level - 誓約13使用レベル
	pub useLevel_vowType13:i8,

	/// NAME: Pledge 14 usage level - 誓約14使用レベル
	/// DESC: Pledge 14 usage level - 誓約14使用レベル
	pub useLevel_vowType14:i8,

	/// NAME: Pledge 15 usage level - 誓約15使用レベル
	/// DESC: Pledge 15 usage level - 誓約15使用レベル
	pub useLevel_vowType15:i8,

	/// NAME: Appropriate level of use - 使用適正レベル
	/// DESC: Appropriate level of use - 使用適正レベル
	pub useLevel:u16,

	/// NAME: Reserved area - 予約領域
	/// DESC: Reserved area - 予約領域
	pub reserve5:[u8;2],

	/// NAME: Item acquisition tutorial Judgment flag ID - アイテム入手チュートリアル判定フラグID
	/// DESC: Event flag ID for the tutorial when you first get the item. Flag ON when item is obtained. - 初めてアイテム入手した時のチュートリアル用のイベントフラグID。アイテム入手時にフラグON。
	pub itemGetTutorialFlagId:u32,

	/// NAME: Reserved area - 予約領域
	/// DESC: Reserved area - 予約領域
	pub reserve3:[u8;8],
}

impl EQUIP_PARAM_GOODS_ST {
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
	}	/// Is it possible to use a surviving player? - 生存プレイヤー使用可能か
	/// Bitfield2
	pub fn get_enable_live(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_enable_live(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// Can Gray Ghost be used? - グレイゴースト使用可能か
	/// Bitfield2
	pub fn get_enable_gray(&self) -> bool {
		&self.Bitfield2 & 0x2 != 0
	}

	/// Bitfield2
	pub fn set_enable_gray(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x2
		} else {
			self.Bitfield2 &= 0xFD
		}
	}
	/// Can white ghost be used? - ホワイトゴースト使用可能か
	/// Bitfield2
	pub fn get_enable_white(&self) -> bool {
		&self.Bitfield2 & 0x4 != 0
	}

	/// Bitfield2
	pub fn set_enable_white(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x4
		} else {
			self.Bitfield2 &= 0xFB
		}
	}
	/// Is it possible to do a black ghost? - ブラックゴーストしよう可能か
	/// Bitfield2
	pub fn get_enable_black(&self) -> bool {
		&self.Bitfield2 & 0x8 != 0
	}

	/// Bitfield2
	pub fn set_enable_black(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x8
		} else {
			self.Bitfield2 &= 0xF7
		}
	}
	/// Can it be used during multiplayer? - マルチプレイ中に使用可能か？
	/// Bitfield2
	pub fn get_enable_multi(&self) -> bool {
		&self.Bitfield2 & 0x10 != 0
	}

	/// Bitfield2
	pub fn set_enable_multi(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x10
		} else {
			self.Bitfield2 &= 0xEF
		}
	}
	/// Is it unavailable while offline? - オフライン中に使用不可か？
	/// Bitfield2
	pub fn get_disable_offline(&self) -> bool {
		&self.Bitfield2 & 0x20 != 0
	}

	/// Bitfield2
	pub fn set_disable_offline(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x20
		} else {
			self.Bitfield2 &= 0xDF
		}
	}
	/// Whether it can be equipped - 装備できるかどうか
	/// Bitfield2
	pub fn get_isEquip(&self) -> bool {
		&self.Bitfield2 & 0x40 != 0
	}

	/// Bitfield2
	pub fn set_isEquip(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x40
		} else {
			self.Bitfield2 &= 0xBF
		}
	}
	/// Will it be consumed when used (whether the number of possessions will decrease) - 使用時に消耗するか(所持数が減るか)
	/// Bitfield2
	pub fn get_isConsume(&self) -> bool {
		&self.Bitfield2 & 0x80 != 0
	}

	/// Bitfield2
	pub fn set_isConsume(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x80
		} else {
			self.Bitfield2 &= 0x7F
		}
	}
	/// Will it be equipped automatically when picked up? - 拾った時に自動で装備するか？
	/// Bitfield3
	pub fn get_isAutoEquip(&self) -> bool {
		&self.Bitfield3 & 0x1 != 0
	}

	/// Bitfield3
	pub fn set_isAutoEquip(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x1
		} else {
			self.Bitfield3 &= 0xFE
		}
	}
	/// Is it a stationary item? - 設置型アイテムか？
	/// Bitfield3
	pub fn get_isEstablishment(&self) -> bool {
		&self.Bitfield3 & 0x2 != 0
	}

	/// Bitfield3
	pub fn set_isEstablishment(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x2
		} else {
			self.Bitfield3 &= 0xFD
		}
	}
	/// Is it an item that you can only have one? - 1個しか持てないアイテムか
	/// Bitfield3
	pub fn get_isOnlyOne(&self) -> bool {
		&self.Bitfield3 & 0x4 != 0
	}

	/// Bitfield3
	pub fn set_isOnlyOne(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x4
		} else {
			self.Bitfield3 &= 0xFB
		}
	}
	/// Can you throw away the item? TRUE = thrown away - アイテムを捨てれるか？TRUE=捨てれる
	/// Bitfield3
	pub fn get_isDiscard(&self) -> bool {
		&self.Bitfield3 & 0x8 != 0
	}

	/// Bitfield3
	pub fn set_isDiscard(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x8
		} else {
			self.Bitfield3 &= 0xF7
		}
	}
	/// Can you leave it in the warehouse? - 倉庫に預けれるか
	/// Bitfield3
	pub fn get_isDeposit(&self) -> bool {
		&self.Bitfield3 & 0x10 != 0
	}

	/// Bitfield3
	pub fn set_isDeposit(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x10
		} else {
			self.Bitfield3 &= 0xEF
		}
	}
	/// Can't be used when the right-handed weapon is bare-handed? - 右手武器が素手の場合に使用不可か
	/// Bitfield3
	pub fn get_isDisableHand(&self) -> bool {
		&self.Bitfield3 & 0x20 != 0
	}

	/// Bitfield3
	pub fn set_isDisableHand(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x20
		} else {
			self.Bitfield3 &= 0xDF
		}
	}
	/// Whether to delete at the time of lap - 周回時削除するか
	/// Bitfield3
	pub fn get_isRemoveItem_forGameClear(&self) -> bool {
		&self.Bitfield3 & 0x40 != 0
	}

	/// Bitfield3
	pub fn set_isRemoveItem_forGameClear(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x40
		} else {
			self.Bitfield3 &= 0xBF
		}
	}
	/// Used to determine replenishable items - 補充可能アイテムを判別するのに使用します
	/// Bitfield3
	pub fn get_isSuppleItem(&self) -> bool {
		&self.Bitfield3 & 0x80 != 0
	}

	/// Bitfield3
	pub fn set_isSuppleItem(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x80
		} else {
			self.Bitfield3 &= 0x7F
		}
	}
	/// Used to determine replenished items - 補充済みアイテムを判別するのに使用します
	/// Bitfield4
	pub fn get_isFullSuppleItem(&self) -> bool {
		&self.Bitfield4 & 0x1 != 0
	}

	/// Bitfield4
	pub fn set_isFullSuppleItem(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x1
		} else {
			self.Bitfield4 &= 0xFE
		}
	}
	/// Do you want to enchant your weapon? - 武器にエンチャントするか？
	/// Bitfield4
	pub fn get_isEnhance(&self) -> bool {
		&self.Bitfield4 & 0x2 != 0
	}

	/// Bitfield4
	pub fn set_isEnhance(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x2
		} else {
			self.Bitfield4 &= 0xFD
		}
	}
	/// Is it an item to repair? - 修理するアイテムか？
	/// Bitfield4
	pub fn get_isFixItem(&self) -> bool {
		&self.Bitfield4 & 0x4 != 0
	}

	/// Bitfield4
	pub fn set_isFixItem(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x4
		} else {
			self.Bitfield4 &= 0xFB
		}
	}
	/// Is multi-drop sharing prohibited? - マルチドロップ共有禁止か
	/// Bitfield4
	pub fn get_disableMultiDropShare(&self) -> bool {
		&self.Bitfield4 & 0x8 != 0
	}

	/// Bitfield4
	pub fn set_disableMultiDropShare(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x8
		} else {
			self.Bitfield4 &= 0xF7
		}
	}
	/// Is it prohibited to use in the arena? - 闘技場で使用禁止か
	/// Bitfield4
	pub fn get_disableUseAtColiseum(&self) -> bool {
		&self.Bitfield4 & 0x10 != 0
	}

	/// Bitfield4
	pub fn set_disableUseAtColiseum(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x10
		} else {
			self.Bitfield4 &= 0xEF
		}
	}
	/// Is it prohibited to use outside the arena? - 闘技場以外で使用禁止か
	/// Bitfield4
	pub fn get_disableUseAtOutOfColiseum(&self) -> bool {
		&self.Bitfield4 & 0x20 != 0
	}

	/// Bitfield4
	pub fn set_disableUseAtOutOfColiseum(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x20
		} else {
			self.Bitfield4 &= 0xDF
		}
	}
	/// Is it possible to cancel early? - 早いキャンセル可能か
	/// Bitfield4
	pub fn get_isEnableFastUseItem(&self) -> bool {
		&self.Bitfield4 & 0x40 != 0
	}

	/// Bitfield4
	pub fn set_isEnableFastUseItem(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x40
		} else {
			self.Bitfield4 &= 0xBF
		}
	}
	/// Whether to reflect special effects (such as ability value correction) - （能力値補正など）特殊効果を反映するか
	/// Bitfield4
	pub fn get_isApplySpecialEffect(&self) -> bool {
		&self.Bitfield4 & 0x80 != 0
	}

	/// Bitfield4
	pub fn set_isApplySpecialEffect(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x80
		} else {
			self.Bitfield4 &= 0x7F
		}
	}
	/// Can it be used with a large rune activated? - 大ルーン発動状態で使用可能か
	/// Bitfield5
	pub fn get_enable_ActiveBigRune(&self) -> bool {
		&self.Bitfield5 & 0x1 != 0
	}

	/// Bitfield5
	pub fn set_enable_ActiveBigRune(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x1
		} else {
			self.Bitfield5 &= 0xFE
		}
	}
	/// If the state change type "Warp Prohibition" is applied when TRUE, remove the function to disable the item. - TRUEの時に状態変化タイプの「ワープ禁止」がかかっていればそのアイテムを使用不可にする機能を外す
	/// Bitfield5
	pub fn get_isBonfireWarpItem(&self) -> bool {
		&self.Bitfield5 & 0x2 != 0
	}

	/// Bitfield5
	pub fn set_isBonfireWarpItem(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x2
		} else {
			self.Bitfield5 &= 0xFD
		}
	}
	/// Check here for items available in the ladder - はしご中に使用可能なアイテムはここにチェックを入れます
	/// Bitfield5
	pub fn get_enable_Ladder(&self) -> bool {
		&self.Bitfield5 & 0x4 != 0
	}

	/// Bitfield5
	pub fn set_enable_Ladder(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x4
		} else {
			self.Bitfield5 &= 0xFB
		}
	}
	/// Whether items can be used between session probability and initial synchronization - セッション確率～初期同期の間でアイテムを使用できるかどうか
	/// Bitfield5
	pub fn get_isUseMultiPlayPreparation(&self) -> bool {
		&self.Bitfield5 & 0x8 != 0
	}

	/// Bitfield5
	pub fn set_isUseMultiPlayPreparation(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x8
		} else {
			self.Bitfield5 &= 0xF7
		}
	}
	/// Can it be used together? - まとめて使えるか
	/// Bitfield5
	pub fn get_canMultiUse(&self) -> bool {
		&self.Bitfield5 & 0x10 != 0
	}

	/// Bitfield5
	pub fn set_canMultiUse(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x10
		} else {
			self.Bitfield5 &= 0xEF
		}
	}
	/// Is it a shield enchantment? - 盾エンチャントか
	/// Bitfield5
	pub fn get_isShieldEnchant(&self) -> bool {
		&self.Bitfield5 & 0x20 != 0
	}

	/// Bitfield5
	pub fn set_isShieldEnchant(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x20
		} else {
			self.Bitfield5 &= 0xDF
		}
	}
	/// When this is TRUE, disable the item if the state change type "Warp Prohibition" is applied. - これがTRUEの時に、状態変化タイプの「ワープ禁止」がかかっていればそのアイテムを使用不可にする 
	/// Bitfield5
	pub fn get_isWarpProhibited(&self) -> bool {
		&self.Bitfield5 & 0x40 != 0
	}

	/// Bitfield5
	pub fn set_isWarpProhibited(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x40
		} else {
			self.Bitfield5 &= 0xBF
		}
	}
	/// A flag that allows you to determine if an item is only available when a client disconnect penalty is incurred. - クライアント切断ペナルティが発生しているときのみ使用可能なアイテムかどうかを判断できるようにするためのフラグ
	/// Bitfield5
	pub fn get_isUseMultiPenaltyOnly(&self) -> bool {
		&self.Bitfield5 & 0x80 != 0
	}

	/// Bitfield5
	pub fn set_isUseMultiPenaltyOnly(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x80
		} else {
			self.Bitfield5 &= 0x7F
		}
	}
	/// Can I put the item on the spot? TRUE = can be placed - アイテムをその場に置けるか？TRUE=置ける
	/// Bitfield6
	pub fn get_isDrop(&self) -> bool {
		&self.Bitfield6 & 0x1 != 0
	}

	/// Bitfield6
	pub fn set_isDrop(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x1
		} else {
			self.Bitfield6 &= 0xFE
		}
	}
	/// Whether to display in the item acquisition log when acquiring the item (not entered: ○) - アイテム取得時にアイテム取得ログに表示するか（未入力: ○）
	/// Bitfield6
	pub fn get_showLogCondType(&self) -> bool {
		&self.Bitfield6 & 0x2 != 0
	}

	/// Bitfield6
	pub fn set_showLogCondType(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x2
		} else {
			self.Bitfield6 &= 0xFD
		}
	}
	/// Is it an item that summons a horse? Cannot be used if the horse died or the PC is in a horse-prohibited area - 馬を召喚するアイテムか？馬が死亡、PCが馬禁止エリアに居る場合は使用不可
	/// Bitfield6
	pub fn get_isSummonHorse(&self) -> bool {
		&self.Bitfield6 & 0x4 != 0
	}

	/// Bitfield6
	pub fn set_isSummonHorse(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x4
		} else {
			self.Bitfield6 &= 0xFB
		}
	}
	/// Whether to display it in the item acquisition dialog when acquiring an item (not entered: new only) - アイテム取得時にアイテム取得ダイアログに表示するか（未入力: newのみ）
	/// Bitfield6
	pub fn get_showDialogCondType(&self) -> u8 {
		&self.Bitfield6 & 0x18
	}

	/// Bitfield6 MAX: 3
	pub fn set_showDialogCondType(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 3) & 0x18;
			let newVal = &self.Bitfield6 & 0xE7 | val;
			self.Bitfield6 = newVal
		} else {
			self.Bitfield6 &= 0xE7
		}
	}	/// Is it a Nemuri collection item? - ネムリ収集アイテムか
	/// Bitfield6
	pub fn get_isSleepCollectionItem(&self) -> bool {
		&self.Bitfield6 & 0x20 != 0
	}

	/// Bitfield6
	pub fn set_isSleepCollectionItem(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x20
		} else {
			self.Bitfield6 &= 0xDF
		}
	}
	/// Can it be used while riding? - 騎乗中使用可能か
	/// Bitfield6
	pub fn get_enableRiding(&self) -> bool {
		&self.Bitfield6 & 0x40 != 0
	}

	/// Bitfield6
	pub fn set_enableRiding(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x40
		} else {
			self.Bitfield6 &= 0xBF
		}
	}
	/// Is it prohibited to use while not riding? - 非騎乗中使用禁止か
	/// Bitfield6
	pub fn get_disableRiding(&self) -> bool {
		&self.Bitfield6 & 0x80 != 0
	}

	/// Bitfield6
	pub fn set_disableRiding(&mut self, state: bool) {
		if state {
			self.Bitfield6 |= 0x80
		} else {
			self.Bitfield6 &= 0x7F
		}
	}
	/// Can it be used in an attack-prohibited area? - 攻撃禁止区域で使用できるか
	/// Bitfield7
	pub fn get_isUseNoAttackRegion(&self) -> bool {
		&self.Bitfield7 & 0x1 != 0
	}

	/// Bitfield7
	pub fn set_isUseNoAttackRegion(&mut self, state: bool) {
		if state {
			self.Bitfield7 |= 0x1
		} else {
			self.Bitfield7 &= 0xFE
		}
	}
	/// (Old log icon) - （旧ログ用アイコン）
	/// Bitfield7
	pub fn get_pad1(&self) -> u8 {
		&self.Bitfield7 & 0xFE
	}

	/// Bitfield7 MAX: 127
	pub fn set_pad1(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0xFE;
			let newVal = &self.Bitfield7 & 0x1 | val;
			self.Bitfield7 = newVal
		} else {
			self.Bitfield7 &= 0x1
		}
	}
}
