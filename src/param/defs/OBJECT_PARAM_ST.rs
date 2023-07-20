/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 106
#[repr(C)]
pub struct OBJECT_PARAM_ST {
    /// NAME: HP - HP
    /// DESC: Durability until destruction (-1: Indestructible) - 破壊までの耐久力(-1:破壊不可)
    pub hp: i16,

    /// NAME: Defense power - 防御力
    /// DESC: Attack power below this value is no damage - この値以下の攻撃力はダメージなし
    pub defense: u16,

    /// NAME: Xref texture ID - 外部参照テクスチャID
    /// DESC: mAA / mAA_ ????. Tpf (-1: None) (AA: Area number) - mAA/mAA_????.tpf(-1:なし)(AA:エリア番号)
    pub extRefTexId: i16,

    /// NAME: Material ID - 材質ID
    /// DESC: Material ID. Treated the same as the floor material. When -1, the same behavior as before - マテリアルID。床材質と同じ扱い。-1のときは今までと同じ挙動
    pub materialId: i16,

    /// NAME: Anime destruction ID maximum value - アニメ破壊ID最大値
    /// DESC: What is the animation destruction ID from 0 to what? - アニメ破壊IDが0番から何番までか
    pub animBreakIdMax: u8,

    /// NAME: Does the camera hit? - カメラが当たるか
    /// DESC: Does the camera hit (0: not hit, 1: hit) - カメラが当たるか(0:当たらない, 1:当たる)
    pub Bitfield1: u8,

    /// NAME: Do you want to stop the animation during the poly play? - ポリ劇中アニメを停止するか
    /// DESC: Do you want to stop the animation during the poly play (0: not, 1: do) - ポリ劇中アニメを停止するか(0:しない, 1:する)
    pub Bitfield2: u8,

    /// NAME: Default LOD Param ID - デフォルトLODパラムID
    /// DESC: Default LOD Param ID (-1: None) - デフォルトLODパラムID(-1：なし)
    pub defaultLodParamId: i8,

    /// NAME: SFX ID at the time of destruction - 破壊時SFXID
    /// DESC: SFXID when destroying an object (-1: default (810030)) - オブジェ破壊時のSFXID(-1:デフォルト(810030))
    pub breakSfxId: i32,

    /// NAME: SFX Damipoli ID at the time of destruction - 破壊時SFXダミポリID
    /// DESC: SFX generation position when the object is destroyed Damipoly ID (-1: placement position) - オブジェ破壊時SFXの発生位置ダミポリID(-1：配置位置）
    pub breakSfxCpId: i32,

    /// NAME: Bomb generation at the time of destruction Action parameter ID - 破壊時 弾発生 行動パラメータID
    /// DESC: Action parameter of [bullet] at the time of destruction (-1: does not occur) - 破壊時[弾]の行動パラメータ(-1:発生しない)
    pub breakBulletBehaviorId: i32,

    /// NAME: Damipoli ID - 破壊時 弾発生 ダミポリID
    /// DESC: Location of [bullet] at the time of destruction Damipoli ID (-1: placement position) - 破壊時[弾]の発生位置ダミポリID(-1:配置位置)
    pub breakBulletCpId: i32,

    /// NAME: Fall destruction height (m) - 落下破壊高さ(m)
    /// DESC: Height at which the object breaks when dropped (0: does not break when dropped) - 落下時にオブジェクトが壊れる高さ（0：落下では壊れない)
    pub breakFallHeight: u8,

    /// NAME: Wind effect type (before destruction) - 風影響タイプ(破壊前)
    /// DESC: Wind effect type (before destruction) - 風影響タイプ(破壊前)
    pub windEffectType_0: u8,

    /// NAME: Wind effect type (after destruction) - 風影響タイプ(破壊後)
    /// DESC: Wind effect type (after destruction) - 風影響タイプ(破壊後)
    pub windEffectType_1: u8,

    /// NAME: Camera avoidance setting - カメラ回避設定
    /// DESC: What to do if an object blocks between the camera and player - オブジェクトがカメラ・プレイヤー間を遮蔽した場合の対処方法
    pub camAvoidType: u8,

    /// NAME: Wind coefficient (before destruction) - 風係数(破壊前)
    /// DESC: Wind coefficient (before destruction) - 風係数(破壊前)
    pub windEffectRate_0: f32,

