/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 0
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct AUTO_CREATE_ENV_SOUND_PARAM_ST {
    /// NAME: Appearance distance Min [m] - 出現距離Min[m]
    /// DESC: Appearance distance Min [m] - 出現距離Min[m]
    pub RangeMin: f32,

    /// NAME: Appearance distance Max [m] - 出現距離Max[m]
    /// DESC: Appearance distance Max [ - 出現距離Max[
    pub RangeMax: f32,

    /// NAME: Lifespan Min [seconds] - 寿命Min[秒]
    /// DESC: Lifespan Min [seconds] - 寿命Min[秒]
    pub LifeTimeMin: f32,

    /// NAME: Lifespan Max [seconds] - 寿命Max[秒]
    /// DESC: Lifespan Max [seconds] - 寿命Max[秒]
    pub LifeTimeMax: f32,

    /// NAME: Delete distance [m] - 削除距離[m]
    /// DESC: Delete distance [m] - 削除距離[m]
    pub DeleteDist: f32,

    /// NAME: Neighborhood judgment distance [m] - 近傍判定距離[m]
    /// DESC: Neighborhood judgment distance [m] - 近傍判定距離[m]
    pub NearDist: f32,

    /// NAME: Generation angle limit Min [degree] - 生成角度制限Min[度]
    /// DESC: Angle limit Min [degree] (Specify the Y-axis angle +-in front of the camera. 180 is omnidirectional) - 角度制限Min[度](カメラの前方のY軸角度+-の指定。180なら全方位)
    pub LimiteRotateMin: f32,

    /// NAME: Generation angle limit Max [degrees] - 生成角度制限Max[度]
    /// DESC: Angle limit Max [degree] (Specify the Y-axis angle +-in front of the camera. 180 is omnidirectional) - 角度制限Max[度](カメラの前方のY軸角度+-の指定。180なら全方位)
    pub LimiteRotateMax: f32,
}

impl Paramdef for AUTO_CREATE_ENV_SOUND_PARAM_ST {
    const NAME: &'static str = "AUTO_CREATE_ENV_SOUND_PARAM_ST";
    const VERSION: u16 = 0;
}
