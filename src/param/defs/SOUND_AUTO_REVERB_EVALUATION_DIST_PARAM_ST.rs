/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST {

	/// NAME: Distance to No Hit [m] - NoHitとする距離[m]
	/// DESC: Collision points above this distance [m] are judged as No Hit (less than 0: invalid) - この距離[m]以上の衝突点をNoHitとして判定する(0より小さい:無効)
	pub NoHitDist:f32,

	/// NAME: Do you include NoHit collision points? - NoHitの衝突点含めるか？
	/// DESC: Do you include NoHit collision points? - NoHitの衝突点含めるか？
	pub isCollectNoHitPoint:u8,

	/// NAME: Do you include outdoor collision points? - 屋外の衝突点含めるか？
	/// DESC: Do you include outdoor collision points? - 屋外の衝突点含めるか？
	pub isCollectOutdoorPoint:u8,

	/// NAME: Do you include floor collision points? - 床の衝突点含めるか？
	/// DESC: Do you include floor collision points? - 床の衝突点含めるか？
	pub isCollectFloorPoint:u8,

	/// NAME: Evaluation distance calculation type - 評価距離計算タイプ
	/// DESC: Evaluation distance calculation type (0: normal, average from 1: Max) - 評価距離計算タイプ(0:通常,1:Maxからの平均)
	pub distValCalcType:u8,

	/// NAME: Effective collision point life [seconds] - 有効な衝突点寿命[秒]
	/// DESC: Collision points longer than this life [second] are treated as invalid. Set to be less than or equal to the life of the common setting - この寿命[秒]以上の衝突点は無効扱いにする。共通設定の寿命以下に設定すること
	pub enableLifeTime:f32,

	/// NAME: Max Number of collision point records - Max衝突点レコード数
	/// DESC: Max Number of collision point records - Max衝突点レコード数
	pub maxDistRecordNum:u32,

	/// NAME: Max distance decimation - Max距離間引き数
	/// DESC: Max distance decimation number (0: not decimation) (must be set to the value of "Max collision point record number-1". Incorrect value will be corrected) - Max距離間引き数(0:間引かない)(「Max衝突点レコード数-1」の値に設定する必要がある。不正な値は修正される)
	pub ignoreDistNumForMax:u32,
}

