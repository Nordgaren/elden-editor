/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 1
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct CACL_CORRECT_GRAPH_ST {

	/// NAME: Threshold point 0 - 閾値ポイント0
	/// DESC: Those with "nth threshold [point]" written in the specifications - 仕様書に「n次閾値[point]」と書いてあるもの
	pub stageMaxVal0:f32,

	/// NAME: Threshold point 1 - 閾値ポイント1
	/// DESC: Those with "nth threshold [point]" written in the specifications - 仕様書に「n次閾値[point]」と書いてあるもの
	pub stageMaxVal1:f32,

	/// NAME: Threshold point 2 - 閾値ポイント2
	/// DESC: Those with "nth threshold [point]" written in the specifications - 仕様書に「n次閾値[point]」と書いてあるもの
	pub stageMaxVal2:f32,

	/// NAME: Threshold point 3 - 閾値ポイント3
	/// DESC: Those with "nth threshold [point]" written in the specifications - 仕様書に「n次閾値[point]」と書いてあるもの
	pub stageMaxVal3:f32,

	/// NAME: Threshold point 4 - 閾値ポイント4
	/// DESC: Those with "nth threshold [point]" written in the specifications - 仕様書に「n次閾値[point]」と書いてあるもの
	pub stageMaxVal4:f32,

	/// NAME: Threshold coefficient 0 - 閾値係数0
	/// DESC: Those with "nth threshold [coefficient]" written in the specifications - 仕様書に「n次閾値[係数]」と書いてあるもの
	pub stageMaxGrowVal0:f32,

	/// NAME: Threshold coefficient 1 - 閾値係数1
	/// DESC: Those with "nth threshold [coefficient]" written in the specifications - 仕様書に「n次閾値[係数]」と書いてあるもの
	pub stageMaxGrowVal1:f32,

	/// NAME: Threshold coefficient 2 - 閾値係数2
	/// DESC: Those with "nth threshold [coefficient]" written in the specifications - 仕様書に「n次閾値[係数]」と書いてあるもの
	pub stageMaxGrowVal2:f32,

	/// NAME: Threshold coefficient 3 - 閾値係数3
	/// DESC: Those with "nth threshold [coefficient]" written in the specifications - 仕様書に「n次閾値[係数]」と書いてあるもの
	pub stageMaxGrowVal3:f32,

	/// NAME: Threshold coefficient 4 - 閾値係数4
	/// DESC: Those with "nth threshold [coefficient]" written in the specifications - 仕様書に「n次閾値[係数]」と書いてあるもの
	pub stageMaxGrowVal4:f32,

	/// NAME: Adjustment factor 0 - 調整係数0
	/// DESC: Adjustment factor - 調整係数
	pub adjPt_maxGrowVal0:f32,

	/// NAME: Adjustment factor 1 - 調整係数1
	/// DESC: Adjustment factor - 調整係数
	pub adjPt_maxGrowVal1:f32,

	/// NAME: Adjustment factor 2 - 調整係数2
	/// DESC: Adjustment factor - 調整係数
	pub adjPt_maxGrowVal2:f32,

	/// NAME: Adjustment factor 3 - 調整係数3
	/// DESC: Adjustment factor - 調整係数
	pub adjPt_maxGrowVal3:f32,

	/// NAME: Adjustment factor 4 - 調整係数4
	/// DESC: Adjustment factor - 調整係数
	pub adjPt_maxGrowVal4:f32,

	/// NAME: Growth Soul Slope of the early graph α1 - 成長ソウル 初期のグラフの傾きα1
	/// DESC: Growth Soul Slope of the early graph α1 - 成長ソウル 初期のグラフの傾きα1
	pub init_inclination_soul:f32,

	/// NAME: Growth soul Early soul adjustment α2 - 成長ソウル 初期のsoul調整α2
	/// DESC: Growth soul Early soul adjustment α2 - 成長ソウル 初期のsoul調整α2
	pub adjustment_value:f32,

	/// NAME: Affects the slope of the graph after the growth soul threshold α3 - 成長ソウル 閾値後のグラフの傾きに影響α3
	/// DESC: Affects the slope of the graph after the growth soul threshold α3 - 成長ソウル 閾値後のグラフの傾きに影響α3
	pub boundry_inclination_soul:f32,

	/// NAME: Growth soul threshold t - 成長ソウル 閾値 t
	/// DESC: Growth soul threshold t - 成長ソウル 閾値 t
	pub boundry_value:f32,

	/// NAME: Padding - パディング
	pub pad:[u8;4],
}

impl Paramdef for CACL_CORRECT_GRAPH_ST {
const NAME: &'static str = "CACL_CORRECT_GRAPH_ST";
const VERSION: u16 = 1;
}