    /// NAME: Wind coefficient (after destruction) - 風係数(破壊後)
    /// DESC: Wind coefficient (after destruction) - 風係数(破壊後)
    pub windEffectRate_1: f32,

    /// NAME: Forced stop time after destruction - 破壊後強制停止時間
    /// DESC: Time from destruction to forced stop of rigid body (do not force stop at 0) - 破壊されてから剛体を強制的に停止するまでの時間（0で強制停止しない）
    pub breakStopTime: f32,

    /// NAME: Burning time (seconds) - 燃焼時間(秒)
    /// DESC: Burning time (seconds) (continues to burn at 0) - 燃焼時間(秒)(0で燃え続ける)
    pub burnTime: f32,

    /// NAME: Combustion fracture judgment progress - 燃焼 破壊判定進行度
    /// DESC: Burnup threshold for switching to the ruptured state - 破壊状態に切り替わる燃焼度の閾値
    pub burnBraekRate: f32,

    /// NAME: Combustion SFXID: 0 - 燃焼 SFXID：0
    /// DESC: SFX ID at the time of combustion: 0 (-1: No SFX) - 燃焼時のSFXID：0 (-1：SFXなし)
    pub burnSfxId: i32,

    /// NAME: Combustion SFXID: 1 - 燃焼 SFXID：1
    /// DESC: SFX ID at the time of combustion: 1 (-1: No SFX) - 燃焼時のSFXID：1 (-1：SFXなし)
    pub burnSfxId_1: i32,

    /// NAME: Combustion SFXID: 2 - 燃焼 SFXID：2
    /// DESC: SFX ID at the time of combustion: 2 (-1: No SFX) - 燃焼時のSFXID：2 (-1：SFXなし)
    pub burnSfxId_2: i32,

    /// NAME: Combustion SFXID: 3 - 燃焼 SFXID：3
    /// DESC: SFX ID at the time of combustion: 3 (-1: No SFX) - 燃焼時のSFXID：3 (-1：SFXなし)
    pub burnSfxId_3: i32,

    /// NAME: Combustion bullet generation Behavior parameter: 0 - 燃焼 弾発生 行動パラメータ：0
    /// DESC: Bullet generation behavior parameter at the time of burning: 0 (-1: does not occur) - 燃焼時の弾発生行動パラメータ：0(-1:発生しない)
    pub burnBulletBehaviorId: i32,

    /// NAME: Combustion bullet generation Behavior parameter: 1 - 燃焼 弾発生 行動パラメータ：1
    /// DESC: Bullet generation behavior parameter at the time of burning: 1 (-1: does not occur) - 燃焼時の弾発生行動パラメータ：1(-1:発生しない)
    pub burnBulletBehaviorId_1: i32,

    /// NAME: Combustion bullet generation Behavior parameter: 2 - 燃焼 弾発生 行動パラメータ：2
    /// DESC: Bullet generation behavior parameter at the time of burning: 2 (-1: does not occur) - 燃焼時の弾発生行動パラメータ：2(-1:発生しない)
    pub burnBulletBehaviorId_2: i32,

    /// NAME: Combustion bullet generation Behavior parameters: 3 - 燃焼 弾発生 行動パラメータ：3
    /// DESC: Bullet generation behavior parameter at the time of burning: 3 (-1: does not occur) - 燃焼時の弾発生行動パラメータ：3(-1:発生しない)
    pub burnBulletBehaviorId_3: i32,

    /// NAME: Combustion bullet generation interval (frame) - 燃焼 弾発生間隔(フレーム)
    /// DESC: Interval (frame) to generate bullets for spreading fire - 延焼用の弾を発生する間隔(フレーム)
    pub burnBulletInterval: u16,

    /// NAME: Navi mesh flag - ナビメッシュフラグ
    /// DESC: Navigation mesh flag set from the object - オブジェから設定されるナビメッシュフラグ
    pub navimeshFlag: u8,

    /// NAME: Collision detection type - 衝突判定タイプ
    /// DESC: Collision detection type - 衝突判定タイプ
    pub collisionType: u8,

