/* This file was automatically generated from XML paramdefs. */
/// Data Version: 4
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct SOUND_CHR_PHYSICS_SE_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Ground contact SEID - 地面接触SEID
	/// DESC: SEID pronounced when contacting the ground after death. (-1: Invalid). SEID category is fixed to c - 死亡後、地面と接触したときに発音するSEID。(-1：無効)。SEIDカテゴリーはc固定
	pub ContactLandSeId:i32,

	/// NAME: Ground contact additional SEID (for material) - 地面接触追加SEID(材質用)
	/// DESC: Additional SEID (for material) that sounds when it comes into contact with the ground after death. (-1: Invalid). SEID category is fixed to c - 死亡後、地面と接触したときに発音する追加SEID(材質用)。(-1：無効)。SEIDカテゴリーはc固定
	pub ContactLandAddSeId:i32,

	/// NAME: Number of ground contact pronunciations - 地面接触発音回数
	/// DESC: Number of pronunciations when touching the ground after death - 死亡後、地面接触時に発音する回数
	pub ContactLandPlayNum:i32,

	/// NAME: Do you count the number of ground contact sounds in rigid body units? - 地面接触発音回数を剛体単位でカウントするか？
	/// DESC: Do you count the number of pronunciations of surface contact after death in rigid body units? (○: Rigid body unit, ×: Character unit) - 死亡後地、面接触発音回数を剛体単位でカウントするか？(〇：剛体単位、×：キャラ単位)
	pub IsEnablePlayCountPerRigid:u8,

	/// NAME: pad - pad
	pub pad:[u8;3],

	/// NAME: Ground contact minimum impulse value - 地面接触最小力積値
	/// DESC: Minimum impulse value required for ground contact determination after death - 死亡後、地面接触判定に必要な最小力積値
	pub ContactLandMinImpuse:f32,

	/// NAME: Ground contact minimum velocity value - 地面接触最小速度値
	/// DESC: Minimum speed value required for ground contact determination after death - 死亡後、地面接触判定に必要な最小速度値
	pub ContactLandMinSpeed:f32,

	/// NAME: Player contact SEID - Player接触SEID
	/// DESC: SEID that sounds when you come into contact with Player after death. (-1: Invalid). SEID category is fixed to c - 死亡後、Playerと接触したときに鳴らすSEID。(-1：無効)。SEIDカテゴリーはc固定
	pub ContactPlayerSeId:i32,

	/// NAME: Player minimum contact impulse value - Player接触最小力積値
	/// DESC: Minimum impulse value required for Player contact judgment after death - 死亡後、Player接触判定に必要な最小力積値
	pub ContactPlayerMinImpuse:f32,

	/// NAME: Player minimum contact speed value - Player接触最小速度値
	/// DESC: Minimum speed value required for Player contact judgment after death - 死亡後、Player接触判定に必要な最小速度値
	pub ContactPlayerMinSpeed:f32,

	/// NAME: Contact judgment rigid body IDX0 - 接触判定剛体IDX0
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx0:i8,

	/// NAME: Contact judgment rigid body IDX1 - 接触判定剛体IDX1
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx1:i8,

	/// NAME: Contact judgment rigid body IDX2 - 接触判定剛体IDX2
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx2:i8,

	/// NAME: Contact judgment rigid body IDX3 - 接触判定剛体IDX3
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx3:i8,

	/// NAME: Contact judgment rigid body IDX4 - 接触判定剛体IDX4
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx4:i8,

	/// NAME: Contact judgment rigid body IDX5 - 接触判定剛体IDX5
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx5:i8,

	/// NAME: Contact judgment rigid body IDX6 - 接触判定剛体IDX6
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx6:i8,

	/// NAME: Contact judgment rigid body IDX7 - 接触判定剛体IDX7
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx7:i8,

	/// NAME: Contact judgment rigid body IDX8 - 接触判定剛体IDX8
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx8:i8,

	/// NAME: Contact judgment rigid body IDX9 - 接触判定剛体IDX9
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx9:i8,

	/// NAME: Contact judgment rigid body IDX10 - 接触判定剛体IDX10
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx10:i8,

	/// NAME: Contact judgment rigid body IDX11 - 接触判定剛体IDX11
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx11:i8,

	/// NAME: Contact judgment rigid body IDX12 - 接触判定剛体IDX12
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx12:i8,

	/// NAME: Contact judgment rigid body IDX13 - 接触判定剛体IDX13
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx13:i8,

	/// NAME: Contact judgment rigid body IDX14 - 接触判定剛体IDX14
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx14:i8,

	/// NAME: Contact judgment rigid body IDX15 - 接触判定剛体IDX15
	/// DESC: Specify the INDEX of the rigid body for contact judgment. Specify only the rigid body to which you want to attach SE - 接触判定する剛体のINDEXを指定します。SEを付けたい剛体だけ指定します
	pub ContactCheckRigidIdx15:i8,
}

impl SOUND_CHR_PHYSICS_SE_PARAM_ST {
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
	}
}
