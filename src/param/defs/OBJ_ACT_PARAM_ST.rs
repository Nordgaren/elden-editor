/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct OBJ_ACT_PARAM_ST {
    /// NAME: MsgID when action is enabled - アクション有効時のMsgID
    /// DESC: The MsgID of the menu displayed when the action is enabled. - アクションが有効時に表示するメニューのMsgIDです。
    pub actionEnableMsgId: i32,

    /// NAME: MsgID when action fails - アクション失敗時のMsgID
    /// DESC: The MsgID of the menu to display when the action fails. - アクションが失敗時に表示するメニューのMsgIDです。
    pub actionFailedMsgId: i32,

    /// NAME: Event flags for special condition paths - 特殊条件パス用イベントフラグ
    /// DESC: Event flags for unconditionally passing special conditions. - 特殊条件を無条件パスするためのイベントフラグ.
    pub spQualifiedPassEventFlag: u32,

    /// NAME: Player anime ID 0 - プレイヤのアニメID0
    /// DESC: This is the animation ID for the action of the player character. - プレイヤーキャラのアクション時のアニメIDです。
    pub playerAnimId: u32,

    /// NAME: Character anime ID0 - キャラのアニメID0
    /// DESC: Anime ID at the time of action such as enemy - 敵などのアクション時のアニメID
    pub chrAnimId: u32,

    /// NAME: Effective distance of action [cm] - アクションの有効距離[cm]
    /// DESC: The effective distance of the action. - アクションの有効距離です。
    pub validDist: u16,

    /// NAME: Special condition ID - 特殊条件のID
    /// DESC: Special condition ID - 特殊条件のID
    pub spQualifiedId: u16,

    /// NAME: Special condition ID 2 - 特殊条件のID 2
    /// DESC: Special condition ID part 2 - 特殊条件のIDその2
    pub spQualifiedId2: u16,

    /// NAME: Damipoli ID0 of the object - オブジェのダミポリID0
    /// DESC: Damipoli ID that is the action position of the object - オブジェクトのアクション位置となるダミポリIDです
    pub objDummyId: u8,

    /// NAME: Whether to synchronize event kicks - イベントキックを同期させるか
    /// DESC: ObjAct Whether to synchronize the events used in the execution judgment. Set to basic ×. It may be set to ○ only when the actor is not important. - ObjAct実行判定で使用されるイベントを同期させるか。基本×に設定する。アクターが重要ではない場合のみ○に設定しても良い。
    pub isEventKickSync: u8,

    /// NAME: Object Anime ID0 - オブジェのアニメID0
    /// DESC: This is the animation ID for the action of the object. - オブジェクトのアクション時のアニメＩＤです。
    pub objAnimId: u32,

    /// NAME: Effective angle of player action - プレイヤのアクション有効角度
    /// DESC: The effective angle of the player's action. Effective angle difference between the player's orientation vector and the orientation vector to the object - プレイヤのアクションの有効角度です。プレイヤの向きベクトルとオブジェへの方向ベクトルの有効角度差
    pub validPlayerAngle: u8,

    /// NAME: Special condition type - 特殊条件のタイプ
    /// DESC: Types of special conditions - 特殊条件の種類
    pub spQualifiedType: u8,

    /// NAME: Special condition type 2 - 特殊条件のタイプ2
    /// DESC: Type of special condition 2 - 特殊条件の種類2
    pub spQualifiedType2: u8,

    /// NAME: Effective angle of action of the object - オブジェのアクション有効角度
    /// DESC: The action effective angle of the object. Effective angle difference between the action vector of the object and the character vector - オブジェのアクション有効角度です。オブジェのアクションベクトルとキャラベクトルの有効角度差
    pub validObjAngle: u8,

    /// NAME: Character adsorption type - キャラの吸着タイプ
    /// DESC: It is a method of adsorbing characters during object action. - オブジェアクション時のキャラの吸着方法です
    pub chrSorbType: u8,

    /// NAME: Event activation timing - イベント発動タイミング
    /// DESC: Event execution timing - イベントの実行タイミング
    pub eventKickTiming: u8,

    /// NAME: pad1 - pad1
    pub pad1: [u8; 2],

    /// NAME: Action button parameter ID - アクションボタンパラメータID
    /// DESC: Action button parameter ID - アクションボタンパラメータID
    pub actionButtonParamId: i32,

    /// NAME: Treasure activation delay (seconds) - 宝有効化ディレイ(秒)
    /// DESC: The number of seconds from the execution of the object action to the activation of the treasure. A setting exclusively for the object type "treasure box" of the object act. - オブジェアクション実行から宝有効化までの秒数。オブジェアクトのオブジェタイプ「宝箱」専用の設定。
    pub enableTreasureDelaySec: f32,

    /// NAME: Damipoly ID for SFX before execution - 実行前SFX用ダミポリID
    /// DESC: Issue SFX from this Damipoli ID before executing the object. If it is -1, start from the origin. - オブジェアクト実行前のときにこのダミポリIDからSFXを出す。-1なら原点から出す。
    pub preActionSfxDmypolyId: i32,

    /// NAME: Pre-execution SFXID - 実行前SFXID
    /// DESC: SFX issued before executing the object. If it is -1, it will not be issued. - オブジェアクト実行前のときに出すSFX。-1なら出さない。
    pub preActionSfxId: i32,

    /// NAME: pad2 - pad2
    pub pad2: [u8; 40],
}

impl Paramdef for OBJ_ACT_PARAM_ST {
    const NAME: &'static str = "OBJ_ACT_PARAM_ST";
    const VERSION: u16 = 2;
}
