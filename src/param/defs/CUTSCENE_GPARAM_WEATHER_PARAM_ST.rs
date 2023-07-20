/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 6
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CUTSCENE_GPARAM_WEATHER_PARAM_ST {
    /// NAME: Do you remove it from the NT version output? - NT版出力から外すか
    /// DESC: Parameters marked with ○ are excluded in the NT version package. - ○をつけたパラメータをNT版パッケージでは除外します
    pub Bitfield1: u8,

    /// NAME: Reserve for package output 2 - パッケージ出力用リザーブ2
    /// DESC: Reserve for package output 2 - パッケージ出力用リザーブ2
    pub disableParamReserve2: [u8; 3],

    /// NAME: Sunny - 晴れ
    /// DESC: Sunny - 晴れ
    pub DstWeather_Sunny: i16,

    /// NAME: Sunny - 快晴
    /// DESC: Sunny - 快晴
    pub DstWeather_ClearSky: i16,

    /// NAME: Light cloudy - 薄曇り
    /// DESC: Light cloudy - 薄曇り
    pub DstWeather_WeakCloudy: i16,

    /// NAME: cloudy - 曇り
    /// DESC: cloudy - 曇り
    pub DstWeather_Cloud: i16,

    /// NAME: rain - 雨
    /// DESC: rain - 雨
    pub DstWeather_Rain: i16,

    /// NAME: Heavy rain - 豪雨
    /// DESC: Heavy rain - 豪雨
    pub DstWeather_HeavyRain: i16,

    /// NAME: storm - 嵐
    /// DESC: storm - 嵐
    pub DstWeather_Storm: i16,

    /// NAME: Storm (for combat with the descendants of the guardian) - 嵐（守護者の末裔との戦闘用）
    /// DESC: Storm (for combat with the descendants of the guardian) - 嵐（守護者の末裔との戦闘用）
    pub DstWeather_StormForBattle: i16,

    /// NAME: snow - 雪
    /// DESC: snow - 雪
    pub DstWeather_Snow: i16,

    /// NAME: heavy snow - 大雪
    /// DESC: heavy snow - 大雪
    pub DstWeather_HeavySnow: i16,

    /// NAME: fog - 霧
    /// DESC: fog - 霧
    pub DstWeather_Fog: i16,

    /// NAME: Thick fog - 濃霧
    /// DESC: Thick fog - 濃霧
    pub DstWeather_HeavyFog: i16,

    /// NAME: Sandstorm - 砂嵐
    /// DESC: Sandstorm - 砂嵐
    pub DstWeather_SandStorm: i16,

    /// NAME: Thick fog (rain) - 濃霧（雨）
    /// DESC: Thick fog (rain) - 濃霧（雨）
    pub DstWeather_HeavyFogRain: i16,

    /// NAME: In-game weather specifications at the end of playback (unused, invalid) - 再生終了時のインゲーム天候指定(未使用、無効)
    /// DESC: In-game weather specifications at the end of playback (nothing is done if blank or "invalid") - 再生終了時のインゲーム天候指定(空白または「無効」の場合は何も行われない。)
    pub PostPlayIngameWeather: i16,

    /// NAME: Indoor / outdoor designation - 屋内屋外指定
    /// DESC: When indoors, the SFX specified by "Weather SfxId (Outdoor)" and "Wind SfxId (Outdoor)" in "Weather Parameter.xlsm" will be invalid in the cutscene. - 屋内にすると「天候パラメータ.xlsm」の「天候SfxId(屋外)」と「風SfxId(屋外)」で指定されたSFXがカットシーン内で無効になります。
    pub IndoorOutdoorType: u8,

    /// NAME: In-game weather SFX to take over? _Sunny - インゲームの天候SFX引き継ぐか？_晴れ
    /// DESC: In-game weather SFX to take over? _Sunny - インゲームの天候SFX引き継ぐか？_晴れ
    pub TakeOverDstWeather_Sunny: u8,

    /// NAME: In-game weather SFX to take over? _ Sunny - インゲームの天候SFX引き継ぐか？_快晴
    /// DESC: In-game weather SFX to take over? _ Sunny - インゲームの天候SFX引き継ぐか？_快晴
    pub TakeOverDstWeather_ClearSky: u8,

    /// NAME: Will the in-game weather SFX be taken over? _ Light cloudy - インゲームの天候SFX引き継ぐか？_薄曇り
    /// DESC: Will the in-game weather SFX be taken over? _ Light cloudy - インゲームの天候SFX引き継ぐか？_薄曇り
    pub TakeOverDstWeather_WeakCloudy: u8,

    /// NAME: In-game weather SFX to take over? _cloudy - インゲームの天候SFX引き継ぐか？_曇り
    /// DESC: Will the in-game weather SFX be taken over? _cloudy - インゲームの天候SFX引き継ぐか？_曇り
    pub TakeOverDstWeather_Cloud: u8,

    /// NAME: In-game weather SFX to take over? _rain - インゲームの天候SFX引き継ぐか？_雨
    /// DESC: In-game weather SFX to take over? _rain - インゲームの天候SFX引き継ぐか？_雨
    pub TakeOverDstWeather_Rain: u8,

    /// NAME: Will the in-game weather SFX be taken over? _ Heavy rain - インゲームの天候SFX引き継ぐか？_豪雨
    /// DESC: In-game weather SFX to take over? _ Heavy rain - インゲームの天候SFX引き継ぐか？_豪雨
    pub TakeOverDstWeather_HeavyRain: u8,

    /// NAME: Will the in-game weather SFX be taken over? _storm - インゲームの天候SFX引き継ぐか？_嵐
    /// DESC: In-game weather SFX to take over? _storm - インゲームの天候SFX引き継ぐか？_嵐
    pub TakeOverDstWeather_Storm: u8,

    /// NAME: In-game weather SFX to take over? _ Storm (for battle with the descendants of the guardian) - インゲームの天候SFX引き継ぐか？_嵐（守護者の末裔との戦闘用）
    /// DESC: In-game weather SFX to take over? _ Storm (for battle with the descendants of the guardian) - インゲームの天候SFX引き継ぐか？_嵐（守護者の末裔との戦闘用）
    pub TakeOverDstWeather_StormForBattle: u8,

    /// NAME: In-game weather SFX to take over? _snow - インゲームの天候SFX引き継ぐか？_雪
    /// DESC: In-game weather SFX to take over? _snow - インゲームの天候SFX引き継ぐか？_雪
    pub TakeOverDstWeather_Snow: u8,

    /// NAME: In-game weather SFX to take over? _heavy snow - インゲームの天候SFX引き継ぐか？_大雪
    /// DESC: In-game weather SFX to take over? _heavy snow - インゲームの天候SFX引き継ぐか？_大雪
    pub TakeOverDstWeather_HeavySnow: u8,

    /// NAME: In-game weather SFX to take over? _fog - インゲームの天候SFX引き継ぐか？_霧
    /// DESC: Will the in-game weather SFX be taken over? _fog - インゲームの天候SFX引き継ぐか？_霧
    pub TakeOverDstWeather_Fog: u8,

    /// NAME: In-game weather SFX to take over? _ Thick fog - インゲームの天候SFX引き継ぐか？_濃霧
    /// DESC: Will the in-game weather SFX be taken over? _ Thick fog - インゲームの天候SFX引き継ぐか？_濃霧
    pub TakeOverDstWeather_HeavyFog: u8,

    /// NAME: Will the in-game weather SFX be taken over? _ Sandstorm - インゲームの天候SFX引き継ぐか？_砂嵐
    /// DESC: In-game weather SFX to take over? _ Sandstorm - インゲームの天候SFX引き継ぐか？_砂嵐
    pub TakeOverDstWeather_SandStorm: u8,

    /// NAME: In-game weather SFX to take over? _ Thick fog (rain) - インゲームの天候SFX引き継ぐか？_濃霧（雨）
    /// DESC: In-game weather SFX to take over? _ Thick fog (rain) - インゲームの天候SFX引き継ぐか？_濃霧（雨）
    pub TakeOverDstWeather_HeavyFogRain: u8,

    /// NAME: reserved - reserved
    /// DESC: reserved - reserved
    pub reserved: [u8; 7],

    /// NAME: Snowstorm - 吹雪
    /// DESC: Snowstorm - 吹雪
    pub DstWeather_Snowstorm: i16,

    /// NAME: Storm (thunder) - 嵐（雷）
    /// DESC: Preliminary weather 2 - 予備天候2
    pub DstWeather_LightningStorm: i16,

    /// NAME: Snow special (spare 3) - 雪特殊(予備3)
    /// DESC: Preliminary weather 3 - 予備天候3
    pub DstWeather_Reserved3: i16,

    /// NAME: Preliminary weather 4 - 予備天候4
    /// DESC: Preliminary weather 4 - 予備天候4
    pub DstWeather_Reserved4: i16,

    /// NAME: Preliminary weather 5 - 予備天候5
    /// DESC: Preliminary weather 5 - 予備天候5
    pub DstWeather_Reserved5: i16,

    /// NAME: Preliminary weather 6 - 予備天候6
    /// DESC: Preliminary weather 6 - 予備天候6
    pub DstWeather_Reserved6: i16,

    /// NAME: Preliminary weather 7 - 予備天候7
    /// DESC: Preliminary weather 7 - 予備天候7
    pub DstWeather_Reserved7: i16,

    /// NAME: Preliminary weather 8 - 予備天候8
    /// DESC: Preliminary weather 8 - 予備天候8
    pub DstWeather_Reserved8: i16,

    /// NAME: In-game weather SFX to take over? _ Snowstorm - インゲームの天候SFX引き継ぐか？_吹雪
    /// DESC: In-game weather SFX to take over? _ Snowstorm - インゲームの天候SFX引き継ぐか？_吹雪
    pub TakeOverDstWeather_Snowstorm: u8,

    /// NAME: Will the in-game weather SFX be taken over? _ Storm (thunder) - インゲームの天候SFX引き継ぐか？_嵐（雷）
    /// DESC: In-game weather SFX to take over? _ Storm (thunder) - インゲームの天候SFX引き継ぐか？_嵐（雷）
    pub TakeOverDstWeather_LightningStorm: u8,

    /// NAME: In-game weather SFX to take over? _ Snow Special (Spare 3) - インゲームの天候SFX引き継ぐか？_雪特殊(予備3)
    /// DESC: Will the in-game weather SFX be taken over? _ Preliminary weather 3 - インゲームの天候SFX引き継ぐか？_予備天候3
    pub TakeOverDstWeather_Reserved3: u8,

    /// NAME: Will the in-game weather SFX be taken over? _ Preliminary weather 4 - インゲームの天候SFX引き継ぐか？_予備天候4
    /// DESC: In-game weather SFX to take over? _ Preliminary weather 4 - インゲームの天候SFX引き継ぐか？_予備天候4
    pub TakeOverDstWeather_Reserved4: u8,

    /// NAME: In-game weather SFX to take over? _ Preliminary weather 5 - インゲームの天候SFX引き継ぐか？_予備天候5
    /// DESC: Will the in-game weather SFX be taken over? _ Preliminary weather 5 - インゲームの天候SFX引き継ぐか？_予備天候5
    pub TakeOverDstWeather_Reserved5: u8,

    /// NAME: In-game weather SFX to take over? _ Preliminary weather 6 - インゲームの天候SFX引き継ぐか？_予備天候6
    /// DESC: Will the in-game weather SFX be taken over? _ Preliminary weather 6 - インゲームの天候SFX引き継ぐか？_予備天候6
    pub TakeOverDstWeather_Reserved6: u8,

    /// NAME: In-game weather SFX to take over? _ Preliminary weather 7 - インゲームの天候SFX引き継ぐか？_予備天候7
    /// DESC: Will the in-game weather SFX be taken over? _ Preliminary weather 7 - インゲームの天候SFX引き継ぐか？_予備天候7
    pub TakeOverDstWeather_Reserved7: u8,

    /// NAME: Will the in-game weather SFX be taken over? _ Preliminary weather 8 - インゲームの天候SFX引き継ぐか？_予備天候8
    /// DESC: In-game weather SFX to take over? _ Preliminary weather 8 - インゲームの天候SFX引き継ぐか？_予備天候8
    pub TakeOverDstWeather_Reserved8: u8,

    /// NAME: Do you want to apply the MapGD local ID to the weather Gparam? - 天候GparamにMapGD地方IDを適用するか？
    /// DESC: Do you want to apply the mapGD local ID changes to the cutscene weather Gparam as in the in-game? ([GR] SEQ30194) - カットシーン天候Gparamにインゲーム同様MapGD地方IDによる変化を適用するか？(【GR】SEQ30194)
    pub IsEnableApplyMapGdRegionIdForGparam: u8,

    /// NAME: reserved1 - reserved1
    /// DESC: reserved1 ver4-> 5 64-> 96 - reserved1 ver4->5 64->96へ増量
    pub reserved2: [u8; 1],

    /// NAME: Local ID overwrite for weather Gparam MapGD - 天候GparamMapGD用地方ID上書き
    /// DESC: Overwrite the ID used for cutscene weather Gparam (-1: No overwrite. MapGD local ID during cutscene playback is used). If "Do you want to apply MapGD local ID to weather Gparam?" Is x, it is not referenced. - カットシーン天候Gparamに使用されるIDを上書きする(-1：上書きなし。カットシーン再生時のMapGD地方IDが使用される)。「天候GparamにMapGD地方IDを適用するか？」が×の場合は参照されない
    pub OverrideMapGdRegionId: i16,

    /// NAME: reserved1 - reserved1
    /// DESC: reserved1 ver4-> 5 64-> 96 - reserved1 ver4->5 64->96へ増量
    pub reserved1: [u8; 12],
}

