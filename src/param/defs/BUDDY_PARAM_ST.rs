/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct BUDDY_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Summon special effect ID - 召喚特殊効果ID
	/// DESC: Set the special effect ID that will be the summoning condition - 召喚条件になる特殊効果IDを設定します 
	pub triggerSpEffectId:i32,

	/// NAME: NPC parameter ID - NPCパラメータID
	/// DESC: Set the NPC parameter ID of the summoned buddy - 召喚されるバディのNPCパラメータIDを設定します
	pub npcParamId:i32,

	/// NAME: Thinking parameter ID - 思考パラメータID
	/// DESC: Sets the NPC Thinking Parameter ID of the summoned buddy - 召喚されるバディのNPC思考パラメータIDを設定します
	pub npcThinkParamId:i32,

	/// NAME: Riding (riding side): NPC parameter ID - 騎乗（乗られる側）：NPCパラメータID
	/// DESC: For buddies that you want to summon while riding, set the NPC parameter ID for the "rider" - 騎乗状態で召喚したいバディの場合、「乗られる側」のNPCパラメータIDを設定します 
	pub npcParamId_ridden:i32,

	/// NAME: Riding (riding side): Thinking parameter ID - 騎乗（乗られる側）：思考パラメータID
	/// DESC: For buddies that you want to summon while riding, set the NPC Thinking Parameter ID for the "rider" - 騎乗状態で召喚したいバディの場合、「乗られる側」のNPC思考パラメータIDを設定します
	pub npcThinkParamId_ridden:i32,

	/// NAME: X: Placement coordinate offset [m] - X：配置座標オフセット[m]
	/// DESC: Sets the distance in meters to offset the buddy from the summon point to the X coordinate - バディを召喚ポイントから、X座標にオフセットする距離をメートル単位で設定します
	pub x_offset:f32,

	/// NAME: Z: Placement coordinate offset [m] - Z：配置座標オフセット[m]
	/// DESC: Sets the distance in meters that offsets the buddy from the summon point to the Z coordinate. - バディを召喚ポイントから、Z座標にオフセットする距離をメートル単位で設定します
	pub z_offset:f32,

	/// NAME: Y: Your placement angle [deg] - Y：自分の配置角度[deg]
	/// DESC: Set the angle to rotate yourself around the Y axis - Y軸を中心に、自分を回転させる角度を設定します
	pub y_angle:f32,

	/// NAME: Will it emerge from around the stone monument? - 石碑周辺から出現するか？
	/// DESC: Will it emerge from around the stone monument? - 石碑周辺から出現するか？
	pub appearOnAroundSekihi:u8,

	/// NAME: Do you want to skip target sharing with your PC? - PCとのターゲット共有をスキップするか？
	/// DESC: Do you want to skip target sharing with your PC? - PCとのターゲット共有をスキップするか？
	pub disablePCTargetShare:u8,

	/// NAME: PC tracking & warp type - PC追従＆ワープタイプ 
	/// DESC: PC tracking & warp type - PC追従＆ワープタイプ 
	pub pcFollowType:u8,

	/// NAME: Reserve - リザーブ
	/// DESC: Reserve - リザーブ
	pub Reserve:[u8;1],

	/// NAME: +0 o'clock doping special effect - +0時ドーピング特殊効果
	/// DESC: +0 o'clock doping special effect - +0時ドーピング特殊効果
	pub dopingSpEffect_lv0:i32,

	/// NAME: +1 o'clock doping special effect - +1時ドーピング特殊効果
	/// DESC: +0 o'clock doping special effect - +0時ドーピング特殊効果
	pub dopingSpEffect_lv1:i32,

	/// NAME: +2 o'clock doping special effect - +2時ドーピング特殊効果
	/// DESC: +0 o'clock doping special effect - +0時ドーピング特殊効果
	pub dopingSpEffect_lv2:i32,

	/// NAME: +3 o'clock doping special effect - +3時ドーピング特殊効果
	/// DESC: +0 o'clock doping special effect - +0時ドーピング特殊効果
	pub dopingSpEffect_lv3:i32,

	/// NAME: +4 o'clock doping special effect - +4時ドーピング特殊効果
	/// DESC: +0 o'clock doping special effect - +0時ドーピング特殊効果
	pub dopingSpEffect_lv4:i32,

	/// NAME: +5 o'clock doping special effect - +5時ドーピング特殊効果
	/// DESC: +0 o'clock doping special effect - +0時ドーピング特殊効果
	pub dopingSpEffect_lv5:i32,

	/// NAME: +6 o'clock doping special effect - +6時ドーピング特殊効果
	/// DESC: +0 o'clock doping special effect - +0時ドーピング特殊効果
	pub dopingSpEffect_lv6:i32,

	/// NAME: +7 o'clock doping special effect - +7時ドーピング特殊効果
	/// DESC: +0 o'clock doping special effect - +0時ドーピング特殊効果
	pub dopingSpEffect_lv7:i32,

	/// NAME: +8 o'clock doping special effect - +8時ドーピング特殊効果
	/// DESC: +0 o'clock doping special effect - +0時ドーピング特殊効果
	pub dopingSpEffect_lv8:i32,

	/// NAME: +9 o'clock doping special effect - +9時ドーピング特殊効果
	/// DESC: +0 o'clock doping special effect - +0時ドーピング特殊効果
	pub dopingSpEffect_lv9:i32,

	/// NAME: +10 o'clock doping special effect - +10時ドーピング特殊効果
	/// DESC: +0 o'clock doping special effect - +0時ドーピング特殊効果
	pub dopingSpEffect_lv10:i32,

	/// NAME: Initial parameter ID by architype - アーキタイプ別初期パラメータID
	/// DESC: Initial parameter ID by architype - アーキタイプ別初期パラメータID
	pub npcPlayerInitParamId:i32,

	/// NAME: Generate Anime ID - ジェネレートアニメID
	/// DESC: Generate Anime ID - ジェネレートアニメID
	pub generateAnimId:i32,

	/// NAME: Unk1
	pub Unk1:u32,

	/// NAME: Unk2
	pub Unk2:u32,

	/// NAME: Unk3
	pub Unk3:i32,

	/// NAME: Unk4
	pub Unk4:i32,

	/// NAME: Unk5
	pub Unk5:i32,

	/// NAME: Unk6
	pub Unk6:i32,

	/// NAME: Unk7
	pub Unk7:i32,

	/// NAME: Unk8
	pub Unk8:i32,

	/// NAME: Unk9
	pub Unk9:i32,

	/// NAME: Unk10
	pub Unk10:i32,

	/// NAME: Unk11
	pub Unk11:u32,

	/// NAME: Unk12
	pub Unk12:i32,

	/// NAME: Unk13
	pub Unk13:i32,

	/// NAME: Unk14
	pub Unk14:i32,

	/// NAME: Unk15
	pub Unk15:i32,

	/// NAME: Unk16
	pub Unk16:i32,

	/// NAME: Unk17
	pub Unk17:u32,
}

impl Paramdef for BUDDY_PARAM_ST {
const NAME: &'static str = "BUDDY_PARAM_ST";
const VERSION: u16 = 2;
}
impl BUDDY_PARAM_ST {
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
