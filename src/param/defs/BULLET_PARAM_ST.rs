/* This file was automatically generated from XML paramdefs. */
/// Data Version: 4
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct BULLET_PARAM_ST {

	/// NAME: Attack ID - 攻撃ID
	/// DESC: Register each attack parameter ID. → Attack type / Attack material / Physical attack power / Magic attack power / Stamina attack power / Knockback distance. - 攻撃パラメータのＩＤをそれぞれ登録する.→攻撃タイプ／攻撃材質／物理攻撃力／魔法攻撃力／スタミナ攻撃力／ノックバック距離.
	pub atkId_Bullet:i32,

	/// NAME: SFX ID [bullet] - SFXID【弾】
	/// DESC: Enter the SFX ID for [Bullet]. -1 does not occur. - 【弾】用SFX IDを入れる。-1は発生しない。
	pub sfxId_Bullet:i32,

	/// NAME: SFXID [landing] - SFXID【着弾】
	/// DESC: [Impact] Enter the SFX ID. -1 does not occur. - 【着弾】SFXIDを入れる。-1は発生しない。
	pub sfxId_Hit:i32,

	/// NAME: SFXID [at the time of repelling] - SFXID【はじき時】
	/// DESC: [At the time of repelling] Enter the SFX ID. -1 does not occur. - 【はじき時】SFXIDを入れる。-1は発生しない。
	pub sfxId_Flick:i32,

	/// NAME: Lifespan [s] - 寿命[s]
	/// DESC: Time for missiles to continue to exist (-1 is infinite). - 飛び道具が存在し続けられる時間（-1なら無限）.
	pub life:f32,

	/// NAME: Range [m] - 射程距離[m]
	/// DESC: Distance at which attenuation begins (not actual flight distance). - 減衰が開始される距離（実際の飛距離ではない）.
	pub dist:f32,

	/// NAME: Fire interval [s] - 発射間隔[s]
	/// DESC: Specify how many seconds the missile will be fired. - 飛び道具を何秒間隔で発射するかを指定.
	pub shootInterval:f32,

	/// NAME: Gravity within range [m / s ^ 2] - 射程距離内重力[m/s^2]
	/// DESC: Downward gravity within range. - 射程距離内での下向きにかかる重力.
	pub gravityInRange:f32,

	/// NAME: Gravity outside range [m / s ^ 2] - 射程距離外重力[m/s^2]
	/// DESC: Gravity applied downward when damping begins (expresses the feeling of falling down. - 減衰がはじまったときの下向きにかかる重力（ポトンと落ちる感じを表現.
	pub gravityOutRange:f32,

	/// NAME: Guidance stop distance [m] - 誘導停止距離[m]
	/// DESC: The distance to the target to stop the guidance. A parameter that prevents you from hitting too much with a guided bullet. - 誘導を停止するターゲットとの距離。誘導弾で当たり過ぎないようにするパラメータ。
	pub hormingStopRange:f32,

	/// NAME: Initial velocity [m / s] - 初速[m/s]
	/// DESC: Initial speed of SFX. - ＳＦＸの初速度.
	pub initVellocity:f32,

	/// NAME: Acceleration within range [m / s ^ 2] - 射程距離内加速度[m/s^2]
	/// DESC: Acceleration within SFX range. - ＳＦＸの射程内の加速度.
	pub accelInRange:f32,

	/// NAME: Acceleration outside range [m / s ^ 2] - 射程距離外加速度[m/s^2]
	/// DESC: Acceleration when SFX goes out of range. - ＳＦＸが射程距離外に出たときの加速度.
	pub accelOutRange:f32,

	/// NAME: Maximum speed [m / s] - 最高速度[m/s]
	/// DESC: maximum speed. - 最高速度.
	pub maxVellocity:f32,

	/// NAME: Minimum speed [m / s] - 最低速度[m/s]
	/// DESC: Minimum guaranteed speed. - 最低保証速度.
	pub minVellocity:f32,

	/// NAME: Acceleration start time [s] - 加速開始時間[s]
	/// DESC: Until this time, do not accelerate (make sure you can shoot magic like rockets). - この時間までは、加速しない（ロケット弾みたいな魔法を撃つことができるようにしておく）.
	pub accelTime:f32,

	/// NAME: Guidance start distance [m] - 誘導開始距離[m]
	/// DESC: How many meters should the guidance start? - 何ｍ進んだ地点から誘導を開始するか.
	pub homingBeginDist:f32,

	/// NAME: Initial radius [m] - 初期弾半径[m]
	/// DESC: Set the radius of the hit ball. - 当たり球の半径を設定する.
	pub hitRadius:f32,

	/// NAME: Maximum radius [m] - 最大弾半径[m]
	/// DESC: Maximum radius of the hit sphere (If -1, make it the same as the initial radius / default) - あたり球の最大半径（－1の場合、初期半径と同じにする／デフォルト）
	pub hitRadiusMax:f32,

	/// NAME: Range diffusion time [s] - 範囲拡散時間[s]
	/// DESC: The time when the radius of the range expands to a small extent. - 範囲半径が細大にまで広がる時間.
	pub spreadTime:f32,

	/// NAME: Activation delay [s] - 発動遅延[s]
	/// DESC: Time from landing to explosion (if 0, it explodes immediately). - 着弾後、爆発までの時間（０の場合はすぐに爆発）.
	pub expDelay:f32,

	/// NAME: Induction shift amount [m] - 誘導ずらし量[m]
	/// DESC: It is accurate if it is 0. At the time of shooting, each component of XYZ should be aimed by shifting this amount. - ０だと正確。射撃時にXYZ各成分を、この量だけずらして狙うようにする。
	pub hormingOffsetRange:f32,

	/// NAME: Time to live in damage hit history [s] - ダメージヒット履歴の生存時間[s]
	/// DESC: Damage hit history survival time [sec] (<= 0.0f: indefinite) - ダメージヒット履歴の生存時間[sec](<=0.0f：無期限)
	pub dmgHitRecordLifeTime:f32,

	/// NAME: External force [m / s ^ 2] - 外力[m/s^2]
	/// DESC: External force applied in the direction of shooting. (Y-axis is removed) - 射撃時の方向にかかる外力.(Y軸は抜いている)
	pub externalForce:f32,

	/// NAME: Special effects on the person who shot - 射撃した人にかける特殊効果
	/// DESC: Special effects on the person who shot - 射撃した人にかける特殊効果
	pub spEffectIDForShooter:i32,

	/// NAME: Funnel NPC Thinking ID - ファンネルNPC思考ID
	/// DESC: Parameters used by the funnel to search for the target - ファンネルがターゲットの検索使用するパラメータ
	pub autoSearchNPCThinkID:i32,

	/// NAME: Generated bullet ID - 発生弾丸ID
	/// DESC: Specify the ID when generating a new bullet parameter from the bullet parameter - 弾丸パラメータから、新しく弾丸パラメータを発生させるときにＩＤを指定する
	pub HitBulletID:i32,

	/// NAME: Special effect ID0 - 特殊効果ID0
	/// DESC: Register each special effect parameter ID. → General special effects. - 特殊効果パラメータのＩＤをそれぞれ登録する.→特殊効果全般.
	pub spEffectId0:i32,

	/// NAME: Special effect ID1 - 特殊効果ID1
	/// DESC: Register each special effect parameter ID. → General special effects. - 特殊効果パラメータのＩＤをそれぞれ登録する.→特殊効果全般.
	pub spEffectId1:i32,

	/// NAME: Special effect ID2 - 特殊効果ID2
	/// DESC: Register each special effect parameter ID. → General special effects. - 特殊効果パラメータのＩＤをそれぞれ登録する.→特殊効果全般.
	pub spEffectId2:i32,

	/// NAME: Special effect ID3 - 特殊効果ID3
	/// DESC: Register each special effect parameter ID. → General special effects. - 特殊効果パラメータのＩＤをそれぞれ登録する.→特殊効果全般.
	pub spEffectId3:i32,

	/// NAME: Special effect ID4 - 特殊効果ID4
	/// DESC: Register each special effect parameter ID. → General special effects. - 特殊効果パラメータのＩＤをそれぞれ登録する.→特殊効果全般.
	pub spEffectId4:i32,

	/// NAME: Number of shots - 発射数
	/// DESC: The number of missiles fired at one time. - 一度に発射する飛び道具の数.
	pub numShoot:u16,

	/// NAME: Induction performance [deg / s] - 誘導性能[deg/s]
	/// DESC: How many corrections per second? .. - 1秒間に何度まで補正するか？.
	pub homingAngle:i16,

	/// NAME: Launch angle [deg] - 発射角度[deg]
	/// DESC: Specify how many times the missile is fired forward. - 飛び道具を前方何度に向かって発射するかを指定.
	pub shootAngle:i16,

	/// NAME: Launch angle interval [deg] - 発射角度間隔[deg]
	/// DESC: When firing multiple missiles, specify how often to fire them. (Y-axis) - 飛び道具を複数発射する場合、何度間隔で発射するかを指定.(Y軸)
	pub shootAngleInterval:i16,

	/// NAME: Launch elevation interval [deg] - 発射仰角間隔[deg]
	/// DESC: When firing multiple missiles, specify how often to fire them. (X-axis) - 飛び道具を複数発射する場合、何度間隔で発射するかを指定.(X軸)
	pub shootAngleXInterval:i16,

	/// NAME: Physical attack power attenuation rate [% / s] - 物理攻撃力減衰率[%/s]
	/// DESC: Correction value that decreases in 1 second after the attenuation distance. - 減衰距離以降、1秒間に減少する補正値.
	pub damageDamp:i8,

	/// NAME: Magic attack power attenuation rate [% / s] - 魔法攻撃力減衰率[%/s]
	/// DESC: Correction value that decreases in 1 second after the attenuation distance. - 減衰距離以降、1秒間に減少する補正値.
	pub spelDamageDamp:i8,

	/// NAME: Flame attack power attenuation rate [% / s] - 炎攻撃力減衰率[%/s]
	/// DESC: Correction value that decreases in 1 second after the attenuation distance. - 減衰距離以降、1秒間に減少する補正値.
	pub fireDamageDamp:i8,

	/// NAME: Electric shock attack power attenuation rate [% / s] - 電撃攻撃力減衰率[%/s]
	/// DESC: Correction value that decreases in 1 second after the attenuation distance. - 減衰距離以降、1秒間に減少する補正値.
	pub thunderDamageDamp:i8,

	/// NAME: Stamina damage attenuation rate [% / s] - スタミナダメージ減衰率[%/s]
	/// DESC: Correction value that decreases in 1 second after the attenuation distance. - 減衰距離以降、1秒間に減少する補正値.
	pub staminaDamp:i8,

	/// NAME: Knockback attenuation factor [% / s] - ノックバック減衰率[%/s]
	/// DESC: Correction value that decreases in 1 second after the attenuation distance. - 減衰距離以降、1秒間に減少する補正値.
	pub knockbackDamp:i8,

	/// NAME: Launch elevation angle [deg] - 発射仰角[deg]
	/// DESC: Additional elevation angle from the horizontal. - 水平方向からの追加仰角。
	pub shootAngleXZ:i8,

	/// NAME: Lock direction limit angle - ロック方向制限角度
	/// DESC: Limit angle when facing the lock direction - ロック方向を向かせるときの制限角度
	pub lockShootLimitAng:u8,

	/// NAME: pad - pad
	pub pad2:[u8;1],

	/// NAME: Previous movement direction addition rate [%] - 前回の移動方向加算率[%]
	/// DESC: Ratio to add the previous movement direction to the current direction when the sliding bullet hits the wall - 滑る弾が壁にヒット時に前回の移動方向を今の方向へ加算する比率
	pub prevVelocityDirRate:u8,

	/// NAME: Physical attributes - 物理属性
	/// DESC: Set the physical attributes to set for the bullet - 弾丸に設定する物理属性を設定
	pub atkAttribute:u8,

	/// NAME: Special attributes - 特殊属性
	/// DESC: Set special attributes to set for bullets - 弾丸に設定する特殊属性を設定
	pub spAttribute:u8,

	/// NAME: Attack attribute [SFX / SE] - 攻撃属性[SFX/SE]
	/// DESC: Specify what the attack attribute is - 攻撃属性が何かを指定する
	pub Material_AttackType:u8,

	/// NAME: Attack material [SFX / SE] - 攻撃材質[SFX/SE]
	/// DESC: Used for SFX / SE during attack - 攻撃時のSFX/ＳＥに使用
	pub Material_AttackMaterial:u8,

	/// NAME: Penetrate the character? - キャラを貫通？
	/// DESC: If it is ON, it will penetrate without landing when it hits the character. - ONであればキャラに当たったときに着弾せず貫通する
	pub Bitfield1:u8,

	/// NAME: Occurrence condition - 発生条件
	/// DESC: Specify the condition to judge whether to generate a bullet when it lands or the life is extinguished - 着弾・寿命消滅時に弾を発生させるか判定する条件を指定
	pub launchConditionType:u8,

	/// NAME: Follow-up type - 追従タイプ
	/// DESC: Follow-up type. "Do not follow" is the default. - 追従タイプ。「追従しない」がデフォルト。
	pub Bitfield2:u8,

	/// NAME: Penetrate the map? - マップを貫通？
	/// DESC: If it is ON, it will penetrate without landing when hitting a hit / static asset. - ONであればヒット/静的アセットに当たったときに着弾せず貫通する
	pub Bitfield3:u8,

	/// NAME: Whether to ignore the state transition at the time of water collision - 水面衝突時の状態遷移を無視するか
	/// DESC: Whether to ignore the state transition even if it hits water - 水に当たっても状態遷移を無視するか
	pub Bitfield4:u8,

	/// NAME: Dark attack power attenuation rate [% / s] - 闇攻撃力減衰率[%/s]
	/// DESC: Correction value that decreases in 1 second after the attenuation distance. - 減衰距離以降、1秒間に減少する補正値.
	pub darkDamageDamp:i8,

	/// NAME: Bullet SFX extinction type at the time of landing - 着弾時の弾丸SFX消滅タイプ
	/// DESC: Bullet SFX extinction type when landing or playing - 着弾or弾き時の弾丸SFX消滅タイプ
	pub bulletSfxDeleteType_byHit:i8,

	/// NAME: Bullet SFX extinction type at the end of life - 寿命時の弾丸SFX消滅タイプ
	/// DESC: Bullet SFX extinction type at the end of life - 寿命時の弾丸SFX消滅タイプ
	pub bulletSfxDeleteType_byLifeDead:i8,

	/// NAME: Target vertical offset [m] - 目標上下オフセット[m]
	/// DESC: Vertical offset of landing position. Shift the target position up and down at the time of launch and during homing. (-N ~ n) - 着弾位置の上下オフセット。発射時とホーミング中のターゲット位置を上下にずらす。（-n～n）
	pub targetYOffsetRange:f32,

	/// NAME: Launch angle random number [deg] - 発射角度乱数[deg]
	/// DESC: Upper limit of random number of firing angle (0 to 360) - 発射角度乱数の上限（0～360）
	pub shootAngleYMaxRandom:f32,

	/// NAME: Elevation angle random number [deg] - 発射仰角乱数[deg]
	/// DESC: Upper limit of firing elevation random number (0 to 360) - 発射仰角乱数の上限（0～360）
	pub shootAngleXMaxRandom:f32,

	/// NAME: Interval specified bullet ID - 間隔指定発生弾丸ID
	/// DESC: Bullet ID used when making bullets at regular intervals - 一定間隔で弾丸を作る時に使う、弾丸のID
	pub intervalCreateBulletId:i32,

	/// NAME: Occurrence interval: Minimum time [s] - 発生間隔：最小時間[s]
	/// DESC: Minimum interval for making bullets at regular intervals (0 to n) - 一定間隔で弾丸を作る間隔の最小（0～n）
	pub intervalCreateTimeMin:f32,

	/// NAME: Occurrence interval: Maximum time [s] - 発生間隔：最大時間[s]
	/// DESC: Maximum interval for making bullets at regular intervals (function is disabled if 0 to n 0) - 一定間隔で弾丸を作る間隔の最大（0～n 0なら機能無効）
	pub intervalCreateTimeMax:f32,

	/// NAME: Predicted shooting velocity observation time [s] - 予測射撃の速度観測時間[s]
	/// DESC: Average speed observation time of predicted shooting function (function is invalid if 0 to 40) - 予測射撃機能の平均速度観測時間（0～4 0なら機能無効）
	pub predictionShootObserveTime:f32,

	/// NAME: Waiting time for start of specified interval [s] - 間隔指定発生開始待ち時間[s]
	/// DESC: Waiting time to start making bullets at regular intervals - 一定間隔で弾丸を作り始めるまでの待ち時間
	pub intervalCreateWaitTime:f32,

	/// NAME: The type of SFX attitude generated from the bullet - 弾丸から発生したSFXの姿勢のタイプ
	/// DESC: Set the initial attitude of an SFX or sub-bullet created from a bullet - 弾丸から作成されたSFXまたは子弾丸の初期姿勢を設定する
	pub sfxPostureType:u8,

	/// NAME: Creation restriction group Id - 作成制限グループId
	/// DESC: If it is 0, it is unused. If the upper limit is reached when creating a bullet set in the same group Id, that bullet will not be created. (Bullets created synchronously on the network will be released regardless) - 0なら未使用。同一のグループIdに設定された弾丸を作成するときに上限に達していたらその弾丸を作成しない。（ネットワークで同期作成された弾は関係なく出る）
	pub createLimitGroupId:u8,

	/// NAME: pad - pad
	pub pad5:[u8;1],

	/// NAME: Will the speed be taken over by the submunition? - 子弾に速度を引き継ぐか
	/// DESC: Take over the speed of the timing to replace the submunition. Ignore the speed set for the submunition - 子弾に差し替わるタイミングの速度を引き継ぐ。子弾に設定された速度は無視する
	pub Bitfield5:u8,

	/// NAME: Occurrence range (radius) at random occurrence [m] - ランダム発生時の発生範囲(半径)[m]
	/// DESC: The range of bullets used when the source type is set to occur at random locations. - 発生源タイプがランダムな位置に発生する設定の場合に利用される、弾丸の発生範囲。
	pub randomCreateRadius:f32,

	/// NAME: Funnel tracking position_base point height [m] - ファンネル追従位置_基点高さ[m]
	/// DESC: Funnel tracking position_base point height [m] - ファンネル追従位置_基点高さ[m]
	pub followOffset_BaseHeight:f32,

	/// NAME: Asset number generated at the time of landing - 着弾時に発生するアセット番号
	/// DESC: The number of the asset to be generated at the time of landing. -1: Do not generate. The asset number is the last 6 digits of the asset. Example: AEG999_999 = 999999 - 着弾時に発生させるアセットの番号。-1：生成しない。アセット番号はアセットの下6桁の数値です。例： AEG999_999 = 999999
	pub assetNo_Hit:i32,

	/// NAME: Lifetime random number [s] - 寿命乱数[s]
	/// DESC: Add a random number of seconds with a set time fluctuation range to the "lifetime [s]". - 「寿命[s]」に対して、設定した時間の振れ幅を持つ乱数秒を加える
	pub lifeRandomRange:f32,

	/// NAME: Induction performance (X-axis individual) [deg / s] - 誘導性能（X軸個別）[deg/s]
	/// DESC: Only the X-axis component of the inductive performance is changed. Do not change with -1 - 誘導性能のX軸成分だけを変えます。-1で変えません
	pub homingAngleX:i16,

	/// NAME: Ballistic calculation type - 弾道計算タイプ
	/// DESC: Ballistic calculation type - 弾道計算タイプ
	pub ballisticCalcType:u8,

	/// NAME: Attach effect type - アタッチ効果タイプ
	/// DESC: Effect type to attach - アタッチする効果タイプ
	pub attachEffectType:u8,

	/// NAME: SEID1 [bullet] - SEID1【弾】
	/// DESC: Insert SE ID1 for [Bullet]. -1: Not generated 9 digits. Sound type is fixed to s: SFX. - 【弾】用SE ID1 を入れる。-1：生成しない　9桁。サウンドタイプはs：SFX固定。
	pub seId_Bullet1:i32,

	/// NAME: SEID2 [bullet] - SEID2【弾】
	/// DESC: Insert SE ID 2 for [Bullet]. -1: Not generated 9 digits. Sound type is fixed to s: SFX. - 【弾】用SE ID2 を入れる。-1：生成しない　9桁。サウンドタイプはs：SFX固定。
	pub seId_Bullet2:i32,

	/// NAME: SEID [landing] - SEID【着弾】
	/// DESC: Insert SE ID 1 for [landing]. -1 does not occur. 9 digits. Sound type is fixed to s: SFX. - 【着弾】用SE ID1 を入れる。-1は発生しない。 9桁。サウンドタイプはs：SFX固定。
	pub seId_Hit:i32,

	/// NAME: SEID [at the time of repelling] - SEID【はじき時】
	/// DESC: [At the time of repelling] Enter SE ID1 for. -1 does not occur. 9 digits. Sound type is fixed to s: SFX. - 【はじき時】用SE ID1 を入れる。-1は発生しない。 9桁。サウンドタイプはs：SFX固定。
	pub seId_Flick:i32,

	/// NAME: [Curly shooting] Launch elevation limit_lower limit [deg] - 【曲射】発射仰角制限_下限[deg]
	/// DESC: [Curly fire] The lower limit [deg] with the firing elevation angle as a reference (0 deg) before applying the curve fire calculation. - 【曲射】曲射計算の適用前の発射仰角を基準(0deg)とした下限[deg]。
	pub howitzerShootAngleXMin:i16,

	/// NAME: [Song firing] Launch elevation limit_upper limit [deg] - 【曲射】発射仰角制限_上限[deg]
	/// DESC: [Sky firing] The upper limit [deg] based on the firing elevation angle (0deg) before applying the bending firing calculation. - 【曲射】曲射計算の適用前の発射仰角を基準(0deg)とした上限[deg]。
	pub howitzerShootAngleXMax:i16,

	/// NAME: [Song shooting] Minimum speed limit [m / s] - 【曲射】最低制限速度[m/s]
	/// DESC: [Song firing] The minimum speed limit for song firing calculation [m / s]. - 【曲射】曲射計算の最低制限速度[m/s]。
	pub howitzerInitMinVelocity:f32,

	/// NAME: [Song shooting] Maximum speed limit [m / s] - 【曲射】最高制限速度[m/s]
	/// DESC: [Song firing] Maximum speed limit for song firing calculation [m / s]. - 【曲射】曲射計算の最高制限速度[m/s]。
	pub howitzerInitMaxVelocity:f32,

	/// NAME: SFXID [At the time of forced erasure] - SFXID【強制消去時】
	/// DESC: SFX ID at the time of forced erasure. -1 does not occur. - 強制消去時SFXID。-1は発生しない。
	pub sfxId_ForceErase:i32,

	/// NAME: Bullet SFX extinction type at the time of forced erasure - 強制消去時の弾丸SFX消滅タイプ
	/// DESC: Bullet SFX extinction type at the time of forced erasure - 強制消去時の弾丸SFX消滅タイプ
	pub bulletSfxDeleteType_byForceErase:i8,

	/// NAME: Padding 3 - パディング3
	/// DESC: pad3 - pad3
	pub pad3:[u8;1],

	/// NAME: SFX direction specification when following Damipoli - 追従時SFX方向指定ダミポリ
	/// DESC: SFX direction specification when following Damipoli - 追従時SFX方向指定ダミポリ
	pub followDmypoly_forSfxPose:i16,

	/// NAME: Funnel tracking position_radius [m] - ファンネル追従位置_半径[m]
	/// DESC: Funnel tracking position_radius [m] - ファンネル追従位置_半径[m]
	pub followOffset_Radius:f32,

	/// NAME: Special effect flight distance correction magnification - 特殊効果飛距離補正倍率
	/// DESC: Special effect flight distance correction magnification - 特殊効果飛距離補正倍率
	pub spBulletDistUpRate:f32,

	/// NAME: Target range [m] when unlocked - 非ロック時ターゲット射程距離[m]
	/// DESC: Target range when unlocked (-1: Refer to "range", 0: No target) - 非ロック時ターゲット射程距離（-1：「射程距離」を参照する、0：ターゲットなし）
	pub nolockTargetDist:f32,

	/// NAME: pad - pad
	pub pad4:[u8;8],
}