    /// NAME: Combustion bullet generation delay time (seconds) - 燃焼 弾発生遅延時間(秒)
    /// DESC: Time to delay the generation of bullets for fire spread (seconds) - 延焼用の弾発生を遅らせる時間(秒)
    pub burnBulletDelayTime: f32,

    /// NAME: Combustion SFX generation delay Start time (seconds): 0 - 燃焼 SFX発生遅延 開始時間(秒)：0
    /// DESC: SFX generation delay time during combustion Randomly determined between start and end times - 燃焼時のSFX発生遅延時間 開始～終了時間の間でランダムに決まる
    pub burnSfxDelayTimeMin: f32,

    /// NAME: Combustion SFX generation delay Start time (seconds): 1 - 燃焼 SFX発生遅延 開始時間(秒)：1
    /// DESC: SFX generation delay time during combustion Randomly determined between start and end times - 燃焼時のSFX発生遅延時間 開始～終了時間の間でランダムに決まる
    pub burnSfxDelayTimeMin_1: f32,

    /// NAME: Combustion SFX generation delay Start time (seconds): 2 - 燃焼 SFX発生遅延 開始時間(秒)：2
    /// DESC: SFX generation delay time during combustion Randomly determined between start and end times - 燃焼時のSFX発生遅延時間 開始～終了時間の間でランダムに決まる
    pub burnSfxDelayTimeMin_2: f32,

    /// NAME: Combustion SFX generation delay Start time (seconds): 3 - 燃焼 SFX発生遅延 開始時間(秒)：3
    /// DESC: SFX generation delay time during combustion Randomly determined between start and end times - 燃焼時のSFX発生遅延時間 開始～終了時間の間でランダムに決まる
    pub burnSfxDelayTimeMin_3: f32,

    /// NAME: Combustion SFX generation delay End time (seconds): 0 - 燃焼 SFX発生遅延 終了時間(秒)：0
    /// DESC: SFX generation delay time during combustion Randomly determined between start and end times - 燃焼時のSFX発生遅延時間 開始～終了時間の間でランダムに決まる
    pub burnSfxDelayTimeMax: f32,

    /// NAME: Combustion SFX generation delay End time (seconds): 1 - 燃焼 SFX発生遅延 終了時間(秒)：1
    /// DESC: SFX generation delay time during combustion Randomly determined between start and end times - 燃焼時のSFX発生遅延時間 開始～終了時間の間でランダムに決まる
    pub burnSfxDelayTimeMax_1: f32,

    /// NAME: Combustion SFX generation delay End time (seconds): 2 - 燃焼 SFX発生遅延 終了時間(秒)：2
    /// DESC: SFX generation delay time during combustion Randomly determined between start and end times - 燃焼時のSFX発生遅延時間 開始～終了時間の間でランダムに決まる
    pub burnSfxDelayTimeMax_2: f32,

    /// NAME: Combustion SFX generation delay End time (seconds): 3 - 燃焼 SFX発生遅延 終了時間(秒)：3
    /// DESC: SFX generation delay time during combustion Randomly determined between start and end times - 燃焼時のSFX発生遅延時間 開始～終了時間の間でランダムに決まる
    pub burnSfxDelayTimeMax_3: f32,

    /// NAME: AI sound ID generated at the time of destruction - 破壊時発生AI音ID
    /// DESC: AI sound ID generated at the time of destruction - 破壊時に発生させるAI音ID
    pub BreakAiSoundID: i32,

    /// NAME: Hidden debris Wait time (seconds) - 破片非表示 待機時間(秒)
    /// DESC: Material ID of debris (-1: Do not hide) - 破片のマテリアルID(-1：非表示処理を行なわない)
    pub FragmentInvisibleWaitTime: f32,

    /// NAME: Debris non-display time (seconds) - 破片非表示 時間(秒)
    /// DESC: Time to hide debris (seconds) - 破片を非表示にさせる時間(秒)
    pub FragmentInvisibleTime: f32,

    /// NAME: Padding - パディング
    /// DESC: Material ID of debris (-1: Do not hide) - 破片のマテリアルID(-1：非表示処理を行なわない)
    pub pad_3: [u8; 16],

