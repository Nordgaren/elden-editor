/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct DECAL_PARAM_ST {

	/// NAME: Texture ID - テクスチャID
	/// DESC: Texture ID - テクスチャID
	pub textureId:i32,

	/// NAME: Damipoli ID - ダミポリID
	/// DESC: Damipoly ID of decal generation standard. If specified by TAE, it will be the value of TAE. - デカール発生基準のダミポリID。TAEで指定している場合はTAEの値になる
	pub dmypolyId:i32,

	/// NAME: Reference angle offset_up / down [deg] - 基準角度オフセット_上下[deg]
	/// DESC: Reference angle offset_up / down [deg] - 基準角度オフセット_上下[deg]
	pub pitchAngle:f32,

	/// NAME: Reference angle offset_left / right [deg] - 基準角度オフセット_左右[deg]
	/// DESC: Reference angle offset_left / right [deg] - 基準角度オフセット_左右[deg]
	pub yawAngle:f32,

	/// NAME: Pasting start distance [m] - 貼り付け開始距離[m]
	/// DESC: Pasting start distance [m] - 貼り付け開始距離[m]
	pub nearDistance:f32,

	/// NAME: Paste end distance [m] - 貼り付け終了距離[m]
	/// DESC: Paste end distance [m] - 貼り付け終了距離[m]
	pub farDistance:f32,

	/// NAME: Size at the start distance [m] - 開始距離での大きさ[m]
	/// DESC: Size at the start distance [m] - 開始距離での大きさ[m]
	pub nearSize:f32,

	/// NAME: Size at the end distance [m] - 終了距離での大きさ[m]
	/// DESC: Size at the end distance [m] - 終了距離での大きさ[m]
	pub farSize:f32,

	/// NAME: Monitoring special effects ID - 監視特殊効果ID
	/// DESC: Monitoring special effect ID. If you enter an arbitrary special effect ID, decals will not be generated unless the special effect is applied. - 監視特殊効果ID。任意の特殊効果IDを入れた場合、その特殊効果がかかっていないとデカールを発生しなくなる。
	pub maskSpeffectId:i32,

	/// NAME: Padding - パディング
	/// DESC: Padding - パディング
	pub Bitfield1:u32,

	/// NAME: Random scale minimum [%] - ランダムスケール最小値[％]
	/// DESC: Random scale minimum [%] - ランダムスケール最小値[％]
	pub randomSizeMin:i16,

	/// NAME: Random scale maximum [%] - ランダムスケール最大値[％]
	/// DESC: Random scale maximum [%] - ランダムスケール最大値[％]
	pub randomSizeMax:i16,

	/// NAME: Random angle_minimum twist [deg] - ランダム角度_ひねり最小値[deg]
	/// DESC: Random angle_minimum twist [deg] - ランダム角度_ひねり最小値[deg]
	pub randomRollMin:f32,

	/// NAME: Random angle _ maximum twist [deg] - ランダム角度_ひねり最大値[deg]
	/// DESC: Random angle _ maximum twist [deg] - ランダム角度_ひねり最大値[deg]
	pub randomRollMax:f32,

	/// NAME: Random angle_minimum top and bottom [deg] - ランダム角度_上下最小値[deg]
	/// DESC: Random angle_minimum top and bottom [deg] - ランダム角度_上下最小値[deg]
	pub randomPitchMin:f32,

	/// NAME: Random angle_maximum value [deg] - ランダム角度_上下最大値[deg]
	/// DESC: Random angle_maximum value [deg] - ランダム角度_上下最大値[deg]
	pub randomPitchMax:f32,

	/// NAME: Random angle_minimum left and right [deg] - ランダム角度_左右最小値[deg]
	/// DESC: Random angle_minimum left and right [deg] - ランダム角度_左右最小値[deg]
	pub randomYawMin:f32,

	/// NAME: Random angle_maximum left and right [deg] - ランダム角度_左右最大値[deg]
	/// DESC: Random angle_maximum left and right [deg] - ランダム角度_左右最大値[deg]
	pub randomYawMax:f32,

	/// NAME: POM height scale - POM高さスケール
	/// DESC: POM height scale - POM高さスケール
	pub pomHightScale:f32,

	/// NAME: Minimum number of POM samples - POM最小サンプル数
	/// DESC: Minimum number of POM samples - POM最小サンプル数
	pub pomSampleMin:u8,

	/// NAME: Maximum number of POM samples - POM最大サンプル数
	/// DESC: Maximum number of POM samples - POM最大サンプル数
	pub pomSampleMax:u8,

	/// NAME: Blend mode - ブレンドモード
	/// DESC: Blend mode - ブレンドモード
	pub blendMode:i8,

	/// NAME: Reference coordinates for skipping decals - デカールを飛ばす基準座標
	/// DESC: Reference coordinates that determine the direction in which the decal is flown - デカールを飛ばす方向を決定する基準座標
	pub appearDirType:i8,

	/// NAME: Emissive starting price - エミッシブ 開始値
	/// DESC: Emissive starting price - エミッシブ 開始値
	pub emissiveValueBegin:f32,

	/// NAME: Emissive end value - エミッシブ 終了値
	/// DESC: Emissive end value - エミッシブ 終了値
	pub emissiveValueEnd:f32,

	/// NAME: Emissive update time (seconds) - エミッシブ 更新時間(秒)
	/// DESC: Interpolation time from start value to end value - 開始値～終了値の補間時間
	pub emissiveTime:f32,

	/// NAME: Do you want to interpolate? - 補間するか？
	/// DESC: Do you generate the time when the bar is extended due to the decal generation of TAE? - TAEのデカル発生でバーを伸ばしてる時間発生させるか？
	pub bIntpEnable:u8,

	/// NAME: Padding - パディング
	/// DESC: Padding - パディング
	pub pad_01:[u8;3],

	/// NAME: Interpolation interval [m] - 補間間隔[m]
	/// DESC: Distance to interpolate decals generated between TAE bars when interpolation is enabled - 補間有効時にTAEのバーの間で発生したデカルを補間する距離
	pub intpIntervalDist:f32,

	/// NAME: Texture ID at the start of interpolation - 補間開始時のテクスチャID
	/// DESC: Texture ID at the start of interpolation (-1 uses the same value as the texture ID) - 補間開始時のテクスチャID（-1でテクスチャIDと同じ値を使う）
	pub beginIntpTextureId:i32,

	/// NAME: Texture ID at the end of interpolation - 補間終了時のテクスチャID
	/// DESC: Texture at the end of interpolation Texture ID (-1 uses the same value as the texture ID) - 補間終了時のテクスチャテクスチャID（-1でテクスチャIDと同じ値を使う）
	pub endIntpTextureId:i32,

	/// NAME: SFX ID issued when the decal is affixed - デカールが貼られた時に出すSFXID
	/// DESC: SFXID issued when the decal is affixed (Nothing is issued with -1) - デカールが貼られた時に出すSFXID（-1で何も出さない）
	pub appearSfxId:i32,

	/// NAME: Offset position of SFX - SFXのオフセット位置
	/// DESC: Offset distance of SFX generation position - SFX発生位置のオフセット距離
	pub appearSfxOffsetPos:f32,

	/// NAME: Mask texture ID - マスクテクスチャID
	/// DESC: Mask texture ID (see texture ID with -1) - マスクテクスチャID（-1でテクスチャIDを見る）
	pub maskTextureId:i32,

	/// NAME: Albedo texture ID - アルベドテクスチャID
	/// DESC: Albedo Texture ID (See Texture ID with -1) - アルベドテクスチャID（-1でテクスチャIDを見る）
	pub diffuseTextureId:i32,

	/// NAME: Reflect texture ID - リフレクテクスチャID
	/// DESC: Reflectance texture ID (see texture ID with -1) - リフレクタンステクスチャID（-1でテクスチャIDを見る）
	pub reflecTextureId:i32,

	/// NAME: Mask strength - マスクの強さ
	/// DESC: Mask strength (currently valid only with deferred decals) - マスクの強さ（現状、デファードデカールでのみ有効）
	pub maskScale:f32,

	/// NAME: Normal texture ID - ノーマルテクスチャID
	/// DESC: Normal texture ID (see texture ID with -1) - ノーマルテクスチャID（-1でテクスチャIDを見る）
	pub normalTextureId:i32,

	/// NAME: Height texture ID - ハイトテクスチャID
	/// DESC: Height texture ID (see texture ID with -1) - ハイトテクスチャID（-1でテクスチャIDを見る）
	pub heightTextureId:i32,

	/// NAME: Emissive Texture ID - エミッシブテクスチャID
	/// DESC: Emissive Texture ID (See Texture ID with -1) - エミッシブテクスチャID（-1でテクスチャIDを見る）
	pub emissiveTextureId:i32,

	/// NAME: Albedo color: R - アルベドカラー：R
	/// DESC: Albedo color: R - アルベドの色：R
	pub diffuseColorR:u8,

	/// NAME: Albedo color: G - アルベドカラー：G
	/// DESC: Albedo color: G - アルベドの色：G
	pub diffuseColorG:u8,

	/// NAME: Albedo color: B - アルベドカラー：B
	/// DESC: Albedo color: B - アルベドの色：B
	pub diffuseColorB:u8,

	/// NAME: Padding - パディング
	/// DESC: Padding - パディング
	pub pad_03:[u8;1],

	/// NAME: Reflect color: R - リフレクカラー：R
	/// DESC: Reflect color: R - リフレクの色：R
	pub reflecColorR:u8,

	/// NAME: Reflect color: G - リフレクカラー：G
	/// DESC: Reflect color: G - リフレクの色：G
	pub reflecColorG:u8,

	/// NAME: Reflect color: B - リフレクカラー：B
	/// DESC: Reflect color: B - リフレクの色：B
	pub reflecColorB:u8,

	/// NAME: Is the life effective? - 寿命が有効か
	/// DESC: Is the life effective? - 寿命が有効か
	pub bLifeEnable:u8,

	/// NAME: The strength of shinyness - シャイニネスの強さ
	/// DESC: The strength of shinyness - シャイニネスの強さ
	pub siniScale:f32,

	/// NAME: Lifespan [seconds] - 寿命[秒]
	/// DESC: Lifespan [seconds] (Time after decal is applied, fade-in time does not matter) - 寿命[秒]（デカールが貼られてからの時間、フェードイン時間は関係ない）
	pub lifeTimeSec:f32,

	/// NAME: Fade out time [seconds] - フェードアウト時間[秒]
	/// DESC: Fade out time [seconds] - フェードアウト時間[秒]
	pub fadeOutTimeSec:f32,

	/// NAME: priority - 優先度
	/// DESC: The larger this value, the easier it is to remain (-1 does not disappear) - この値が大きいほど残りやすい（-1は消滅しない）
	pub priority:i16,

	/// NAME: If there is a decal nearby, will it be thinned out? - 近くにデカールがあれば間引くか
	/// DESC: Whether to thin out if there is a decal nearby - 近くにデカールがあれば間引くかどうか
	pub bDistThinOutEnable:u8,

	/// NAME: Fix random pattern - ランダムパターンを固定化する
	/// DESC: If you select "Yes", one variation number randomly determined for each texture other than 0 will be applied. The number of non-zero variations must be the same. - 「はい」にすると、各バリエーション数が0以外のテクスチャについてランダムに決めた一つのバリエーション番号が適用されるようになります。0以外の各バリエーション数は同じ値に揃える必要があります。
	pub bAlignedTexRandomVariationEnable:u8,

	/// NAME: Candidates for thinning within this distance - この距離以内なら間引き候補
	/// DESC: Candidates for thinning if there is a decal within this distance - この距離以内にデカールがあれば間引き候補
	pub distThinOutCheckDist:f32,

	/// NAME: Candidates for thinning if the difference in direction is within this angle [degrees] - 方向の差がこの角度[度]以内なら間引き候補
	/// DESC: Candidates for thinning if the difference in decal direction is within this angle - デカールの方向の差がこの角度以内なら間引き候補
	pub distThinOutCheckAngleDeg:f32,

	/// NAME: If the number of satisfied conditions is more than this number, thin out - 条件を満たした数がこの数以上なら間引く
	/// DESC: If the distance and angle are more than this number, thin out - 距離、角度がこの数以上なら間引く
	pub distThinOutMaxNum:u8,

	/// NAME: How many recent thinnings to check - 直近何個まで間引きチェックするか
	/// DESC: How many recent thinning candidates to check - 間引き候補を直近何個まで調べるか
	pub distThinOutCheckNum:u8,

	/// NAME: Delay frame until it occurs [frame (30FPS conversion)] - 発生するまでの遅延フレーム[フレーム（30FPS換算）]
	/// DESC: After trying to paste the decal, it will actually be pasted after this frame - デカールを貼り付けようとしてからこのフレーム後に実際に貼り付けられる
	pub delayAppearFrame:i16,

	/// NAME: Number of albedo variations - アルベド・バリエーション数
	/// DESC: Number of random variations of albedo texture (including 0th, 2 for 2 textures) - アルベドテクスチャのランダムバリエーション数（0番目を含む、2でテクスチャ2枚分）
	pub Bitfield2:u32,

	/// NAME: Fade-in time [seconds] - フェードイン時間[秒]
	/// DESC: Fade-in time [seconds] - フェードイン時間[秒]
	pub fadeInTimeSec:f32,

	/// NAME: Decimation: Duplicate multiplication value - 間引き:重複乗算値
	/// DESC: Determine if the decal size is duplicated by multiplying it by this value. - デカールサイズにこの値を乗算した範囲で重複かを判定する
	pub thinOutOverlapMultiRadius:f32,

	/// NAME: Decimation: Neighborhood addition distance [m] - 間引き:近隣加算距離[m]
	/// DESC: Judge whether it is a neighborhood within the range of adding this distance [m] to the decal size. - デカールサイズにこの距離[m]を加算した範囲で近隣かを判定する
	pub thinOutNeighborAddRadius:f32,

	/// NAME: Decimation: Overlapping limit - 間引き:重複限界数
	/// DESC: Limit number that can be duplicated - 重複可能な限界数
	pub thinOutOverlapLimitNum:u32,

	/// NAME: Decimation: Neighborhood limit - 間引き:近隣限界数
	/// DESC: Maximum number of neighbors - 近隣可能な限界数
	pub thinOutNeighborLimitNum:u32,

	/// NAME: Thinning mode - 間引きモード
	/// DESC: Thinning mode - 間引きモード
	pub thinOutMode:i8,

	/// NAME: Emissive color: R - エミッシブカラー：R
	/// DESC: Emissive color: R - エミッシブの色：R
	pub emissiveColorR:u8,

	/// NAME: Emissive color: G - エミッシブカラー：G
	/// DESC: Emissive color: G - エミッシブの色：G
	pub emissiveColorG:u8,

	/// NAME: Emissive color: B - エミッシブカラー：B
	/// DESC: Emissive color: B - エミッシブの色：B
	pub emissiveColorB:u8,

	/// NAME: SFX generation upper limit angle - SFX発生上限角度
	/// DESC: SFX generation upper limit angle - SFX発生上限角度
	pub maxDecalSfxCreatableSlopeAngleDeg:f32,

	/// NAME: Padding - パディング
	/// DESC: Padding - パディング
	pub pad_02:[u8;40],
}

