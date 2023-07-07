/* This file was automatically generated from XML paramdefs. */
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CHARMAKEMENUTOP_PARAM_ST {

	/// NAME: Command type - コマンドタイプ
	/// DESC: Command type - コマンドの種別
	pub commandType:i32,

	/// NAME: Item text ID - 項目テキストID
	/// DESC: ID of the text to be displayed - 表示するテキストのID
	pub captionId:i32,

	/// NAME: Face Param ID - 顔パラムID
	/// DESC: Face Param ID - 顔パラムID
	pub faceParamId:i32,

	/// NAME: Table ID (male) - テーブルID（男性）
	/// DESC: First ID (male) in the list of items to select. The command type is [Slider: Text ID of minimum label (n) and maximum label (n + 1), Color: ID of color table, Grid or text: First ID of character make list item, Other: Ignore] - 選択するアイテム一覧の先頭ID（男）。コマンドタイプが[スライダー：最小値ラベル(n)と最大値ラベル(n+1)のテキストID、カラー：カラーテーブルのID、グリッド or テキスト：キャラメイクリストアイテムの先頭ID、それ以外：無視]
	pub tableId:i32,

	/// NAME: Display conditions - 表示条件
	/// DESC: Conditions for displaying this command - このコマンドを表示する条件
	pub viewCondition:i32,

	/// NAME: Preview mode - プレビューモード
	/// DESC: Display mode of the character model displayed in preview - プレビュー表示しているキャラクターモデルの表示モード
	pub previewMode:i8,

	/// NAME: reserve - 予約
	pub reserved2:[u8;3],

	/// NAME: Table ID (female) - テーブルID（女性）
	/// DESC: For women with table ID. If -1, refer to for men - テーブルIDの女用。-1なら男用を参照する
	pub tableId2:i32,

	/// NAME: Referenced face Param ID - 参照先の顔パラムID
	/// DESC: Face param ID of the matching destination for "matching to XX" - 「○○に合わせる」用の合わせ先の顔パラムID
	pub refFaceParamId:i32,

	/// NAME: Referenced text ID - 参照先テキストID
	/// DESC: Display text ID for "fit to XX" - 「○○に合わせる」用の表示テキストID
	pub refTextId:i32,

	/// NAME: 1 line help text ID (overwrite) - 1行ヘルプテキストID（上書き）
	/// DESC: 1-line help text ID (-1: Get 1-line help with item text ID). Basically, item text ID = 1 line help text ID, but if you want to overwrite part of it, specify it with this parameter. - 1行ヘルプのテキストID(-1: 項目テキストIDで1行ヘルプを取得する)。基本的に項目テキストID＝1行ヘルプテキストIDになっているが、一部上書きしたいときにこのパラメータで指定する
	pub helpTextId:i32,

	/// NAME: Event flag ID - イベントフラグID
	/// DESC: Event flag to unlock this item (0: invalid value). Invalid value will always be unlocked - この項目をアンロックするイベントフラグ(0:無効値)。無効値なら常にアンロックされる
	pub unlockEventFlagId:u32,

	/// NAME: reserve - 予約
	pub reserved:[u8;4],
}

