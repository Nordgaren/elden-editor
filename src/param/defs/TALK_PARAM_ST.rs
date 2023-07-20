/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 4
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct TALK_PARAM_ST {

	/// NAME: Do you remove it from the NT version output? - NT版出力から外すか
	/// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
	pub Bitfield1:u8,

	/// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
	/// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
	pub disableParamReserve2:[u8;3],

	/// NAME: PC Gender is Male: Message ID - PC性別が男：メッセージID
	/// DESC: PC Gender is Male: Specify Message-> Menu - PC性別が男：メッセージを指定->メニュー
	pub msgId:i32,

	/// NAME: PC Gender is Male: Voice ID - PC性別が男：ボイスID
	/// DESC: PC Gender is Male: Specify Voice-> Sound - PC性別が男：ボイスを指定->サウンド
	pub voiceId:i32,

	/// NAME: Special effect ID0 - 特殊効果ID0
	/// DESC: Specify special effects-> Character - 特殊効果を指定->キャラ
	pub spEffectId0:i32,

	/// NAME: Motion ID 0 - モーションID0
	/// DESC: Specify motion-> Character - モーションを指定->キャラ
	pub motionId0:i32,

	/// NAME: Special effect ID1 - 特殊効果ID1
	/// DESC: Specify special effects-> Character - 特殊効果を指定->キャラ
	pub spEffectId1:i32,

	/// NAME: Motion ID 1 - モーションID1
	/// DESC: Specify motion-> Character - モーションを指定->キャラ
	pub motionId1:i32,

	/// NAME: Return position - 復帰位置
	/// DESC: Relative position of returning conversation-> Conversation - 復帰する会話の相対位置->会話
	pub returnPos:i32,

	/// NAME: Reaction ID - リアクションID
	/// DESC: Conversation specification when returning-> Conversation - 復帰時の会話指定->会話
	pub reactionId:i32,

	/// NAME: Event ID - イベントID
	/// DESC: Event ID-> Event - イベントID->イベント
	pub eventId:i32,

	/// NAME: PC Gender is Female: Message - PC性別が女：メッセージ
	/// DESC: PC Gender is Female: Specify Message-> Menu - PC性別が女：メッセージを指定->メニュー
	pub msgId_female:i32,

	/// NAME: PC Gender is Female: Voice ID - PC性別が女：ボイスID
	/// DESC: PC Gender is Female: Specify Voice-> Sound - PC性別が女：ボイスを指定->サウンド
	pub voiceId_female:i32,

	/// NAME: Speaker: Lip-sync start time - 話者：口パク開始時間
	/// DESC: Speaker: Lip-sync start time. -1 with no lip-sync - 話者：口パク開始時間。-1で口パク無し
	pub lipSyncStart:i16,

	/// NAME: Speaker: Lip-sync duration - 話者：口パク継続時間
	/// DESC: Speaker: Lip-sync duration. Lip-sync at -1 continues forever - 話者：口パク継続時間。-1で口パクずっと継続
	pub lipSyncTime:i16,

	/// NAME: Padding - パディング
	pub pad2:[u8;4],

	/// NAME: Voice playback timeout time - ボイス再生タイムアウト時間
	/// DESC: Voice playback timeout period. In case of -1, time-out processing is performed in "NPC conversation voice playback timeout time" of "Common_Game system parameter". - ボイス再生タイムアウト時間。-1の場合は「共通_ゲームシステムパラメータ」の「NPC会話ボイス再生タイムアウト時間」でタイムアウト処理を行う。
	pub timeout:f32,

	/// NAME: Speaker: Subtitled play Anime ID - 話者：字幕芝居アニメID
	/// DESC: Speaker: Animation ID during conversation - 話者：会話中のアニメーションID
	pub talkAnimationId:i32,

	/// NAME: Whether to forcibly display subtitles - 強制的に字幕を表示するか
	/// DESC: Do you want to force the display of subtitles? Display subtitles even when subtitles are turned off as an option - 強制的に字幕を表示するか。オプションで字幕オフでも字幕を表示する
	pub Bitfield2:u8,

	/// NAME: Padding - パディング
	/// DESC: Padding - パディング
	pub pad1:[u8;31],
}

impl Paramdef for TALK_PARAM_ST {
const NAME: &'static str = "TALK_PARAM_ST";
const VERSION: u16 = 4;
}
impl TALK_PARAM_ST {
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
	}	/// Do you want to force the display of subtitles? Display subtitles even when subtitles are turned off as an option - 強制的に字幕を表示するか。オプションで字幕オフでも字幕を表示する
	/// Bitfield2
	pub fn get_isForceDisp(&self) -> bool {
		&self.Bitfield2 & 0x1 != 0
	}

	/// Bitfield2
	pub fn set_isForceDisp(&mut self, state: bool) {
		if state {
			self.Bitfield2 |= 0x1
		} else {
			self.Bitfield2 &= 0xFE
		}
	}
	/// 
	/// Bitfield2
	pub fn get_pad3(&self) -> u8 {
		&self.Bitfield2 & 0xFE
	}

	/// Bitfield2 MAX: 127
	pub fn set_pad3(&mut self, state: u8) {
		if state != 0 {
			let val = (state << 1) & 0xFE;
			let newVal = &self.Bitfield2 & 0x1 | val;
			self.Bitfield2 = newVal
		} else {
			self.Bitfield2 &= 0x1
		}
	}
}
