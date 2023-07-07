/* This file was automatically generated from XML paramdefs. */
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MULTI_ESTUS_FLASK_BONUS_PARAM_ST {

	/// NAME: host - ホスト
	/// DESC: Number of host est recovery - ホストのエスト回復数
	pub host:u8,

	/// NAME: White sign - 白サイン
	/// DESC: Number of white spirits recovering from the cooperation sign - 協力サインの白霊のエスト回復数
	pub WhiteGhost_None:u8,

	/// NAME: Gold spirit (sun) - 金霊（太陽）
	/// DESC: Number of est recovery of gold spirits of cooperation sign - 協力サインの金霊のエスト回復数
	pub WhiteGhost_Umbasa:u8,

	/// NAME: White berserker - 白バーサーカー
	/// DESC: Number of recovery of white Berserker's est of cooperation sign - 協力サインの白バーサーカーのエスト回復数
	pub WhiteGhost_Berserker:u8,

	/// NAME: Red sign - 赤サイン
	/// DESC: Number of est recovery of red spirits of hostile sign - 敵対サインの赤霊のエスト回復数
	pub BlackGhost_None_Sign:u8,

	/// NAME: Red gold spirit (signature) - 赤金霊（サイン）
	/// DESC: Number of est recovery of red gold spirit of hostile sign - 敵対サインの赤金霊のエスト回復数
	pub BlackGhost_Umbasa_Sign:u8,

	/// NAME: Red berserker (sign) - 赤バーサーカー（サイン）
	/// DESC: Number of est recovery of red Berserker of hostile sign - 敵対サインの赤バーサーカーのエスト回復数
	pub BlackGhost_Berserker_Sign:u8,

	/// NAME: Invasion - 侵入
	/// DESC: Number of intrusion est recovery - 侵入のエスト回復数
	pub BlackGhost_None_Invade:u8,

	/// NAME: Red Gold Spirit (Invasion) - 赤金霊（侵入）
	/// DESC: Number of est recovery of red gold spirits of invading orbs - 侵入オーブの赤金霊のエスト回復数
	pub BlackGhost_Umbasa_Invade:u8,

	/// NAME: Red berserker (invasion) - 赤バーサーカー（侵入）
	/// DESC: Invasion Orb Red Berserker Est Recovery Number - 侵入オーブの赤バーサーカーのエスト回復数
	pub BlackGhost_Berserker_Invade:u8,

	/// NAME: Relief guest - 救援ゲスト
	/// DESC: Number of rescue guests' est recovery - 救援ゲストのエスト回復数
	pub RedHunter1:u8,

	/// NAME: Red Scare Spirit 2 - 赤狩り霊２
	/// DESC: Number of est recovery of Red Scare Spirit 2 - 赤狩り霊２のエスト回復数
	pub RedHunter2:u8,

	/// NAME: Map Guardian Spirit (Forest) - マップ守護霊(森)
	/// DESC: Map guardian spirit (forest) est recovery number - マップ守護霊（森）のエスト回復数
	pub GuardianOfForest:u8,

	/// NAME: Map Guardian (Anor) - マップ守護霊(アノール)
	/// DESC: Map Guardian Spirit (Anor) Est Recovery Number - マップ守護霊(アノール)のエスト回復数
	pub GuardianOfAnor:u8,

	/// NAME: Arena - 闘技場
	/// DESC: Number of est recovery in the arena - 闘技場のエスト回復数
	pub BattleRoyal:u8,

	/// NAME: Yellow robe's old man - 黄衣の翁
	/// DESC: The number of est recovery of the old man in yellow - 黄衣の翁のエスト回復数
	pub YellowMonk:u8,

	/// NAME: pad - pad
	pub pad1:[u8;48],
}