    /// NAME: Rigid body collision point distance coefficient [soft] - 剛体 衝突点距離係数 [柔らかい]
    /// DESC: Rigid body soft contact setting Collision point distance coefficient [soft] - 剛体ソフトコンタクト設定 衝突点距離係数 [柔らかい]
    pub RigidPenetrationScale_Soft: f32,

    /// NAME: Rigid body collision point distance coefficient [normal] - 剛体 衝突点距離係数 [通常]
    /// DESC: Rigid soft contact setting Collision point distance coefficient [Normal] - 剛体ソフトコンタクト設定 衝突点距離係数 [通常]
    pub RigidPenetrationScale_Normal: f32,

    /// NAME: Rigid body collision point distance coefficient [hard] - 剛体 衝突点距離係数 [固い]
    /// DESC: Rigid soft contact setting Collision point distance coefficient [hard] - 剛体ソフトコンタクト設定 衝突点距離係数 [固い]
    pub RigidPenetrationScale_Hard: f32,

    /// NAME: SFX ID at the time of terrain contact - 地形接触時のSFXID
    /// DESC: SFX ID at terrain contact (-1: offset by terrain material) - 地形接触時のSFXID(-1:地形のマテリアルによりオフセット)
    pub LandTouchSfxId: i32,

    /// NAME: Do you want to shield the damage? - ダメージを遮蔽するか
    /// DESC: Whether to pass the damage to the other side when receiving damage (0: pass, 1: do not pass) - ダメージを受けたときに、そのダメージを反対側に通さないかどうか　(0:通す, 1:通さない)
    pub Bitfield3: u8,

    /// NAME: Padding - パディング
    pub pad_4: [u8; 1],

    /// NAME: Paint decal target size - ペイントデカールターゲットサイズ
    /// DESC: Paint decal target size (only powers of 0 to 4096 2 allowed) - ペイントデカールターゲットサイズ(0～4096 ２のべき乗のみ許可)
    pub paintDecalTargetTextureSize: u16,

    /// NAME: Life of dynamically generated Obj (seconds) - 動的生成Objの寿命(秒)
    /// DESC: Time until dynamically generated Obj disappears after generation (0: does not disappear) - 動的生成Objが生成後に消滅するまでの時間 (0:消滅しない)
    pub lifeTime_forDC: f32,

    /// NAME: Cross update distance (m) - クロス更新距離(m)
    /// DESC: Distance from the camera to update havokCloth (0: always update) - havokClothの更新を行なうカメラからの距離(0:必ず更新する)
    pub clothUpdateDist: f32,

    /// NAME: SE ID when contacting a player - プレイヤー接触時SE ID
    /// DESC: SE ID to play when touched by a local player operated by you (-1: Do not play) - 自分が操作するローカルプレイヤーが触れた際に再生するSEのID(-1:再生しない)
    pub contactSeId: i32,

    /// NAME: SFX identifier when landing after destruction - 破壊後着地時SFX識別子
    /// DESC: Object material-dependent SFX identifier that regenerates when first landing after being destroyed (-1: does not occur) - 破壊された後、最初に着地した際に再生するオブジェ材質依存SFXの識別子(-1:発生しない)
    pub breakLandingSfxId: i32,

    /// NAME: Waypoint Damipoli ID_0 - ウェイポイントダミポリID_0
    /// DESC: Waypoint Damipoli ID_0 (-1: None) - ウェイポイントダミポリID_0(-1:なし)
    pub waypointDummyPolyId_0: i32,

    /// NAME: Waypoint parameter ID_0 - ウェイポイントパラメータID_0
    /// DESC: Waypoint parameter ID_0 (-1: none) - ウェイポイントパラメータID_0(-1:なし)
    pub waypointParamId_0: i32,

    /// NAME: Sound bank ID - サウンドのバンクID
    /// DESC: Sound bank ID (-1: no bank, other: bank with specified ID) - サウンドのバンクID(-1:バンクなし, それ以外:指定したIDのバンク)
    pub soundBankId: i32,

    /// NAME: Drawing parameter reference ID - 描画パラメータ参照ID
    /// DESC: Reference ID of drawing parameter - 描画パラメータの参照ID
    pub refDrawParamId: i32,