impl BULLET_PARAM_ST {
	/// If it is ON, it will penetrate without landing when it hits the character. - ONであればキャラに当たったときに着弾せず貫通する
	/// Bitfield1
	pub fn get_isPenetrateChr(&self) -> bool {
		&self.Bitfield1 & 0x1 != 0
	}

	/// Bitfield1
	pub fn set_isPenetrateChr(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x1
		} else {
			self.Bitfield1 &= 0xFE
		}
	}
	/// If it is ON, it will penetrate without landing when hitting a dynamic / partial destruction asset. - ONであれば動的/部分破壊アセットに当たったときに着弾せず貫通する
	/// Bitfield1
	pub fn get_isPenetrateObj(&self) -> bool {
		&self.Bitfield1 & 0x2 != 0
	}

	/// Bitfield1
	pub fn set_isPenetrateObj(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x2
		} else {
			self.Bitfield1 &= 0xFD
		}
	}
	/// 
	/// Bitfield1
	pub fn get_pad(&self) -> u8 {
		&self.Bitfield1 & 0xFC
	}

	/// Bitfield1 MAX: 63
	pub fn set_pad(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 2) & 0xFC;
			let newVal = &self.Bitfield1 & 0x3 | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0x3
		}
	}	/// Follow-up type. "Do not follow" is the default. - 追従タイプ。「追従しない」がデフォルト。
	/// Bitfield2
	pub fn get_FollowType(&self) -> u8 {
		&self.Bitfield2 & 0x1C
	}

	/// Bitfield2 MAX: 7
	pub fn set_FollowType(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 2) & 0x1C;
			let newVal = &self.Bitfield2 & 0xE3 | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0xE3
		}
	}	/// Source type. Usually from Damipoli. (Introduced to judge meteo) - 発生源タイプ。ダミポリからが通常。（メテオを判定するために導入）
	/// Bitfield2
	pub fn get_EmittePosType(&self) -> u8 {
		&self.Bitfield2 & 0x38
	}

	/// Bitfield2 MAX: 7
	pub fn set_EmittePosType(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 3) & 0x38;
			let newVal = &self.Bitfield2 & 0xC7 | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0xC7
		}
	}	/// Set whether bullets such as arrows will remain stuck in the character - 矢などの弾丸が、キャラクターに刺さったままになるかどうかを設定する
	/// Bitfield2
	pub fn get_isAttackSFX(&self) -> bool {
		&self.Bitfield2 & 0x40 != 0
	}

	/// Bitfield2
	pub fn set_isAttackSFX(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x40
		} else {
			self.Bitfield2 &= 0xBF
		}
	}
	/// Do you keep hitting? - あたり続けるか？
	/// Bitfield2
	pub fn get_isEndlessHit(&self) -> bool {
		&self.Bitfield2 & 0x80 != 0
	}

	/// Bitfield2
	pub fn set_isEndlessHit(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x80
		} else {
			self.Bitfield2 &= 0x7F
		}
	}
	/// If it is ON, it will penetrate without landing when hitting a hit / static asset. - ONであればヒット/静的アセットに当たったときに着弾せず貫通する
	/// Bitfield3
	pub fn get_isPenetrateMap(&self) -> bool {
		&self.Bitfield3 & 0x1 != 0
	}

	/// Bitfield3
	pub fn set_isPenetrateMap(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x1
		} else {
			self.Bitfield3 &= 0xFE
		}
	}
	/// Are you an enemy or an ally? (Not a wandering ghost) - 敵味方共にあたるか？（徘徊ゴーストにはあたらない）
	/// Bitfield3
	pub fn get_isHitBothTeam(&self) -> bool {
		&self.Bitfield3 & 0x2 != 0
	}

	/// Bitfield3
	pub fn set_isHitBothTeam(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x2
		} else {
			self.Bitfield3 &= 0xFD
		}
	}
	/// Specify whether to share the hit list - ヒットリストを共有するかを指定
	/// Bitfield3
	pub fn get_isUseSharedHitList(&self) -> bool {
		&self.Bitfield3 & 0x4 != 0
	}

	/// Bitfield3
	pub fn set_isUseSharedHitList(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x4
		} else {
			self.Bitfield3 &= 0xFB
		}
	}
	/// Do you use the same Damipoly ID more than once when placing bullets? - 弾配置時に同一ダミポリIDを複数使うか？
	/// Bitfield3
	pub fn get_isUseMultiDmyPolyIfPlace(&self) -> bool {
		&self.Bitfield3 & 0x8 != 0
	}

	/// Bitfield3
	pub fn set_isUseMultiDmyPolyIfPlace(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x8
		} else {
			self.Bitfield3 &= 0xF7
		}
	}
	/// Does it hit other bullets forced erasure A? - 他弾強制消去Aに当たるか
	/// Bitfield3
	pub fn get_isHitOtherBulletForceEraseA(&self) -> bool {
		&self.Bitfield3 & 0x10 != 0
	}

	/// Bitfield3
	pub fn set_isHitOtherBulletForceEraseA(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x10
		} else {
			self.Bitfield3 &= 0xEF
		}
	}
	/// Does it hit the other bullet forced erasure B? - 他弾強制消去Bに当たるか
	/// Bitfield3
	pub fn get_isHitOtherBulletForceEraseB(&self) -> bool {
		&self.Bitfield3 & 0x20 != 0
	}

	/// Bitfield3
	pub fn set_isHitOtherBulletForceEraseB(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x20
		} else {
			self.Bitfield3 &= 0xDF
		}
	}
	/// Do you hit the force magic? - フォース魔法に当たるか
	/// Bitfield3
	pub fn get_isHitForceMagic(&self) -> bool {
		&self.Bitfield3 & 0x40 != 0
	}

	/// Bitfield3
	pub fn set_isHitForceMagic(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x40
		} else {
			self.Bitfield3 &= 0xBF
		}
	}
	/// Should I ignore the effect if it hits the surface of the water? - 水面に当たった場合はエフェクト無視するか
	/// Bitfield3
	pub fn get_isIgnoreSfxIfHitWater(&self) -> bool {
		&self.Bitfield3 & 0x80 != 0
	}

	/// Bitfield3
	pub fn set_isIgnoreSfxIfHitWater(&mut self, state: bool) {
		if state {
			self.Bitfield3 |= 0x80
		} else {
			self.Bitfield3 &= 0x7F
		}
	}
	/// Whether to ignore the state transition even if it hits water - 水に当たっても状態遷移を無視するか
	/// Bitfield4
	pub fn get_isIgnoreMoveStateIfHitWater(&self) -> bool {
		&self.Bitfield4 & 0x1 != 0
	}

	/// Bitfield4
	pub fn set_isIgnoreMoveStateIfHitWater(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x1
		} else {
			self.Bitfield4 &= 0xFE
		}
	}
	/// Do you hit the dark force magic? - 闇フォース魔法に当たるか
	/// Bitfield4
	pub fn get_isHitDarkForceMagic(&self) -> bool {
		&self.Bitfield4 & 0x2 != 0
	}

	/// Bitfield4
	pub fn set_isHitDarkForceMagic(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x2
		} else {
			self.Bitfield4 &= 0xFD
		}
	}
	/// Damage calculation side. During multiplayer, the damage calculation is for switching between the giving side and the receiving side. - ダメージ計算サイド。　マルチプレイ時に、ダメージの計算を、与えた側 or 受けた側を切り替えるためのもの。
	/// Bitfield4
	pub fn get_dmgCalcSide(&self) -> u8 {
		&self.Bitfield4 & 0xC
	}

	/// Bitfield4 MAX: 3
	pub fn set_dmgCalcSide(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 2) & 0xC;
			let newVal = &self.Bitfield4 & 0xF3 | val;
			self.Bitfield4 = newVal
		} else {
			self.Bitfield4 &= 0xF3
		}
	}	/// Whether to automatically follow when not locked on - 非ロックオン時に自動追従するか
	/// Bitfield4
	pub fn get_isEnableAutoHoming(&self) -> bool {
		&self.Bitfield4 & 0x10 != 0
	}

	/// Bitfield4
	pub fn set_isEnableAutoHoming(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x10
		} else {
			self.Bitfield4 &= 0xEF
		}
	}
	/// In the case of a synchronously generated bullet, the emitter attitude at the time of synchronization is used without recalculating the attitude due to the Damipoli position when the bullet is generated. - 同期生成された弾丸の場合、弾丸生成時にダミポリ位置による姿勢の再計算を行わず、同期時のエミッタ姿勢を使用する。
	/// Bitfield4
	pub fn get_isSyncBulletCulcDumypolyPos(&self) -> bool {
		&self.Bitfield4 & 0x20 != 0
	}

	/// Bitfield4
	pub fn set_isSyncBulletCulcDumypolyPos(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x20
		} else {
			self.Bitfield4 &= 0xDF
		}
	}
	/// Only valid for sub-bullets. If it is ON, the reference direction is the owner. - 子弾丸のみ有効。ONなら基準方向をオーナーにする。
	/// Bitfield4
	pub fn get_isOwnerOverrideInitAngle(&self) -> bool {
		&self.Bitfield4 & 0x40 != 0
	}

	/// Bitfield4
	pub fn set_isOwnerOverrideInitAngle(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x40
		} else {
			self.Bitfield4 &= 0xBF
		}
	}
	/// Take over the SFX of the parent bullet. Ignore the SFX ID set for the sub bullet - 親弾のSFXを引き継ぐ。子弾に設定されたSFXIDは無視する 
	/// Bitfield4
	pub fn get_isInheritSfxToChild(&self) -> bool {
		&self.Bitfield4 & 0x80 != 0
	}

	/// Bitfield4
	pub fn set_isInheritSfxToChild(&mut self, state: bool) {
		if state {
			self.Bitfield4 |= 0x80
		} else {
			self.Bitfield4 &= 0x7F
		}
	}
	/// Take over the speed of the timing to replace the submunition. Ignore the speed set for the submunition - 子弾に差し替わるタイミングの速度を引き継ぐ。子弾に設定された速度は無視する
	/// Bitfield5
	pub fn get_isInheritSpeedToChild(&self) -> bool {
		&self.Bitfield5 & 0x1 != 0
	}

	/// Bitfield5
	pub fn set_isInheritSpeedToChild(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x1
		} else {
			self.Bitfield5 &= 0xFE
		}
	}
	/// When ON, the bullet parameter "landing SFX" does not play even if it hits a character / object. - ONの時、キャラクター/オブジェクトに着弾しても弾丸パラメータの「着弾SFX」を再生しない
	/// Bitfield5
	pub fn get_isDisableHitSfx_byChrAndObj(&self) -> bool {
		&self.Bitfield5 & 0x2 != 0
	}

	/// Bitfield5
	pub fn set_isDisableHitSfx_byChrAndObj(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x2
		} else {
			self.Bitfield5 &= 0xFD
		}
	}
	/// There was a problem with the digging judgment when firing a bullet, so it is for error handling. SEQ23101 [Own character] If you use a soul short arrow or a strong soul short arrow in close contact with a character with a high lock-on position, the direction of the bullet will be reversed. - 弾丸発射時めり込み判定に不具合があったので、それのエラーハンドリング用。SEQ23101 【自キャラ】ロックオン位置の高いキャラに密着してソウルの短矢、強いソウルの短矢を使うと弾丸の方向が反転する
	/// Bitfield5
	pub fn get_isCheckWall_byCenterRay(&self) -> bool {
		&self.Bitfield5 & 0x4 != 0
	}

	/// Bitfield5
	pub fn set_isCheckWall_byCenterRay(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x4
		} else {
			self.Bitfield5 &= 0xFB
		}
	}
	/// Do you hit flare magic? - フレア魔法に当たるか
	/// Bitfield5
	pub fn get_isHitFlare(&self) -> bool {
		&self.Bitfield5 & 0x8 != 0
	}

	/// Bitfield5
	pub fn set_isHitFlare(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x8
		} else {
			self.Bitfield5 &= 0xF7
		}
	}
	/// Do you use primitive magic Atari? It will change to a filter that corresponds to the Atari dedicated to primitive magic. - 原始魔法アタリを使うか？原始魔法専用アタリに当たるフィルタに変わります。
	/// Bitfield5
	pub fn get_isUseBulletWallFilter(&self) -> bool {
		&self.Bitfield5 & 0x10 != 0
	}

	/// Bitfield5
	pub fn set_isUseBulletWallFilter(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x10
		} else {
			self.Bitfield5 &= 0xEF
		}
	}
	/// 
	/// Bitfield5
	pub fn get_pad1(&self) -> bool {
		&self.Bitfield5 & 0x20 != 0
	}

	/// Bitfield5
	pub fn set_pad1(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x20
		} else {
			self.Bitfield5 &= 0xDF
		}
	}
	/// The number of funnels on the PC does not fluctuate by force. Become the number of shots - PCのファンネル数が理力で変動しない。発射数になる
	/// Bitfield5
	pub fn get_isNonDependenceMagicForFunnleNum(&self) -> bool {
		&self.Bitfield5 & 0x40 != 0
	}

	/// Bitfield5
	pub fn set_isNonDependenceMagicForFunnleNum(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x40
		} else {
			self.Bitfield5 &= 0xBF
		}
	}
	/// Does it react to AI bullets (even with 0 attack power)? - AI弾丸反応するか（攻撃力0でも）
	/// Bitfield5
	pub fn get_isAiInterruptShootNoDamageBullet(&self) -> bool {
		&self.Bitfield5 & 0x80 != 0
	}

	/// Bitfield5
	pub fn set_isAiInterruptShootNoDamageBullet(&mut self, state: bool) {
		if state {
			self.Bitfield5 |= 0x80
		} else {
			self.Bitfield5 &= 0x7F
		}
	}

}
