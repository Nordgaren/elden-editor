/* This file was automatically generated from XML paramdefs. */
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct MENU_OFFSCR_REND_PARAM_ST {

	/// NAME: Camera gaze point X - カメラ注視点X
	/// DESC: Camera gaze point X - カメラ注視点X
	pub camAtPosX:f32,

	/// NAME: Camera gaze point Y - カメラ注視点Y
	/// DESC: Camera gaze point Y - カメラ注視点Y
	pub camAtPosY:f32,

	/// NAME: Camera gaze point Z - カメラ注視点Z
	/// DESC: Camera gaze point Z - カメラ注視点Z
	pub camAtPosZ:f32,

	/// NAME: Camera distance - カメラ距離
	/// DESC: Camera distance - カメラ距離
	pub camDist:f32,

	/// NAME: Camera oriented X - カメラ向きX
	/// DESC: Camera oriented X - カメラ向きX
	pub camRotX:f32,

	/// NAME: Suitable for camera Y - カメラ向きY
	/// DESC: Suitable for camera Y - カメラ向きY
	pub camRotY:f32,

	/// NAME: Camera angle of view - カメラ画角
	/// DESC: Camera angle of view - カメラ画角
	pub camFov:f32,

	/// NAME: Shortest distance when operating the camera - カメラ操作時最短距離
	/// DESC: Shortest distance when operating the camera - カメラ操作時最短距離
	pub camDistMin:f32,

	/// NAME: Longest distance when operating the camera - カメラ操作時最長距離
	/// DESC: Longest distance when operating the camera - カメラ操作時最長距離
	pub camDistMax:f32,

	/// NAME: Minimum orientation when operating the camera - カメラ操作時最小向き
	/// DESC: Minimum orientation when operating the camera - カメラ操作時最小向き
	pub camRotXMin:f32,

	/// NAME: Maximum orientation when operating the camera - カメラ操作時最大向き
	/// DESC: Maximum orientation when operating the camera - カメラ操作時最大向き
	pub camRotXMax:f32,

	/// NAME: GparamID - GparamID
	/// DESC: GparamID - GparamID
	pub GparamID:u32,

	/// NAME: Environment map texture ID - 環境マップテクスチャID
	/// DESC: Environment map texture ID. It corresponds to 4 digits of N: \ GR \ data \ Other \ SysEnvTex \ GILM ???? _rem.dds. - 環境マップテクスチャID。N:\GR\data\Other\SysEnvTex\GILM????_rem.dds の数字4桁に対応しています。
	pub envTexId:u32,

	/// NAME: Gparam ID (for PS4) - GparamID(PS4用)
	/// DESC: Gparam ID (for PS4) - GparamID(PS4用)
	pub Grapm_ID_forPS4:u32,

	/// NAME: Gparam ID (for Xbox One) - GparamID(XboxOne用)
	/// DESC: Gparam ID (for Xbox One) - GparamID(XboxOne用)
	pub Grapm_ID_forXB1:u32,

	/// NAME: reserve - 予約
	/// DESC: reserve - 予約
	pub pad:[u8;4],
}