impl DECAL_PARAM_ST {
	/// Padding - パディング
	/// Bitfield1
	pub fn get_pad_10(&self) -> u32 {
		&self.Bitfield1 & 0x78
	}

	/// Bitfield1 MAX: 15
	pub fn set_pad_10(&mut self, state: u32) {
		if state != 0 {
			let val = (state << 3) & 0x78;
			let newVal = &self.Bitfield1 & 0xFFFFFF87 | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0xFFFFFF87
		}
	}	/// When it is generated by an attack hit, 1 changes the texture depending on the defense material. New Texture ID = Blood Material ID * 10000000 + Original Texture ID - 攻撃のヒットで発生させるときに1で防御材質によってテクスチャを変える。新しいテクスチャID=血材質ID*10000000+元のテクスチャID
	/// Bitfield1
	pub fn get_replaceTextureId_byMaterial(&self) -> bool {
		&self.Bitfield1 & 0x10 != 0
	}

	/// Bitfield1
	pub fn set_replaceTextureId_byMaterial(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x10
		} else {
			self.Bitfield1 &= 0xFFFFFFEF
		}
	}
	/// Damipoli Search Location 0: Body 1: Left Hand Weapon 2: Right Hand Weapon - ダミポリ検索場所 0:本体 1:左手武器 2:右手武器
	/// Bitfield1
	pub fn get_dmypolyCategory(&self) -> u32 {
		&self.Bitfield1 & 0x60
	}

	/// Bitfield1 MAX: 3
	pub fn set_dmypolyCategory(&mut self, state: u32) {
		if state != 0 {
			let val = (state << 5) & 0x60;
			let newVal = &self.Bitfield1 & 0xFFFFFF9F | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0xFFFFFF9F
		}
	}	/// Padding - パディング
	/// Bitfield1
	pub fn get_pad_05(&self) -> u32 {
		&self.Bitfield1 & 0x780
	}

	/// Bitfield1 MAX: 15
	pub fn set_pad_05(&mut self, state: u32) {
		if state != 0 {
			let val = (state << 7) & 0x780;
			let newVal = &self.Bitfield1 & 0xFFFFF87F | val;
			self.Bitfield1 = newVal
		} else {
			self.Bitfield1 &= 0xFFFFF87F
		}
	}	/// Acts as a deferred decal at 1 - 1でデファードデカールとして機能する
	/// Bitfield1
	pub fn get_useDeferredDecal(&self) -> bool {
		&self.Bitfield1 & 0x800 != 0
	}

	/// Bitfield1
	pub fn set_useDeferredDecal(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x800
		} else {
			self.Bitfield1 &= 0xFFFFF7FF
		}
	}
	/// Acts as a paint decal at 1 - 1でペイントデカールとして機能する
	/// Bitfield1
	pub fn get_usePaintDecal(&self) -> bool {
		&self.Bitfield1 & 0x1000 != 0
	}

	/// Bitfield1
	pub fn set_usePaintDecal(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x1000
		} else {
			self.Bitfield1 &= 0xFFFFEFFF
		}
	}
	/// Affected by optional bloody expression, ID is +1000 in mild, do not paste if hidden - オプションの流血表現の影響を受けるか、マイルドでIDが+1000される、非表示だと貼り付けない
	/// Bitfield1
	pub fn get_bloodTypeEnable(&self) -> bool {
		&self.Bitfield1 & 0x2000 != 0
	}

	/// Bitfield1
	pub fn set_bloodTypeEnable(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x2000
		} else {
			self.Bitfield1 &= 0xFFFFDFFF
		}
	}
	/// 1 if normal component is used (compatible with normal and shinyness texture integration) - ノーマル成分を使用するなら1（ノーマルとシャイニネスのテクスチャ統合対応）
	/// Bitfield1
	pub fn get_bUseNormal(&self) -> bool {
		&self.Bitfield1 & 0x4000 != 0
	}

	/// Bitfield1
	pub fn set_bUseNormal(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x4000
		} else {
			self.Bitfield1 &= 0xFFFFBFFF
		}
	}
	/// Padding - パディング
	/// Bitfield1
	pub fn get_pad_08(&self) -> bool {
		&self.Bitfield1 & 0x8000 != 0
	}

	/// Bitfield1
	pub fn set_pad_08(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x8000
		} else {
			self.Bitfield1 &= 0xFFFF7FFF
		}
	}
	/// Padding - パディング
	/// Bitfield1
	pub fn get_pad_09(&self) -> bool {
		&self.Bitfield1 & 0x10000 != 0
	}

	/// Bitfield1
	pub fn set_pad_09(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x10000
		} else {
			self.Bitfield1 &= 0xFFFEFFFF
		}
	}
	/// Whether to enable POM - POMを有効にするか
	/// Bitfield1
	pub fn get_usePom(&self) -> bool {
		&self.Bitfield1 & 0x20000 != 0
	}

	/// Bitfield1
	pub fn set_usePom(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x20000
		} else {
			self.Bitfield1 &= 0xFFFDFFFF
		}
	}
	/// Whether to update emissive - エミッシブを更新するか
	/// Bitfield1
	pub fn get_useEmissive(&self) -> bool {
		&self.Bitfield1 & 0x40000 != 0
	}

	/// Bitfield1
	pub fn set_useEmissive(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x40000
		} else {
			self.Bitfield1 &= 0xFFFBFFFF
		}
	}
	/// Whether to paste vertically - 垂直に貼り付けるか
	/// Bitfield1
	pub fn get_putVertical(&self) -> bool {
		&self.Bitfield1 & 0x80000 != 0
	}

	/// Bitfield1
	pub fn set_putVertical(&mut self, state: bool) {
		if state {
			self.Bitfield1 |= 0x80000
		} else {
			self.Bitfield1 &= 0xFFF7FFFF
		}
	}
	/// Number of random variations of albedo texture (including 0th, 2 for 2 textures) - アルベドテクスチャのランダムバリエーション数（0番目を含む、2でテクスチャ2枚分）
	/// Bitfield2
	pub fn get_randVaria_Diffuse(&self) -> u32 {
		&self.Bitfield2 & 0x78
	}

	/// Bitfield2 MAX: 15
	pub fn set_randVaria_Diffuse(&mut self, state: u32) {
		if state != 0 {
			let val = (state << 3) & 0x78;
			let newVal = &self.Bitfield2 & 0xFFFFFF87 | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0xFFFFFF87
		}
	}	/// Number of random variations of mask texture (including 0th, 2 for 2 textures) - マスクテクスチャのランダムバリエーション数（0番目を含む、2でテクスチャ2枚分）
	/// Bitfield2
	pub fn get_randVaria_Mask(&self) -> u32 {
		&self.Bitfield2 & 0xF0
	}

	/// Bitfield2 MAX: 15
	pub fn set_randVaria_Mask(&mut self, state: u32) {
		if state != 0 {
			let val = (state << 4) & 0xF0;
			let newVal = &self.Bitfield2 & 0xFFFFFF0F | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0xFFFFFF0F
		}
	}	/// Number of random variations of reflex textures (including 0th, 2 for 2 textures) - リフレクテクスチャのランダムバリエーション数（0番目を含む、2でテクスチャ2枚分）
	/// Bitfield2
	pub fn get_randVaria_Reflec(&self) -> u32 {
		&self.Bitfield2 & 0xF00
	}

	/// Bitfield2 MAX: 15
	pub fn set_randVaria_Reflec(&mut self, state: u32) {
		if state != 0 {
			let val = (state << 8) & 0xF00;
			let newVal = &self.Bitfield2 & 0xFFFFF0FF | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0xFFFFF0FF
		}
	}	/// 
	/// Bitfield2
	pub fn get_pad_12(&self) -> u32 {
		&self.Bitfield2 & 0xF000
	}

	/// Bitfield2 MAX: 15
	pub fn set_pad_12(&mut self, state: u32) {
		if state != 0 {
			let val = (state << 12) & 0xF000;
			let newVal = &self.Bitfield2 & 0xFFFF0FFF | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0xFFFF0FFF
		}
	}	/// Number of random variations of normal texture (including 0th, 2 for 2 textures) - ノーマルテクスチャのランダムバリエーション数（0番目を含む、2でテクスチャ2枚分）
	/// Bitfield2
	pub fn get_randVaria_Normal(&self) -> u32 {
		&self.Bitfield2 & 0xF0000
	}

	/// Bitfield2 MAX: 15
	pub fn set_randVaria_Normal(&mut self, state: u32) {
		if state != 0 {
			let val = (state << 16) & 0xF0000;
			let newVal = &self.Bitfield2 & 0xFFF0FFFF | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0xFFF0FFFF
		}
	}	/// Number of random variations of height texture (including 0th, 2 for 2 textures) - ハイトテクスチャのランダムバリエーション数（0番目を含む、2でテクスチャ2枚分）
	/// Bitfield2
	pub fn get_randVaria_Height(&self) -> u32 {
		&self.Bitfield2 & 0xF00000
	}

	/// Bitfield2 MAX: 15
	pub fn set_randVaria_Height(&mut self, state: u32) {
		if state != 0 {
			let val = (state << 20) & 0xF00000;
			let newVal = &self.Bitfield2 & 0xFF0FFFFF | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0xFF0FFFFF
		}
	}	/// Number of random variations of emissive textures (including 0th, 2 for 2 textures) - エミッシブテクスチャのランダムバリエーション数（0番目を含む、2でテクスチャ2枚分）
	/// Bitfield2
	pub fn get_randVaria_Emissive(&self) -> u32 {
		&self.Bitfield2 & 0xF000000
	}

	/// Bitfield2 MAX: 15
	pub fn set_randVaria_Emissive(&mut self, state: u32) {
		if state != 0 {
			let val = (state << 24) & 0xF000000;
			let newVal = &self.Bitfield2 & 0xF0FFFFFF | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0xF0FFFFFF
		}
	}	/// Padding - パディング
	/// Bitfield2
	pub fn get_pad_11(&self) -> u32 {
		&self.Bitfield2 & 0xF0000000
	}

	/// Bitfield2 MAX: 15
	pub fn set_pad_11(&mut self, state: u32) {
		if state != 0 {
			let val = (state << 28) & 0xF0000000;
			let newVal = &self.Bitfield2 & 0xFFFFFFF | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0xFFFFFFF
		}
	}
}