    /// NAME: Automatically generated appearance height offset [m] - 自動生成出現高さオフセット[m]
    /// DESC: Map automatic generation OBJ appearance height offset [m], does it float from where the ray cast hits? - マップ自動生成OBJの出現高さオフセット[m]、レイキャストが当たったところから度ぐらい浮かすか
    pub autoCreateDynamicOffsetHeight: f32,

    /// NAME: Reserve - リザーブ
    /// DESC: Reserve - リザーブ
    pub reserved0: i32,

    /// NAME: Destruction sound SEID - 破壊音SEID
    /// DESC: Destruction sound SEID (9 digits) -1: Generated from objId - 破壊音SEID(9桁) -1：objIdから生成
    pub soundBreakSEId: i32,

    /// NAME: Padding - パディング
    pub pad_5: [u8; 40],
}

impl Paramdef for OBJECT_PARAM_ST {
    const NAME: &'static str = "OBJECT_PARAM_ST";
    const VERSION: u16 = 1;
}
impl OBJECT_PARAM_ST {
    /// Does the camera hit (0: not hit, 1: hit) - カメラが当たるか(0:当たらない, 1:当たる)
    /// Bitfield1
    pub fn get_isCamHit(&self) -> bool {
        &self.Bitfield1 & 0x1 != 0
    }

