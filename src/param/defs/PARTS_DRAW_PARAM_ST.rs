/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 5
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct PARTS_DRAW_PARAM_ST {
    /// NAME: LOD level 0-1 boundary distance [m] - LODレベル0-1境界距離[m]
    /// DESC: Switching center - 切り替わる中心
    pub lv01_BorderDist: f32,

    /// NAME: LOD level 0-1 play distance [m] - LODレベル0-1遊び距離[m]
    /// DESC: ± play at the center of the boundary - 境界中心で±遊び
    pub lv01_PlayDist: f32,

    /// NAME: LOD level 1-2 boundary distance [m] - LODレベル1-2境界距離[m]
    /// DESC: Switching center - 切り替わる中心
    pub lv12_BorderDist: f32,

    /// NAME: LOD level 1-2 play distance [m] - LODレベル1-2遊び距離[m]
    /// DESC: ± play at the center of the boundary - 境界中心で±遊び
    pub lv12_PlayDist: f32,

    /// NAME: LOD level 2-3 Boundary distance [m] - LODレベル2-3境界距離[m]
    /// DESC: Switching center - 切り替わる中心
    pub lv23_BorderDist: f32,

    /// NAME: LOD level 2-3 play distance [m] - LODレベル2-3遊び距離[m]
    /// DESC: ± play at the center of the boundary - 境界中心で±遊び
    pub lv23_PlayDist: f32,

    /// NAME: LOD level 3-4 Boundary distance [m] - LODレベル3-4境界距離[m]
    /// DESC: Switching center - 切り替わる中心
    pub lv34_BorderDist: f32,

    /// NAME: LOD level 3-4 play distance [m] - LODレベル3-4遊び距離[m]
    /// DESC: ± play at the center of the boundary - 境界中心で±遊び
    pub lv34_PlayDist: f32,

    /// NAME: LOD level 4-5 Boundary distance [m] - LODレベル4-5境界距離[m]
    /// DESC: Switching center - 切り替わる中心
    pub lv45_BorderDist: f32,

    /// NAME: LOD level 4-5 play distance [m] - LODレベル4-5遊び距離[m]
    /// DESC: ± play at the center of the boundary - 境界中心で±遊び
    pub lv45_PlayDist: f32,

    /// NAME: Texture LOD Level 0-1 Boundary distance [m] - TextureLODレベル0-1境界距離[m]
    /// DESC: Texture switching center (Texure LOD disabled at 0) - Texture切り替わる中心(0でTexureLOD無効)
    pub tex_lv01_BorderDist: f32,

    /// NAME: Texture LOD Level 0-1 Play distance [m] - TextureLODレベル0-1遊び距離[m]
    /// DESC: Texture Play around the boundary - Texture境界中心で±遊び
    pub tex_lv01_PlayDist: f32,

    /// NAME: Crossfade enabled - クロスフェード有効
    /// DESC: Is crossfade enabled (0: disabled, 1: enabled)? - クロスフェード有効か(0:無効,1:有効)
    pub Bitfield1: u32,

    /// NAME: Drawing distance [m] - 描画距離[m]
    /// DESC: Maximum drawing distance. In the open, it will be used for the activation distance - 描画最大距離。オープンではアクティベート距離に利用されます
    pub drawDist: f32,

    /// NAME: Fade range [m] - フェード範囲[m]
    /// DESC: Fade distance from the maximum drawing distance to the actual disappearance - 描画最大距離から、実際に消えるまでのフェード距離
    pub drawFadeRange: f32,

    /// NAME: Shadow drawing distance [m] - 影描画距離[m]
    /// DESC: Maximum shadow drawing distance - 影の描画最大距離
    pub shadowDrawDist: f32,

    /// NAME: Shadow fade range [m] - 影フェード範囲[m]
    /// DESC: Fade distance from the maximum shadow drawing distance to the actual disappearance - 影の描画最大距離から、実際に消えるまでのフェード距離
    pub shadowFadeRange: f32,

    /// NAME: Motion blur drawing boundary distance [m] - モーションブラー描画境界距離[m]
    /// DESC: Distance at which motion blur is enabled - モーションブラーが有効になる距離
    pub motionBlur_BorderDist: f32,

    /// NAME: Cast the shadow of a point light source - 点光源の影を落とす
    /// DESC: Cast the shadow of a point light source - 点光源の影を落とす
    pub isPointLightShadowSrc: i8,

    /// NAME: Cast the shadow of a parallel light source - 平行光源の影を落とす
    /// DESC: Cast the shadow of a parallel light source - 平行光源の影を落とす
    pub isDirLightShadowSrc: i8,

    /// NAME: Receive a shadow - 影を受ける
    /// DESC: Receive a shadow - 影を受ける
    pub isShadowDst: i8,

    /// NAME: Shadow drawing only - 影描画のみ
    /// DESC: Shadow drawing only - 影描画のみ
    pub isShadowOnly: i8,

    /// NAME: Reflected - 映り込む
    /// DESC: Reflected - 映り込む
    pub drawByReflectCam: i8,

    /// NAME: Reflection drawing only - 映り込み描画のみ
    /// DESC: Reflection drawing only - 映り込み描画のみ
    pub drawOnlyReflectCam: i8,

    /// NAME: What level of LodMap to include - どのレベルのLodMapまで含めるか
    /// DESC: What level of LodMap to include - どのレベルのLodMapまで含めるか
    pub IncludeLodMapLv: i8,

    /// NAME: Don't FarClip - FarClipしない
    /// DESC: Disable fur clips and always draw at the innermost depth of the clip space. Mainly for celestial sphere - ファークリップを無効にし、常にクリップ空間の一番奥の深度に描画する。主に天球用
    pub isNoFarClipDraw: u8,

    /// NAME: LOD type - LODタイプ
    /// DESC: Type and size of LOD target - LOD対象の種類、大きさ
    pub lodType: u8,

    /// NAME: Shadow drawing LOD level offset - 影描画LODレベルオフセット
    /// DESC: LOD level offset value when drawing shadows - 影描画時のLODレベルオフセット値
    pub shadowDrawLodOffset: i8,

    /// NAME: Follow the camera on the XZ plane - カメラをXZ平面上で追従する
    /// DESC: Follow the camera on the XZ plane (GR SEQ09242) - カメラをXZ平面上で追従する(GR SEQ09242)
    pub isTraceCameraXZ: u8,

    /// NAME: Switch to the celestial sphere drawing phase - 天球描画フェイズに切り替え
    /// DESC: Set the drawing phase to the celestial sphere (GR SEQ09242) - 描画フェイズを天球に設定する(GR SEQ09242)
    pub isSkydomeDrawPhase: u8,

    /// NAME: Distance switching distance [m] - 遠景切り替え距離[m]
    /// DESC: Distance switching distance [m] - 遠景切り替え距離[m]
    pub DistantViewModel_BorderDist: f32,

    /// NAME: Distant view switching play distance [m] - 遠景切り替え遊び距離[m]
    /// DESC: Distant view switching play distance [m] - 遠景切り替え遊び距離[m]
    pub DistantViewModel_PlayDist: f32,

    /// NAME: Construction limit distance for open [m] - オープン用構築制限距離[m]
    /// DESC: Construction limit distance for open [m]. If the distance to the camera is less than this distance in the open, it will not be built. This is a function for distant view assets. -1: Function disabled (default) - オープン用構築制限距離[m]。オープンにおいてカメラとの距離がこの距離未満だと構築されないようになります。遠景アセット用の機能です。-1:機能無効(デフォルト)
    pub LimtedActivate_BorderDist_forGrid: f32,

    /// NAME: Construction limit play distance for open [m] - オープン用構築制限遊び距離[m]
    /// DESC: Open construction limit play distance [m] - オープン構築制限遊び距離[m]
    pub LimtedActivate_PlayDist_forGrid: f32,

    /// NAME: Z sort offset - Zソートオフセット
    /// DESC: If the distance from the camera is the same in the same drawing phase, the smaller translucent system is drawn in the foreground, and the opaque system is drawn in the larger value. The base point of the offset is the origin in the celestial sphere drawing phase. Other than that, it is mainly Model AABB. (GR SEQ09242) - 同じ描画フェーズ内でカメラからの距離が同じ場合、半透明系は小さいほうが手前、不透明系は値が大きいほうが手前に描画されます。 オフセットの基点は天球描画フェーズのものは原点。それ以外はModelAABB中心。(GR SEQ09242)
    pub zSortOffsetForNoFarClipDraw: f32,

    /// NAME: Shadow drawing alpha test effective distance [m] - 影描画アルファテスト有効距離[m]
    /// DESC: Distance to perform alpha test when drawing shadow [m] - 影描画時にアルファテストを行う距離[m]
    pub shadowDrawAlphaTestDist: f32,

    /// NAME: Forward Drawing environment map Blend type - Forward描画物の環境マップブレンドタイプ
    /// DESC: Forward Drawing environment map Blend type - Forward描画物の環境マップブレンドタイプ
    pub fowardDrawEnvmapBlendType: u8,

    /// NAME: Drawing distance scale parameter ID - 描画距離スケールパラメータID
    /// DESC: Load balancer drawing distance scale parameter ID - ロードバランサー描画距離スケールパラメータID
    pub LBDrawDistScaleParamID: u8,

    /// NAME: reserve - 予約
    /// DESC: reserve - 予約
    pub resereve: [u8; 34],
}

impl Paramdef for PARTS_DRAW_PARAM_ST {
    const NAME: &'static str = "PARTS_DRAW_PARAM_ST";
    const VERSION: u16 = 5;
}
impl PARTS_DRAW_PARAM_ST {
    /// Is crossfade enabled (0: disabled, 1: enabled)? - クロスフェード有効か(0:無効,1:有効)
    /// Bitfield1
    pub fn get_enableCrossFade(&self) -> bool {
        &self.Bitfield1 & 0x1 != 0
    }

    /// Bitfield1
    pub fn set_enableCrossFade(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x1
        } else {
            self.Bitfield1 &= 0xFFFFFFFE
        }
    }
}
