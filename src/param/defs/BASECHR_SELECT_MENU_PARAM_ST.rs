/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct BASECHR_SELECT_MENU_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: Architype ID: Base character - アーキタイプID：ベースキャラクター
	/// DESC: Specify the initial parameter ID for each architype for which face para is set. - フェイスパラを設定したアーキタイプ別初期パラメータIDを指定する 
	pub chrInitParam:u32,

	/// NAME: Architype ID: Identity - アーキタイプID：素性
	/// DESC: Specify the initial parameter ID for each feature archetype - 素性のアーキタイプ別初期パラメータIDを指定する
	pub originChrInitParam:u32,

	/// NAME: Image ID - イメージID
	/// DESC: Images lined up on the base character selection screen. Specify the number of frames of resources embedded in Fla - ベースキャラクター選択画面に並ぶ画像。Flaに埋め込まれたリソースのフレーム数を指定
	pub imageId:i32,

	/// NAME: Text ID - テキストID
	/// DESC: Occupation name menu text ID - 職業名のメニューテキストID
	pub textId:i32,

	/// NAME: Reserve - リザーブ
	pub reserve:[u8;12],
}

impl BASECHR_SELECT_MENU_PARAM_ST {
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