    /// Bitfield1
    pub fn set_isCamHit(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x1
        } else {
            self.Bitfield1 &= 0xFE
        }
    }
    /// Broken when the player touches (0: no, 1:) - プレイヤが接触したときに壊れ(0:ない, 1:る)
    /// Bitfield1
    pub fn get_isBreakByPlayerCollide(&self) -> bool {
        &self.Bitfield1 & 0x2 != 0
    }

    /// Bitfield1
    pub fn set_isBreakByPlayerCollide(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x2
        } else {
            self.Bitfield1 &= 0xFD
        }
    }
    /// Is it animation destruction (0: physical destruction, 1: animation destruction) - アニメ破壊か(0:物理破壊, 1:アニメ破壊)
    /// Bitfield1
    pub fn get_isAnimBreak(&self) -> bool {
        &self.Bitfield1 & 0x4 != 0
    }

    /// Bitfield1
    pub fn set_isAnimBreak(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x4
        } else {
            self.Bitfield1 &= 0xFB
        }
    }
    /// Will the penetrating bullet hit (0: not hit, 1: hit) - 貫通弾丸が当たるか(0:当たらない, 1:当たる)
    /// Bitfield1
    pub fn get_isPenetrationBulletHit(&self) -> bool {
        &self.Bitfield1 & 0x8 != 0
    }

    /// Bitfield1
    pub fn set_isPenetrationBulletHit(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x8
        } else {
            self.Bitfield1 &= 0xF7
        }
    }
    /// Does the character win (0: not hit, 1: hit) - キャラが当たるか(0:当たらない, 1:当たる)
    /// Bitfield1
    pub fn get_isChrHit(&self) -> bool {
        &self.Bitfield1 & 0x10 != 0
    }

    /// Bitfield1
    pub fn set_isChrHit(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x10
        } else {
            self.Bitfield1 &= 0xEF
        }
    }
    /// Do you play the attack (0: do not play, 1: play) - 攻撃を弾くか(0:弾かない, 1:弾く)
    /// Bitfield1
    pub fn get_isAttackBacklash(&self) -> bool {
        &self.Bitfield1 & 0x20 != 0
    }

    /// Bitfield1
    pub fn set_isAttackBacklash(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x20
        } else {
            self.Bitfield1 &= 0xDF
        }
    }
    /// Broken at the initial appearance of the player (0: ru, 1: no) - プレイヤの初期出現で壊れ(0:る, 1:ない)
    /// Bitfield1
    pub fn get_isDisableBreakForFirstAppear(&self) -> bool {
        &self.Bitfield1 & 0x40 != 0
    }

    /// Bitfield1
    pub fn set_isDisableBreakForFirstAppear(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x40
        } else {
            self.Bitfield1 &= 0xBF
        }
    }
    /// Is it a ladder (0: different, 1: yes) - ハシゴか(0:ちがう, 1:そう)
    /// Bitfield1
    pub fn get_isLadder(&self) -> bool {
        &self.Bitfield1 & 0x80 != 0
    }

    /// Bitfield1
    pub fn set_isLadder(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x80
        } else {
            self.Bitfield1 &= 0x7F
        }
    }
    /// Do you want to stop the animation during the poly play (0: not, 1: do) - ポリ劇中アニメを停止するか(0:しない, 1:する)
    /// Bitfield2
    pub fn get_isAnimPauseOnRemoPlay(&self) -> bool {
        &self.Bitfield2 & 0x1 != 0
    }

    /// Bitfield2
    pub fn set_isAnimPauseOnRemoPlay(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x1
        } else {
            self.Bitfield2 &= 0xFE
        }
    }
    /// No damage (0: hit, 1: not hit) - ダメージが当たらない(0:当たる, 1:当たらない)
    /// Bitfield2
    pub fn get_isDamageNoHit(&self) -> bool {
        &self.Bitfield2 & 0x2 != 0
    }

    /// Bitfield2
    pub fn set_isDamageNoHit(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x2
        } else {
            self.Bitfield2 &= 0xFD
        }
    }
    /// Is it a moving object (0: different, 1: yes) - 移動オブジェか(0:ちがう, 1:そう)
    /// Bitfield2
    pub fn get_isMoveObj(&self) -> bool {
        &self.Bitfield2 & 0x4 != 0
    }

    /// Bitfield2
    pub fn set_isMoveObj(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x4
        } else {
            self.Bitfield2 &= 0xFB
        }
    }
    /// Suspension bridge object (0: different, 1: yes) - 吊り橋オブジェクトか(0:ちがう, 1:そう)
    /// Bitfield2
    pub fn get_isRopeBridge(&self) -> bool {
        &self.Bitfield2 & 0x8 != 0
    }

    /// Bitfield2
    pub fn set_isRopeBridge(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x8
        } else {
            self.Bitfield2 &= 0xF7
        }
    }
    /// Does the damage blow the rigid body (0: do not blow, 1: blow) - ダメージによって剛体が吹き飛ぶか(0:吹き飛ばない, 1:吹き飛ぶ)
    /// Bitfield2
    pub fn get_isAddRigidImpulse_ByDamage(&self) -> bool {
        &self.Bitfield2 & 0x10 != 0
    }

    /// Bitfield2
    pub fn set_isAddRigidImpulse_ByDamage(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x10
        } else {
            self.Bitfield2 &= 0xEF
        }
    }
    /// Will it break if the character gets on (0: it won't break 1: it will break) - キャラが乗ったら壊れるか(0:壊れるない 1:壊れる)
    /// Bitfield2
    pub fn get_isBreak_ByChrRide(&self) -> bool {
        &self.Bitfield2 & 0x20 != 0
    }

    /// Bitfield2
    pub fn set_isBreak_ByChrRide(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x20
        } else {
            self.Bitfield2 &= 0xDF
        }
    }
    /// Will it burn (0: not, 1:) - 燃焼するか(0:しない, 1:する)
    /// Bitfield2
    pub fn get_isBurn(&self) -> bool {
        &self.Bitfield2 & 0x40 != 0
    }

    /// Bitfield2
    pub fn set_isBurn(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x40
        } else {
            self.Bitfield2 &= 0xBF
        }
    }
    /// Broken when an enemy character touches (0: No, 1: Ru) - 敵キャラが接触したときに壊れ(0:ない, 1:る)
    /// Bitfield2
    pub fn get_isBreakByEnemyCollide(&self) -> bool {
        &self.Bitfield2 & 0x80 != 0
    }

    /// Bitfield2
    pub fn set_isBreakByEnemyCollide(&mut self, state: bool) {
        if state {
            self.Bitfield2 |= 0x80
        } else {
            self.Bitfield2 &= 0x7F
        }
    }
    /// Whether to pass the damage to the other side when receiving damage (0: pass, 1: do not pass) - ダメージを受けたときに、そのダメージを反対側に通さないかどうか　(0:通す, 1:通さない)
    /// Bitfield3
    pub fn get_isDamageCover(&self) -> bool {
        &self.Bitfield3 & 0x1 != 0
    }

    /// Bitfield3
    pub fn set_isDamageCover(&mut self, state: bool) {
        if state {
            self.Bitfield3 |= 0x1
        } else {
            self.Bitfield3 &= 0xFE
        }
    }
}
