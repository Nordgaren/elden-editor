/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct SP_EFFECT_VFX_PARAM_ST {

	/// NAME: In effect SfxID - 効果中SfxID
	/// DESC: In effect SfxID (-1: invalid) - 効果中SfxID(-1：無効)
	pub midstSfxId:i32,

	/// NAME: In effect SeID - 効果中SeID
	/// DESC: In effect SeID (-1: invalid) - 効果中SeID(-1：無効)
	pub midstSeId:i32,

	/// NAME: SfxID at the time of activation - 発動時SfxID
	/// DESC: SfxID at activation (-1: invalid) - 発動時SfxID(-1：無効)
	pub initSfxId:i32,

	/// NAME: SeID at the time of activation - 発動時SeID
	/// DESC: When activated SeID (-1: invalid) - 発動時SeID(-1：無効)
	pub initSeId:i32,

	/// NAME: SfxID at the time of release - 解除時SfxID
	/// DESC: SfxID at the time of cancellation (-1: invalid) - 解除時SfxID(-1：無効)
	pub finishSfxId:i32,

	/// NAME: SeID at the time of release - 解除時SeID
	/// DESC: SeID at the time of release (-1: invalid) - 解除時SeID(-1：無効)
	pub finishSeId:i32,

	/// NAME: Hidden start distance [m] - 姿隠し開始距離[m]
	/// DESC: It is the camouflage start distance - カムフラージュ開始距離です
	pub camouflageBeginDist:f32,

	/// NAME: Hidden end distance [m] - 姿隠し終了距離[m]
	/// DESC: It is the camouflage end distance - カムフラージュ終了距離です
	pub camouflageEndDist:f32,

	/// NAME: Makeover Armor ID - 変身防具ID
	/// DESC: Makeover Armor ID (-1: None) - 変身防具ID(-1：なし)
	pub transformProtectorId:i32,

	/// NAME: Damipoli ID in effect - 効果中ダミポリID
	/// DESC: In effect Damipoli ID (-1: Root) - 効果中ダミポリID(-1：ルート)
	pub midstDmyId:i16,

	/// NAME: Damipoli ID at the time of activation - 発動時ダミポリID
	/// DESC: Damipoli ID at the time of activation (-1: root) - 発動時ダミポリID(-1：ルート)
	pub initDmyId:i16,

	/// NAME: Damipoli ID at the time of cancellation - 解除時ダミポリID
	/// DESC: Damipoli ID at the time of cancellation (-1: root) - 解除時ダミポリID(-1：ルート)
	pub finishDmyId:i16,

	/// NAME: Effect type - エフェクトタイプ
	/// DESC: Effect type - エフェクトタイプ
	pub effectType:u8,

	/// NAME: Soul Param ID for Weapon Enchantment - 武器エンチャント用ソウルパラムID
	/// DESC: Soul Param ID for Weapon Enchantment (-1: None). Change the applied Phantom Param. - 武器エンチャント用ソウルパラムID(-1：なし).適用されるファントムパラムを変更します。
	pub soulParamIdForWepEnchant:u8,

	/// NAME: VFX playback category - VFX再生カテゴリ
	/// DESC: Controls effect playback due to duplicate effects - 重複効果によるエフェクト再生を制御します
	pub playCategory:u8,

	/// NAME: In-category priority - カテゴリ内優先度
	/// DESC: Set the playback priority when the categories match (lower one has priority) - カテゴリ一致した場合の再生優先度を設定(低い方が優先)
	pub playPriority:u8,

	/// NAME: Is there a large effect? - 大型用エフェクトがあるか
	/// DESC: Is there a large effect? - 大型用エフェクトがあるか
	pub Bitfield1:u8,

	/// NAME: Invisible Weapon for Weapon Enchantment? - 武器エンチャント用インビジブルウェポンか
	/// DESC: Invisible Weapon for Weapon Enchantment (0: Weapon Show, 1: Weapon Hide) - 武器エンチャント用インビジブルウェポンか(0:武器表示, 1:武器非表示)
	pub Bitfield2:u8,

	/// NAME: Decal ID1 - デカールID1
	/// DESC: Decal ID 1 (-1: invalid) - デカールID1(-1：無効)
	pub decalId1:i32,

	/// NAME: Decal ID2 - デカールID2
	/// DESC: Decal ID 2 (-1: invalid) - デカールID2(-1：無効)
	pub decalId2:i32,

	/// NAME: Foot effect priority - フットエフェクト優先度
	/// DESC: Foot effect offset priority (lower is higher) - フットエフェクトオフセットの優先度(低いほうが優先)
	pub footEffectPriority:u8,

	/// NAME: Foot effect offset - フットエフェクトオフセット
	/// DESC: Amount offset to foot effect ID when this special effect is applied - この特殊効果がかかっている場合にフットエフェクトIDにオフセットする量
	pub footEffectOffset:u8,

	/// NAME: Sword flash SFX ID offset type - 剣閃SFXIDオフセットタイプ
	/// DESC: The offset value applied to the sword flash SFX ID. Used for enchantment and sword trajectory effects - 剣閃SFXIDにかけるオフセット値です。エンチャントと剣の軌跡エフェクトに使われる
	pub traceSfxIdOffsetType:u8,

	/// NAME: Forced overwriting of player appearance - プレイヤー見た目強制上書き
	/// DESC: A function that can force the appearance of a character to be alive / dead - キャラクターの見た目を強制的に生者/亡者にできる機能
	pub forceDeceasedType:u8,

	/// NAME: Enchantment time root Damipoli ID_0 - エンチャント時根元ダミポリID＿０
	/// DESC: Damipoli ID generated at the base of enchantment - エンチャント時の根元に発生させるダミポリID
	pub enchantStartDmyId_0:i32,

	/// NAME: Damipoli ID_0 at the time of enchantment - エンチャント時剣先ダミポリID＿０
	/// DESC: Damipoli ID generated at the tip of the sword at the time of enchantment. -1 If specified, it will be automatically put out to the point where it is a serial number. - エンチャント時の剣先に発生させるダミポリID。-1指定で自動的に連番になってるところまで出す。
	pub enchantEndDmyId_0:i32,

	/// NAME: Enchantment time root Damipoli ID_1 - エンチャント時根元ダミポリID＿１
	/// DESC: Damipoli ID generated at the base of enchantment - エンチャント時の根元に発生させるダミポリID
	pub enchantStartDmyId_1:i32,

	/// NAME: Damipoli ID_1 at the time of enchantment - エンチャント時剣先ダミポリID＿１
	/// DESC: Damipoli ID generated at the tip of the sword at the time of enchantment. -1 If specified, it will be automatically put out to the point where it is a serial number. - エンチャント時の剣先に発生させるダミポリID。-1指定で自動的に連番になってるところまで出す。
	pub enchantEndDmyId_1:i32,

	/// NAME: Enchantment time root Damipoli ID_2 - エンチャント時根元ダミポリID＿２
	/// DESC: Damipoli ID generated at the base of enchantment - エンチャント時の根元に発生させるダミポリID
	pub enchantStartDmyId_2:i32,

	/// NAME: Enchantment time sword tip Damipoli ID_2 - エンチャント時剣先ダミポリID＿２
	/// DESC: Damipoli ID generated at the tip of the sword at the time of enchantment. -1 If specified, it will be automatically put out to the point where it is a serial number. - エンチャント時の剣先に発生させるダミポリID。-1指定で自動的に連番になってるところまで出す。
	pub enchantEndDmyId_2:i32,

	/// NAME: Enchantment time root Damipoli ID_3 - エンチャント時根元ダミポリID＿３
	/// DESC: Damipoli ID generated at the base of enchantment - エンチャント時の根元に発生させるダミポリID
	pub enchantStartDmyId_3:i32,

	/// NAME: Damipoli ID_3 at the time of enchantment - エンチャント時剣先ダミポリID＿３
	/// DESC: Damipoli ID generated at the tip of the sword at the time of enchantment. -1 If specified, it will be automatically put out to the point where it is a serial number. - エンチャント時の剣先に発生させるダミポリID。-1指定で自動的に連番になってるところまで出す。
	pub enchantEndDmyId_3:i32,

	/// NAME: Enchantment time root Damipoli ID_4 - エンチャント時根元ダミポリID＿４
	/// DESC: Damipoli ID generated at the base of enchantment - エンチャント時の根元に発生させるダミポリID
	pub enchantStartDmyId_4:i32,

	/// NAME: Damipoli ID_4 at the time of enchantment - エンチャント時剣先ダミポリID＿４
	/// DESC: Damipoli ID generated at the tip of the sword at the time of enchantment. -1 If specified, it will be automatically put out to the point where it is a serial number. - エンチャント時の剣先に発生させるダミポリID。-1指定で自動的に連番になってるところまで出す。
	pub enchantEndDmyId_4:i32,

	/// NAME: Enchantment time root Damipoli ID_5 - エンチャント時根元ダミポリID＿５
	/// DESC: Damipoli ID generated at the base of enchantment - エンチャント時の根元に発生させるダミポリID
	pub enchantStartDmyId_5:i32,

	/// NAME: Damipoli ID_5 at the time of enchantment - エンチャント時剣先ダミポリID＿５
	/// DESC: Damipoli ID generated at the tip of the sword at the time of enchantment. -1 If specified, it will be automatically put out to the point where it is a serial number. - エンチャント時の剣先に発生させるダミポリID。-1指定で自動的に連番になってるところまで出す。
	pub enchantEndDmyId_5:i32,

	/// NAME: Enchantment time root Damipoli ID_6 - エンチャント時根元ダミポリID＿６
	/// DESC: Damipoli ID generated at the base of enchantment - エンチャント時の根元に発生させるダミポリID
	pub enchantStartDmyId_6:i32,

	/// NAME: Damipoli ID_6 at the time of enchantment - エンチャント時剣先ダミポリID＿６
	/// DESC: Damipoli ID generated at the tip of the sword at the time of enchantment. -1 If specified, it will be automatically put out to the point where it is a serial number. - エンチャント時の剣先に発生させるダミポリID。-1指定で自動的に連番になってるところまで出す。
	pub enchantEndDmyId_6:i32,

	/// NAME: Enchantment time root Damipoli ID_7 - エンチャント時根元ダミポリID＿７
	/// DESC: Damipoli ID generated at the base of enchantment - エンチャント時の根元に発生させるダミポリID
	pub enchantStartDmyId_7:i32,

	/// NAME: Damipoli ID_7 at the time of enchantment - エンチャント時剣先ダミポリID＿７
	/// DESC: Damipoli ID generated at the tip of the sword at the time of enchantment. -1 If specified, it will be automatically put out to the point where it is a serial number. - エンチャント時の剣先に発生させるダミポリID。-1指定で自動的に連番になってるところまで出す。
	pub enchantEndDmyId_7:i32,

	/// NAME: SfxID offset type - SfxIDオフセットタイプ
	/// DESC: SfxID offset type - SfxIDオフセットタイプ
	pub SfxIdOffsetType:u8,

	/// NAME: Forced specification of phantom parameters - ファントムパラメータ強制指定
	/// DESC: Forced overwrite type of phantom parameters - ファントムパラメータの強制上書きタイプ
	pub phantomParamOverwriteType:u8,

	/// NAME: Minimum α value when hiding [%] - 姿隠し時最小α値[%]
	/// DESC: Minimum α value when hiding [%] - 姿隠し時最小α値[%]
	pub camouflageMinAlpha:u8,

	/// NAME: Water wet effect - 水濡れ効果
	/// DESC: Generate a wet expression by referring to the wet parameter - ウェットパラメータを参照して水濡れ表現を発生させる
	pub wetAspectType:u8,

	/// NAME: Phantom parameter overwrite ID - ファントムパラメータ上書きID
	/// DESC: Forced Id of phantom parameter - ファントムパラメータの強制Id
	pub phantomParamOverwriteId:i32,

	/// NAME: Material extension parameter ID - マテリアル拡張パラメータID
	/// DESC: ID0-99 are GS reservation IDs. If ID100 or later is specified, the material extension parameter is referenced (-1: invalid value). - ID0～99はGSの予約IDです。ID100以降を指定した場合、マテリアル拡張パラメータを参照します（-1：無効値）
	pub materialParamId:i32,

	/// NAME: Initial values of material parameters - マテリアルパラメータの初期値
	/// DESC: The value at the start of the fade of the material parameter. The target is specified by the material parameter ID. If the material parameter ID is -1, do nothing - マテリアルパラメータのフェード開始時の値。対象はマテリアルパラメータIDで指定する。マテリアルパラメータIDが -1 なら何もしない
	pub materialParamInitValue:f32,

	/// NAME: Material parameter target value - マテリアルパラメータの目標値
	/// DESC: The value at the end of the fade of the material parameter. The target is specified by the material parameter ID. If the material parameter ID is -1, do nothing - マテリアルパラメータのフェード終了時の値。対象はマテリアルパラメータIDで指定する。マテリアルパラメータIDが -1 なら何もしない
	pub materialParamTargetValue:f32,

	/// NAME: Fade time of material parameter values - マテリアルパラメータ値のフェード時間
	/// DESC: Fade time for material parameter values. Gradually reach the target value over this time. If the material parameter ID is -1, do nothing - マテリアルパラメータ値のフェード時間。この時間かけて徐々に目標値へ行く。マテリアルパラメータIDが -1 なら何もしない
	pub materialParamFadeTime:f32,

	/// NAME: Foot Decal Material Offset Forced Overwrite ID - フットデカール材質オフセット強制上書きID 
	/// DESC: Forcibly rewrite the floor material ID offset of the foot decal (-1 unused) - フットデカールの床材質IDオフセットを強制的に書き換える（-1未使用）
	pub footDecalMaterialOffsetOverwriteId:i16,

	/// NAME: Padding - パディング
	/// DESC: Padding - パディング
	pub pad:[u8;14],
}