impl Paramdef for CUTSCENE_GPARAM_WEATHER_PARAM_ST {
    const NAME: &'static str = "CUTSCENE_GPARAM_WEATHER_PARAM_ST";
    const VERSION: u16 = 6;
}
impl CUTSCENE_GPARAM_WEATHER_PARAM_ST {
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
    /// Parameters marked with a circle are excluded from all packages (because they are for debugging). - ○をつけたパラメータは全パッケージから除外します（デバッグ用なので）
    /// Bitfield1
    pub fn get_disableParam_Debug(&self) -> bool {
        &self.Bitfield1 & 0x2 != 0
    }

    /// Bitfield1
    pub fn set_disableParam_Debug(&mut self, state: bool) {
        if state {
            self.Bitfield1 |= 0x2
        } else {
            self.Bitfield1 &= 0xFD
        }
    }
    /// Reserve for package output 1 - パッケージ出力用リザーブ1
    /// Bitfield1
    pub fn get_disableParamReserve1(&self) -> u8 {
        &self.Bitfield1 & 0xFC
    }

    /// Bitfield1 MAX: 63
    pub fn set_disableParamReserve1(&mut self, state: u8) {
        if state != 0 {
            let val = (state << 2) & 0xFC;
            let newVal = &self.Bitfield1 & 0x3 | val;
            self.Bitfield1 = newVal
        } else {
            self.Bitfield1 &= 0x3
        }
    }
}
