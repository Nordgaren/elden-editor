/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MISSILE_PARAM_ST {
    /// NAME: FFXID - FFXID
    /// DESC: ID on the FFX editor - ＦＦＸエディタ上のＩＤ
    pub FFXID: i32,

    /// NAME: Survival time [frame] - 生存時間[frame]
    /// DESC: Survival time. - 生存時間。
    pub LifeTime: u16,

    /// NAME: Hit ball radius [cm] - 当たり球半径[cm]
    /// DESC: Hit ball radius. Unit cm - 当たり球半径。単位cm
    pub HitSphereRadius: u16,

    /// NAME: Landing damage - 着弾ダメージ
    /// DESC: Amount of damage at the time of landing - 着弾時のダメージ量
    pub HitDamage: u16,

    /// NAME: reserve - 予約
    pub reserve0: [u8; 6],

    /// NAME: Initial speed [m / s] - 初速度[m/s]
    /// DESC: Initial speed [m / s] - 初速度[m/s]
    pub InitVelocity: f32,

    /// NAME: Range distance - 射程距離
    /// DESC: Range distance - 射程距離
    pub distance: f32,

    /// NAME: Gravity within range - 射程距離内重力
    /// DESC: Gravity within range - 射程距離内重力
    pub gravityInRange: f32,

    /// NAME: Out-of-range gravity - 射程距離外重力
    /// DESC: Out-of-range gravity - 射程距離外重力
    pub gravityOutRange: f32,

    /// NAME: MP consumption - 消費MP
    /// DESC: MP consumption - 消費MP
    pub mp: i32,

    /// NAME: Acceleration within range - 射程距離内加速度
    /// DESC: Acceleration within range - 射程距離内加速度
    pub accelInRange: f32,

    /// NAME: Out-of-range acceleration - 射程距離外加速度
    /// DESC: Out-of-range acceleration - 射程距離外加速度
    pub accelOutRange: f32,

    /// NAME: reserve - 予約
    pub reserve1: [u8; 20],

    /// NAME: Landing ID - 着弾ＩＤ
    /// DESC: Landing ID - 着弾ＩＤ
    pub HitMissileID: u16,

    /// NAME: Will you die at the end of your life? - 寿命で死ぬか？
    /// DESC: Will it use up its life without dying even if it lands? - 着弾しても、死なずに、寿命を使い切るか？
    pub DiedNaturaly: u8,

    /// NAME: Will it land when the life has expired? - 寿命が切れたときに着弾するか
    /// DESC: Will it land when the life has expired? - 寿命が切れたときに着弾するか
    pub ExplosionDie: u8,

    /// NAME: Action ID on hit - ヒット時行動ID
    /// DESC: Action ID given to the opponent when doing damage - ダメージを与えたとき相手に与える行動ID
    pub behaviorId: i32,

    /// NAME: reserve - 予約
    pub reserve_last: [u8; 56],
}

impl Paramdef for MISSILE_PARAM_ST {
    const NAME: &'static str = "MISSILE_PARAM_ST";
    const VERSION: u16 = 1;
}