impl SP_EFFECT_VFX_PARAM_ST {
	/// Is there a large effect? - 大型用エフェクトがあるか
	/// Bitfield1
	pub fn get_existEffectForLarge(&self) -> bool {
		&self.Bitfield1 & 0x1 != 0
	}

	/// Bitfield1
	pub fn set_existEffectForLarge(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x1
		} else {
			self.Bitfield1 &= 0xFE
		}
	}
	/// Is there an effect for the soul body? - ソウル体用エフェクトがあるか
	/// Bitfield1
	pub fn get_existEffectForSoul(&self) -> bool {
		&self.Bitfield1 & 0x2 != 0
	}

	/// Bitfield1
	pub fn set_existEffectForSoul(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x2
		} else {
			self.Bitfield1 &= 0xFD
		}
	}
	/// Whether to hide the effect when hiding - 姿隠し時にエフェクトを非表示にするか
	/// Bitfield1
	pub fn get_effectInvisibleAtCamouflage(&self) -> bool {
		&self.Bitfield1 & 0x4 != 0
	}

	/// Bitfield1
	pub fn set_effectInvisibleAtCamouflage(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x4
		} else {
			self.Bitfield1 &= 0xFB
		}
	}
	/// Do you hide - 姿隠しするか
	/// Bitfield1
	pub fn get_useCamouflage(&self) -> bool {
		&self.Bitfield1 & 0x8 != 0
	}

	/// Bitfield1
	pub fn set_useCamouflage(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x8
		} else {
			self.Bitfield1 &= 0xF7
		}
	}
	/// Is it hidden even by allies when hiding? - 姿隠し時に味方でも非表示か
	/// Bitfield1
	pub fn get_invisibleAtFriendCamouflage(&self) -> bool {
		&self.Bitfield1 & 0x10 != 0
	}

	/// Bitfield1
	pub fn set_invisibleAtFriendCamouflage(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x10
		} else {
			self.Bitfield1 &= 0xEF
		}
	}
	/// Do you want to turn off the foot effect when hiding? - 姿隠し時にフットエフェクトを消すか
	/// Bitfield1
	pub fn get_isHideFootEffect_forCamouflage(&self) -> bool {
		&self.Bitfield1 & 0x20 != 0
	}

	/// Bitfield1
	pub fn set_isHideFootEffect_forCamouflage(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x20
		} else {
			self.Bitfield1 &= 0xDF
		}
	}
	/// Only translucent appearance - 半透明の姿隠しか
	/// Bitfield1
	pub fn get_halfCamouflage(&self) -> bool {
		&self.Bitfield1 & 0x40 != 0
	}

	/// Bitfield1
	pub fn set_halfCamouflage(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x40
		} else {
			self.Bitfield1 &= 0xBF
		}
	}
	/// Is the transformation armor ID for the whole body? - 変身防具IDが全身用か
	/// Bitfield1
	pub fn get_isFullBodyTransformProtectorId(&self) -> bool {
		&self.Bitfield1 & 0x80 != 0
	}

	/// Bitfield1
	pub fn set_isFullBodyTransformProtectorId(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x80
		} else {
			self.Bitfield1 &= 0x7F
		}
	}
	/// Invisible Weapon for Weapon Enchantment (0: Weapon Show, 1: Weapon Hide) - 武器エンチャント用インビジブルウェポンか(0:武器表示, 1:武器非表示)
	/// Bitfield2
	pub fn get_isInvisibleWeapon(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_isInvisibleWeapon(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// Is it silence? (0: No, 1: Yes) - サイレンスか(0:ちがう, 1:そう)
	/// Bitfield2
	pub fn get_isSilence(&self) -> bool {
		&self.Bitfield2 & 0x2 != 0
	}

	/// Bitfield2
	pub fn set_isSilence(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x2
		} else {
			self.Bitfield2 &= 0xFD
		}
	}
	/// Do you use whole body Damipoli for equipping SFX during effect? Play SFX from torso: 190, head: 191, hands: 192, legs: 193 at 1 - 効果中SFXを装備用全身ダミポリを使用するか。1の時に胴:190,頭:191,手:192,脚:193からSFXを再生する
	/// Bitfield2
	pub fn get_isMidstFullbody(&self) -> bool {
		&self.Bitfield2 & 0x4 != 0
	}

	/// Bitfield2
	pub fn set_isMidstFullbody(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x4
		} else {
			self.Bitfield2 &= 0xFB
		}
	}
	/// Do you use the whole body Damipoli for equipping SFX during activation? Play SFX from torso: 190, head: 191, hands: 192, legs: 193 at 1 - 発動中SFXを装備用全身ダミポリを使用するか。1の時に胴:190,頭:191,手:192,脚:193からSFXを再生する
	/// Bitfield2
	pub fn get_isInitFullbody(&self) -> bool {
		&self.Bitfield2 & 0x8 != 0
	}

	/// Bitfield2
	pub fn set_isInitFullbody(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x8
		} else {
			self.Bitfield2 &= 0xF7
		}
	}
	/// Do you use the whole body Damipoli for equipping SFX at the time of release? Play SFX from torso: 190, head: 191, hands: 192, legs: 193 at 1 - 解除時SFXを装備用全身ダミポリを使用するか。1の時に胴:190,頭:191,手:192,脚:193からSFXを再生する
	/// Bitfield2
	pub fn get_isFinishFullbody(&self) -> bool {
		&self.Bitfield2 & 0x10 != 0
	}

	/// Bitfield2
	pub fn set_isFinishFullbody(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x10
		} else {
			self.Bitfield2 &= 0xEF
		}
	}
	/// If ○, VFX will be displayed even when the corpse is dead. - ○の場合、死体時でもVFXが表示されるようになります。
	/// Bitfield2
	pub fn get_isVisibleDeadChr(&self) -> bool {
		&self.Bitfield2 & 0x20 != 0
	}

	/// Bitfield2
	pub fn set_isVisibleDeadChr(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x20
		} else {
			self.Bitfield2 &= 0xDF
		}
	}
	/// Whether to offset the SfxId according to the "enchantment Sfx size" of the weapon para - 武器パラの「エンチャントSfxサイズ」に従ってSfxIdをオフセットするか
	/// Bitfield2
	pub fn get_isUseOffsetEnchantSfxSize(&self) -> bool {
		&self.Bitfield2 & 0x40 != 0
	}

	/// Bitfield2
	pub fn set_isUseOffsetEnchantSfxSize(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x40
		} else {
			self.Bitfield2 &= 0xBF
		}
	}
	/// Padding - パディング
	/// Bitfield2
	pub fn get_pad_1(&self) -> bool {
		&self.Bitfield2 & 0x80 != 0
	}

	/// Bitfield2
	pub fn set_pad_1(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x80
		} else {
			self.Bitfield2 &= 0x7F
		}
	}

}
