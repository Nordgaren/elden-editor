/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct FACE_RANGE_PARAM_ST {

	/// NAME: Face part ID - 顔パーツID
	/// DESC: Face part ID - 顔パーツID
	pub face_partsId:f32,

	/// NAME: Skin color (R) - 肌の色(Ｒ)
	/// DESC: Skin color (R) - 肌の色(Ｒ)
	pub skin_color_R:f32,

	/// NAME: Skin color (G) - 肌の色(Ｇ)
	/// DESC: Skin color (G) - 肌の色(Ｇ)
	pub skin_color_G:f32,

	/// NAME: Skin color (B) - 肌の色(Ｂ)
	/// DESC: Skin color (B) - 肌の色(Ｂ)
	pub skin_color_B:f32,

	/// NAME: Shiny skin - 肌のつや
	/// DESC: Shiny skin - 肌のつや
	pub skin_gloss:f32,

	/// NAME: pores - 毛穴
	/// DESC: pores - 毛穴
	pub skin_pores:f32,

	/// NAME: Blue beard - 青ひげ
	/// DESC: Blue beard - 青ひげ
	pub face_beard:f32,

	/// NAME: Bear - くま
	/// DESC: Bear - くま
	pub face_aroundEye:f32,

	/// NAME: Bear color (R) - くまの色(R)
	/// DESC: Bear color (R) - くまの色(R)
	pub face_aroundEyeColor_R:f32,

	/// NAME: Bear color (G) - くまの色(G)
	/// DESC: Bear color (G) - くまの色(G)
	pub face_aroundEyeColor_G:f32,

	/// NAME: Bear color (B) - くまの色(B)
	/// DESC: Bear color (B) - くまの色(B)
	pub face_aroundEyeColor_B:f32,

	/// NAME: cheek - チーク
	/// DESC: cheek - チーク
	pub face_cheek:f32,

	/// NAME: Teak color (R) - チークの色(R)
	/// DESC: Teak color (R) - チークの色(R)
	pub face_cheekColor_R:f32,

	/// NAME: Teak color (G) - チークの色(G)
	/// DESC: Teak color (G) - チークの色(G)
	pub face_cheekColor_G:f32,

	/// NAME: Teak color (B) - チークの色(B)
	/// DESC: Teak color (B) - チークの色(B)
	pub face_cheekColor_B:f32,

	/// NAME: Eyeline - アイライン
	/// DESC: Eyeline - アイライン
	pub face_eyeLine:f32,

	/// NAME: Eyeliner color (R) - アイラインの色(R)
	/// DESC: Eyeliner color (R) - アイラインの色(R)
	pub face_eyeLineColor_R:f32,

	/// NAME: Eyeliner color (G) - アイラインの色(G)
	/// DESC: Eyeliner color (G) - アイラインの色(G)
	pub face_eyeLineColor_G:f32,

	/// NAME: Eyeliner color (B) - アイラインの色(B)
	/// DESC: Eyeliner color (B) - アイラインの色(B)
	pub face_eyeLineColor_B:f32,

	/// NAME: Eye shadow (bottom) - アイシャドウ(下)
	/// DESC: Eye shadow (bottom) - アイシャドウ(下)
	pub face_eyeShadowDown:f32,

	/// NAME: Eyeshadow (bottom) color (R) - アイシャドウ(下)の色(R)
	/// DESC: Eyeshadow (bottom) color (R) - アイシャドウ(下)の色(R)
	pub face_eyeShadowDownColor_R:f32,

	/// NAME: Eyeshadow (bottom) color (G) - アイシャドウ(下)の色(G)
	/// DESC: Eyeshadow (bottom) color (G) - アイシャドウ(下)の色(G)
	pub face_eyeShadowDownColor_G:f32,

	/// NAME: Eyeshadow (bottom) color (B) - アイシャドウ(下)の色(B)
	/// DESC: Eyeshadow (bottom) color (B) - アイシャドウ(下)の色(B)
	pub face_eyeShadowDownColor_B:f32,

	/// NAME: Eye shadow (top) - アイシャドウ(上)
	/// DESC: Eye shadow (top) - アイシャドウ(上)
	pub face_eyeShadowUp:f32,

	/// NAME: Eyeshadow (top) color (R) - アイシャドウ(上)の色(R)
	/// DESC: Eyeshadow (top) color (R) - アイシャドウ(上)の色(R)
	pub face_eyeShadowUpColor_R:f32,

	/// NAME: Eyeshadow (top) color (G) - アイシャドウ(上)の色(G)
	/// DESC: Eyeshadow (top) color (G) - アイシャドウ(上)の色(G)
	pub face_eyeShadowUpColor_G:f32,

	/// NAME: Eyeshadow (top) color (B) - アイシャドウ(上)の色(B)
	/// DESC: Eyeshadow (top) color (B) - アイシャドウ(上)の色(B)
	pub face_eyeShadowUpColor_B:f32,

	/// NAME: lipstick - 口紅
	/// DESC: lipstick - 口紅
	pub face_lip:f32,

	/// NAME: Lipstick color (R) - 口紅の色(R)
	/// DESC: Lipstick color (R) - 口紅の色(R)
	pub face_lipColor_R:f32,

	/// NAME: Lipstick color (G) - 口紅の色(G)
	/// DESC: Lipstick color (G) - 口紅の色(G)
	pub face_lipColor_G:f32,

	/// NAME: Lipstick color (B) - 口紅の色(B)
	/// DESC: Lipstick color (B) - 口紅の色(B)
	pub face_lipColor_B:f32,

	/// NAME: Hair thickness - 体毛の濃さ
	/// DESC: Hair thickness - 体毛の濃さ
	pub body_hair:f32,

	/// NAME: Hair color (R) - 体毛の色(R)
	/// DESC: Hair color (R) - 体毛の色(R)
	pub body_hairColor_R:f32,

	/// NAME: Hair color (G) - 体毛の色(G)
	/// DESC: Hair color (G) - 体毛の色(G)
	pub body_hairColor_G:f32,

	/// NAME: Hair color (B) - 体毛の色(B)
	/// DESC: Hair color (B) - 体毛の色(B)
	pub body_hairColor_B:f32,

	/// NAME: Eyeball part ID - 眼球パーツID
	/// DESC: Eyeball part ID - 眼球パーツID
	pub eye_partsId:f32,

	/// NAME: Iris color (R) - 虹彩の色(Ｒ)
	/// DESC: Right eye iris color (R) - 右目の虹彩の色(Ｒ)
	pub eyeR_irisColor_R:f32,

	/// NAME: Iris color (G) - 虹彩の色(Ｇ)
	/// DESC: Right eye iris color (G) - 右目の虹彩の色(Ｇ)
	pub eyeR_irisColor_G:f32,

	/// NAME: Iris color (B) - 虹彩の色(Ｂ)
	/// DESC: Right eye iris color (B) - 右目の虹彩の色(Ｂ)
	pub eyeR_irisColor_B:f32,

	/// NAME: The size of the iris - 虹彩の大きさ
	/// DESC: The size of the iris of the right eye - 右目の虹彩の大きさ
	pub eyeR_irisScale:f32,

	/// NAME: Cloudiness of the crystalline lens - 水晶体の濁り
	/// DESC: Cloudiness of the crystalline lens of the right eye - 右目の水晶体の濁り
	pub eyeR_cataract:f32,

	/// NAME: The turbid color of the crystalline lens (R) - 水晶体の濁りの色(Ｒ)
	/// DESC: The turbid color of the crystalline lens of the right eye (R) - 右目の水晶体の濁りの色(Ｒ)
	pub eyeR_cataractColor_R:f32,

	/// NAME: The turbid color of the crystalline lens (G) - 水晶体の濁りの色(Ｇ)
	/// DESC: The turbid color of the crystalline lens of the right eye (G) - 右目の水晶体の濁りの色(Ｇ)
	pub eyeR_cataractColor_G:f32,

	/// NAME: The turbid color of the crystalline lens (B) - 水晶体の濁りの色(Ｂ)
	/// DESC: The turbid color of the crystalline lens of the right eye (B) - 右目の水晶体の濁りの色(Ｂ)
	pub eyeR_cataractColor_B:f32,

	/// NAME: White eye color (R) - 白目の色(Ｒ)
	/// DESC: White eye color of the right eye (R) - 右目の白目の色(Ｒ)
	pub eyeR_scleraColor_R:f32,

	/// NAME: White eye color (G) - 白目の色(G)
	/// DESC: White eye color of the right eye (G) - 右目の白目の色(G)
	pub eyeR_scleraColor_G:f32,

	/// NAME: White eye color (B) - 白目の色(B)
	/// DESC: White eye color of the right eye (B) - 右目の白目の色(B)
	pub eyeR_scleraColor_B:f32,

	/// NAME: Position of the iris - 虹彩の位置
	/// DESC: Position of the iris of the right eye - 右目の虹彩の位置
	pub eyeR_irisDistance:f32,

	/// NAME: Iris color (R) - 虹彩の色(Ｒ)
	/// DESC: Left eye iris color (R) - 左目の虹彩の色(Ｒ)
	pub eyeL_irisColor_R:f32,

	/// NAME: Iris color (G) - 虹彩の色(Ｇ)
	/// DESC: Left eye iris color (G) - 左目の虹彩の色(Ｇ)
	pub eyeL_irisColor_G:f32,

	/// NAME: Iris color (B) - 虹彩の色(Ｂ)
	/// DESC: Left eye iris color (B) - 左目の虹彩の色(Ｂ)
	pub eyeL_irisColor_B:f32,

	/// NAME: The size of the iris - 虹彩の大きさ
	/// DESC: The size of the iris of the left eye - 左目の虹彩の大きさ
	pub eyeL_irisScale:f32,

	/// NAME: Cloudiness of the crystalline lens - 水晶体の濁り
	/// DESC: Cloudiness of the crystalline lens of the left eye - 左目の水晶体の濁り
	pub eyeL_cataract:f32,

	/// NAME: The turbid color of the crystalline lens (R) - 水晶体の濁りの色(Ｒ)
	/// DESC: The turbid color of the crystalline lens of the left eye (R) - 左目の水晶体の濁りの色(Ｒ)
	pub eyeL_cataractColor_R:f32,

	/// NAME: The turbid color of the crystalline lens (G) - 水晶体の濁りの色(Ｇ)
	/// DESC: The turbid color of the crystalline lens of the left eye (G) - 左目の水晶体の濁りの色(Ｇ)
	pub eyeL_cataractColor_G:f32,

	/// NAME: The turbid color of the crystalline lens (B) - 水晶体の濁りの色(Ｂ)
	/// DESC: The turbid color of the crystalline lens of the left eye (B) - 左目の水晶体の濁りの色(Ｂ)
	pub eyeL_cataractColor_B:f32,

	/// NAME: White eye color (R) - 白目の色(Ｒ)
	/// DESC: White eye color of the left eye (R) - 左目の白目の色(Ｒ)
	pub eyeL_scleraColor_R:f32,

	/// NAME: White eye color (G) - 白目の色(G)
	/// DESC: White eye color of the left eye (G) - 左目の白目の色(G)
	pub eyeL_scleraColor_G:f32,

	/// NAME: White eye color (B) - 白目の色(B)
	/// DESC: White eye color of the left eye (B) - 左目の白目の色(B)
	pub eyeL_scleraColor_B:f32,

	/// NAME: Position of the iris - 虹彩の位置
	/// DESC: Position of the iris of the left eye - 左目の虹彩の位置
	pub eyeL_irisDistance:f32,

	/// NAME: Hair part ID - 髪パーツID
	/// DESC: Hair part ID - 髪パーツID
	pub hair_partsId:f32,

	/// NAME: Hair color (R) - 髪の色(Ｒ)
	/// DESC: Hair color (R) - 髪の色(Ｒ)
	pub hair_color_R:f32,

	/// NAME: Hair color (G) - 髪の色(Ｇ)
	/// DESC: Hair color (G) - 髪の色(Ｇ)
	pub hair_color_G:f32,

	/// NAME: Hair color (B) - 髪の色(Ｂ)
	/// DESC: Hair color (B) - 髪の色(Ｂ)
	pub hair_color_B:f32,

	/// NAME: Gloss - 光沢
	/// DESC: Hair gloss - 髪の光沢
	pub hair_shininess:f32,

	/// NAME: Blackness at the base - 根元の黒さ
	/// DESC: Blackness at the base of hair - 髪の根元の黒さ
	pub hair_rootBlack:f32,

	/// NAME: Amount of gray hair - 白髪の量
	/// DESC: Amount of white hair - 髪の白髪の量
	pub hair_whiteDensity:f32,

	/// NAME: Beard part ID - 髭パーツID
	/// DESC: Beard part ID - 髭パーツID
	pub beard_partsId:f32,

	/// NAME: Beard color (R) - 髭の色(Ｒ)
	/// DESC: Beard color (R) - 髭の色(Ｒ)
	pub beard_color_R:f32,

	/// NAME: Beard color (G) - 髭の色(Ｇ)
	/// DESC: Beard color (G) - 髭の色(Ｇ)
	pub beard_color_G:f32,

	/// NAME: Beard color (B) - 髭の色(Ｂ)
	/// DESC: Beard color (B) - 髭の色(Ｂ)
	pub beard_color_B:f32,

	/// NAME: Gloss - 光沢
	/// DESC: Beard luster - 髭の光沢
	pub beard_shininess:f32,

	/// NAME: Blackness at the base - 根元の黒さ
	/// DESC: Blackness at the base of the beard - 髭の根元の黒さ
	pub beard_rootBlack:f32,

	/// NAME: Amount of gray hair - 白髪の量
	/// DESC: Amount of white hair with a beard - 髭の白髪の量
	pub beard_whiteDensity:f32,

	/// NAME: Eyebrow part ID - 眉パーツID
	/// DESC: Eyebrow part ID - 眉パーツID
	pub eyebrow_partsId:f32,

	/// NAME: Eyebrow color (R) - 眉の色(Ｒ)
	/// DESC: Eyebrow color (R) - 眉の色(Ｒ)
	pub eyebrow_color_R:f32,

	/// NAME: Eyebrow color (G) - 眉の色(Ｇ)
	/// DESC: Eyebrow color (G) - 眉の色(Ｇ)
	pub eyebrow_color_G:f32,

	/// NAME: Eyebrow color (B) - 眉の色(Ｂ)
	/// DESC: Eyebrow color (B) - 眉の色(Ｂ)
	pub eyebrow_color_B:f32,

	/// NAME: Gloss - 光沢
	/// DESC: Glossy eyebrows - 眉の光沢
	pub eyebrow_shininess:f32,

	/// NAME: Blackness at the base - 根元の黒さ
	/// DESC: Blackness at the base of the eyebrows - 眉の根元の黒さ
	pub eyebrow_rootBlack:f32,

	/// NAME: Amount of gray hair - 白髪の量
	/// DESC: Amount of white hair on the eyebrows - 眉の白髪の量
	pub eyebrow_whiteDensity:f32,

	/// NAME: Eyelash parts ID - まつげパーツID
	/// DESC: Eyelash parts ID - まつげパーツID
	pub eyelash_partsId:f32,

	/// NAME: Eyelash color (R) - まつげの色(Ｒ)
	/// DESC: Eyelash color (R) - まつげの色(Ｒ)
	pub eyelash_color_R:f32,

	/// NAME: Eyelash color (G) - まつげの色(Ｇ)
	/// DESC: Eyelash color (G) - まつげの色(Ｇ)
	pub eyelash_color_G:f32,

	/// NAME: Eyelash color (B) - まつげの色(Ｂ)
	/// DESC: Eyelash color (B) - まつげの色(Ｂ)
	pub eyelash_color_B:f32,

	/// NAME: Decorative part ID - 装飾パーツID
	/// DESC: Decorative part ID - 装飾パーツID
	pub accessories_partsId:f32,

	/// NAME: Decoration color (R) - 装飾の色(Ｒ)
	/// DESC: Decoration color (R) - 装飾の色(Ｒ)
	pub accessories_color_R:f32,

	/// NAME: Decoration color (G) - 装飾の色(Ｇ)
	/// DESC: Decoration color (G) - 装飾の色(Ｇ)
	pub accessories_color_G:f32,

	/// NAME: Decorative color (B) - 装飾の色(Ｂ)
	/// DESC: Decorative color (B) - 装飾の色(Ｂ)
	pub accessories_color_B:f32,

	/// NAME: Decal part ID - デカールパーツID
	/// DESC: Decal part ID - デカールパーツID
	pub decal_partsId:f32,

	/// NAME: Decal position (x) - デカール位置(x)
	/// DESC: Decal position (x) - デカール位置(x)
	pub decal_posX:f32,

	/// NAME: Decal position (y) - デカール位置(y)
	/// DESC: Decal position (y) - デカール位置(y)
	pub decal_posY:f32,

	/// NAME: Decal angle - デカール角度
	/// DESC: Decal angle - デカール角度
	pub decal_angle:f32,

	/// NAME: Decal scale - デカールスケール
	/// DESC: Decal scale - デカールスケール
	pub decal_scale:f32,

	/// NAME: Decal color (R) - デカールの色(Ｒ)
	/// DESC: Decal color (R) - デカールの色(Ｒ)
	pub decal_color_R:f32,

	/// NAME: Decal color (G) - デカールの色(Ｇ)
	/// DESC: Decal color (G) - デカールの色(Ｇ)
	pub decal_color_G:f32,

	/// NAME: Decal color (B) - デカールの色(Ｂ)
	/// DESC: Decal color (B) - デカールの色(Ｂ)
	pub decal_color_B:f32,

	/// NAME: Decal gloss - デカールのつや
	/// DESC: Decal gloss - デカールのつや
	pub decal_gloss:f32,

	/// NAME: Decal reversal - デカールの反転
	/// DESC: Decal reversal - デカールの反転
	pub decal_mirror:f32,

	/// NAME: Character body head scale - キャラ体型頭部スケール
	/// DESC: Character body head scale - キャラ体型頭部スケール
	pub chrBodyScaleHead:f32,

	/// NAME: Character body chest scale - キャラ体型胸部スケール
	/// DESC: Character body chest scale - キャラ体型胸部スケール
	pub chrBodyScaleBreast:f32,

	/// NAME: Character body type abdominal scale - キャラ体型腹部スケール
	/// DESC: Character body type abdominal scale - キャラ体型腹部スケール
	pub chrBodyScaleAbdomen:f32,

	/// NAME: Character body type arm scale - キャラ体型腕部スケール
	/// DESC: Character body type arm scale - キャラ体型腕部スケール
	pub chrBodyScaleArm:f32,

	/// NAME: Character body type leg scale - キャラ体型脚部スケール
	/// DESC: Character body type leg scale - キャラ体型脚部スケール
	pub chrBodyScaleLeg:f32,

	/// NAME: age - 年齢
	/// DESC: age - 年齢
	pub age:f32,

	/// NAME: sex - 性別
	/// DESC: sex - 性別
	pub gender:f32,

	/// NAME: Exaggeration (model) - 誇張（モデル）
	/// DESC: Exaggeration (model) - 誇張（モデル）
	pub caricatureGeometry:f32,

	/// NAME: Exaggeration (texture) - 誇張（テクスチャ）
	/// DESC: Exaggeration (texture) - 誇張（テクスチャ）
	pub caricatureTexture:f32,

	/// NAME: Face creation geometry data 00 - 顔作成ジオメトリデータ00
	/// DESC: Face creation geometry data 00 - 顔作成ジオメトリデータ00
	pub faceGeoData00:f32,

	/// NAME: Face creation geometry data 01 - 顔作成ジオメトリデータ01
	/// DESC: Face creation geometry data 01 - 顔作成ジオメトリデータ01
	pub faceGeoData01:f32,

	/// NAME: Face creation geometry data 02 - 顔作成ジオメトリデータ02
	/// DESC: Face creation geometry data 02 - 顔作成ジオメトリデータ02
	pub faceGeoData02:f32,

	/// NAME: Face creation geometry data 03 - 顔作成ジオメトリデータ03
	/// DESC: Face creation geometry data 03 - 顔作成ジオメトリデータ03
	pub faceGeoData03:f32,

	/// NAME: Face creation geometry data 04 - 顔作成ジオメトリデータ04
	/// DESC: Face creation geometry data 04 - 顔作成ジオメトリデータ04
	pub faceGeoData04:f32,

	/// NAME: Face creation geometry data 05 - 顔作成ジオメトリデータ05
	/// DESC: Face creation geometry data 05 - 顔作成ジオメトリデータ05
	pub faceGeoData05:f32,

	/// NAME: Face creation geometry data 06 - 顔作成ジオメトリデータ06
	/// DESC: Face creation geometry data 06 - 顔作成ジオメトリデータ06
	pub faceGeoData06:f32,

	/// NAME: Face creation geometry data 07 - 顔作成ジオメトリデータ07
	/// DESC: Face creation geometry data 07 - 顔作成ジオメトリデータ07
	pub faceGeoData07:f32,

	/// NAME: Face creation geometry data 08 - 顔作成ジオメトリデータ08
	/// DESC: Face creation geometry data 08 - 顔作成ジオメトリデータ08
	pub faceGeoData08:f32,

	/// NAME: Face creation geometry data 09 - 顔作成ジオメトリデータ09
	/// DESC: Face creation geometry data 09 - 顔作成ジオメトリデータ09
	pub faceGeoData09:f32,

	/// NAME: Face creation geometry data 10 - 顔作成ジオメトリデータ10
	/// DESC: Face creation geometry data 10 - 顔作成ジオメトリデータ10
	pub faceGeoData10:f32,

	/// NAME: Face creation geometry data 11 - 顔作成ジオメトリデータ11
	/// DESC: Face creation geometry data 11 - 顔作成ジオメトリデータ11
	pub faceGeoData11:f32,

	/// NAME: Face creation geometry data 12 - 顔作成ジオメトリデータ12
	/// DESC: Face creation geometry data 12 - 顔作成ジオメトリデータ12
	pub faceGeoData12:f32,

	/// NAME: Face creation geometry data 13 - 顔作成ジオメトリデータ13
	/// DESC: Face creation geometry data 13 - 顔作成ジオメトリデータ13
	pub faceGeoData13:f32,

	/// NAME: Face creation geometry data 14 - 顔作成ジオメトリデータ14
	/// DESC: Face creation geometry data 14 - 顔作成ジオメトリデータ14
	pub faceGeoData14:f32,

	/// NAME: Face creation geometry data 15 - 顔作成ジオメトリデータ15
	/// DESC: Face creation geometry data 15 - 顔作成ジオメトリデータ15
	pub faceGeoData15:f32,

	/// NAME: Face creation geometry data 16 - 顔作成ジオメトリデータ16
	/// DESC: Face creation geometry data 16 - 顔作成ジオメトリデータ16
	pub faceGeoData16:f32,

	/// NAME: Face creation geometry data 17 - 顔作成ジオメトリデータ17
	/// DESC: Face creation geometry data 17 - 顔作成ジオメトリデータ17
	pub faceGeoData17:f32,

	/// NAME: Face creation geometry data 18 - 顔作成ジオメトリデータ18
	/// DESC: Face creation geometry data 18 - 顔作成ジオメトリデータ18
	pub faceGeoData18:f32,

	/// NAME: Face creation geometry data 19 - 顔作成ジオメトリデータ19
	/// DESC: Face creation geometry data 19 - 顔作成ジオメトリデータ19
	pub faceGeoData19:f32,

	/// NAME: Face creation geometry data 20 - 顔作成ジオメトリデータ20
	/// DESC: Face creation geometry data 20 - 顔作成ジオメトリデータ20
	pub faceGeoData20:f32,

	/// NAME: Face creation geometry data 21 - 顔作成ジオメトリデータ21
	/// DESC: Face creation geometry data 21 - 顔作成ジオメトリデータ21
	pub faceGeoData21:f32,

	/// NAME: Face creation geometry data 22 - 顔作成ジオメトリデータ22
	/// DESC: Face creation geometry data 22 - 顔作成ジオメトリデータ22
	pub faceGeoData22:f32,

	/// NAME: Face creation geometry data 23 - 顔作成ジオメトリデータ23
	/// DESC: Face creation geometry data 23 - 顔作成ジオメトリデータ23
	pub faceGeoData23:f32,

	/// NAME: Face creation geometry data 24 - 顔作成ジオメトリデータ24
	/// DESC: Face creation geometry data 24 - 顔作成ジオメトリデータ24
	pub faceGeoData24:f32,

	/// NAME: Face creation geometry data 25 - 顔作成ジオメトリデータ25
	/// DESC: Face creation geometry data 25 - 顔作成ジオメトリデータ25
	pub faceGeoData25:f32,

	/// NAME: Face creation geometry data 26 - 顔作成ジオメトリデータ26
	/// DESC: Face creation geometry data 26 - 顔作成ジオメトリデータ26
	pub faceGeoData26:f32,

	/// NAME: Face creation geometry data 27 - 顔作成ジオメトリデータ27
	/// DESC: Face creation geometry data 27 - 顔作成ジオメトリデータ27
	pub faceGeoData27:f32,

	/// NAME: Face creation geometry data 28 - 顔作成ジオメトリデータ28
	/// DESC: Face creation geometry data 28 - 顔作成ジオメトリデータ28
	pub faceGeoData28:f32,

	/// NAME: Face creation geometry data 29 - 顔作成ジオメトリデータ29
	/// DESC: Face creation geometry data 29 - 顔作成ジオメトリデータ29
	pub faceGeoData29:f32,

	/// NAME: Face creation geometry data 30 - 顔作成ジオメトリデータ30
	/// DESC: Face creation geometry data 30 - 顔作成ジオメトリデータ30
	pub faceGeoData30:f32,

	/// NAME: Face creation geometry data 31 - 顔作成ジオメトリデータ31
	/// DESC: Face creation geometry data 31 - 顔作成ジオメトリデータ31
	pub faceGeoData31:f32,

	/// NAME: Face creation geometry data 32 - 顔作成ジオメトリデータ32
	/// DESC: Face creation geometry data 32 - 顔作成ジオメトリデータ32
	pub faceGeoData32:f32,

	/// NAME: Face creation geometry data 33 - 顔作成ジオメトリデータ33
	/// DESC: Face creation geometry data 33 - 顔作成ジオメトリデータ33
	pub faceGeoData33:f32,

	/// NAME: Face creation geometry data 34 - 顔作成ジオメトリデータ34
	/// DESC: Face creation geometry data 34 - 顔作成ジオメトリデータ34
	pub faceGeoData34:f32,

	/// NAME: Face creation geometry data 35 - 顔作成ジオメトリデータ35
	/// DESC: Face creation geometry data 35 - 顔作成ジオメトリデータ35
	pub faceGeoData35:f32,

	/// NAME: Face creation geometry data 36 - 顔作成ジオメトリデータ36
	/// DESC: Face creation geometry data 36 - 顔作成ジオメトリデータ36
	pub faceGeoData36:f32,

	/// NAME: Face creation geometry data 37 - 顔作成ジオメトリデータ37
	/// DESC: Face creation geometry data 37 - 顔作成ジオメトリデータ37
	pub faceGeoData37:f32,

	/// NAME: Face creation geometry data 38 - 顔作成ジオメトリデータ38
	/// DESC: Face creation geometry data 38 - 顔作成ジオメトリデータ38
	pub faceGeoData38:f32,

	/// NAME: Face creation geometry data 39 - 顔作成ジオメトリデータ39
	/// DESC: Face creation geometry data 39 - 顔作成ジオメトリデータ39
	pub faceGeoData39:f32,

	/// NAME: Face creation geometry data 40 - 顔作成ジオメトリデータ40
	/// DESC: Face creation geometry data 40 - 顔作成ジオメトリデータ40
	pub faceGeoData40:f32,

	/// NAME: Face creation geometry data 41 - 顔作成ジオメトリデータ41
	/// DESC: Face creation geometry data 41 - 顔作成ジオメトリデータ41
	pub faceGeoData41:f32,

	/// NAME: Face creation geometry data 42 - 顔作成ジオメトリデータ42
	/// DESC: Face creation geometry data 42 - 顔作成ジオメトリデータ42
	pub faceGeoData42:f32,

	/// NAME: Face creation geometry data 43 - 顔作成ジオメトリデータ43
	/// DESC: Face creation geometry data 43 - 顔作成ジオメトリデータ43
	pub faceGeoData43:f32,

	/// NAME: Face creation geometry data 44 - 顔作成ジオメトリデータ44
	/// DESC: Face creation geometry data 44 - 顔作成ジオメトリデータ44
	pub faceGeoData44:f32,

	/// NAME: Face creation geometry data 45 - 顔作成ジオメトリデータ45
	/// DESC: Face creation geometry data 45 - 顔作成ジオメトリデータ45
	pub faceGeoData45:f32,

	/// NAME: Face creation geometry data 46 - 顔作成ジオメトリデータ46
	/// DESC: Face creation geometry data 46 - 顔作成ジオメトリデータ46
	pub faceGeoData46:f32,

	/// NAME: Face creation geometry data 47 - 顔作成ジオメトリデータ47
	/// DESC: Face creation geometry data 47 - 顔作成ジオメトリデータ47
	pub faceGeoData47:f32,

	/// NAME: Face creation geometry data 48 - 顔作成ジオメトリデータ48
	/// DESC: Face creation geometry data 48 - 顔作成ジオメトリデータ48
	pub faceGeoData48:f32,

	/// NAME: Face creation geometry data 49 - 顔作成ジオメトリデータ49
	/// DESC: Face creation geometry data 49 - 顔作成ジオメトリデータ49
	pub faceGeoData49:f32,

	/// NAME: Face creation geometry data 50 - 顔作成ジオメトリデータ50
	/// DESC: Face creation geometry data 50 - 顔作成ジオメトリデータ50
	pub faceGeoData50:f32,

	/// NAME: Face creation geometry data 51 - 顔作成ジオメトリデータ51
	/// DESC: Face creation geometry data 51 - 顔作成ジオメトリデータ51
	pub faceGeoData51:f32,

	/// NAME: Face creation geometry data 52 - 顔作成ジオメトリデータ52
	/// DESC: Face creation geometry data 52 - 顔作成ジオメトリデータ52
	pub faceGeoData52:f32,

	/// NAME: Face creation geometry data 53 - 顔作成ジオメトリデータ53
	/// DESC: Face creation geometry data 53 - 顔作成ジオメトリデータ53
	pub faceGeoData53:f32,

	/// NAME: Face creation geometry data 54 - 顔作成ジオメトリデータ54
	/// DESC: Face creation geometry data 54 - 顔作成ジオメトリデータ54
	pub faceGeoData54:f32,

	/// NAME: Face creation geometry data 55 - 顔作成ジオメトリデータ55
	/// DESC: Face creation geometry data 55 - 顔作成ジオメトリデータ55
	pub faceGeoData55:f32,

	/// NAME: Face creation geometry data 56 - 顔作成ジオメトリデータ56
	/// DESC: Face creation geometry data 56 - 顔作成ジオメトリデータ56
	pub faceGeoData56:f32,

	/// NAME: Face creation geometry data 57 - 顔作成ジオメトリデータ57
	/// DESC: Face creation geometry data 57 - 顔作成ジオメトリデータ57
	pub faceGeoData57:f32,

	/// NAME: Face creation geometry data 58 - 顔作成ジオメトリデータ58
	/// DESC: Face creation geometry data 58 - 顔作成ジオメトリデータ58
	pub faceGeoData58:f32,

	/// NAME: Face creation geometry data 59 - 顔作成ジオメトリデータ59
	/// DESC: Face creation geometry data 59 - 顔作成ジオメトリデータ59
	pub faceGeoData59:f32,

	/// NAME: Face creation geometry data 60 - 顔作成ジオメトリデータ60
	/// DESC: Face creation geometry data 60 - 顔作成ジオメトリデータ60
	pub faceGeoData60:f32,

	/// NAME: Face creation texture data 00 - 顔作成テクスチャデータ00
	/// DESC: Face creation texture data 00 - 顔作成テクスチャデータ00
	pub faceTexData00:f32,

	/// NAME: Face creation texture data 01 - 顔作成テクスチャデータ01
	/// DESC: Face creation texture data 01 - 顔作成テクスチャデータ01
	pub faceTexData01:f32,

	/// NAME: Face creation texture data 02 - 顔作成テクスチャデータ02
	/// DESC: Face creation texture data 02 - 顔作成テクスチャデータ02
	pub faceTexData02:f32,

	/// NAME: Face creation texture data 03 - 顔作成テクスチャデータ03
	/// DESC: Face creation texture data 03 - 顔作成テクスチャデータ03
	pub faceTexData03:f32,

	/// NAME: Face creation texture data 04 - 顔作成テクスチャデータ04
	/// DESC: Face creation texture data 04 - 顔作成テクスチャデータ04
	pub faceTexData04:f32,

	/// NAME: Face creation texture data 05 - 顔作成テクスチャデータ05
	/// DESC: Face creation texture data 05 - 顔作成テクスチャデータ05
	pub faceTexData05:f32,

	/// NAME: Face creation texture data 06 - 顔作成テクスチャデータ06
	/// DESC: Face creation texture data 06 - 顔作成テクスチャデータ06
	pub faceTexData06:f32,

	/// NAME: Face creation texture data 07 - 顔作成テクスチャデータ07
	/// DESC: Face creation texture data 07 - 顔作成テクスチャデータ07
	pub faceTexData07:f32,

	/// NAME: Face creation texture data 08 - 顔作成テクスチャデータ08
	/// DESC: Face creation texture data 08 - 顔作成テクスチャデータ08
	pub faceTexData08:f32,

	/// NAME: Face creation texture data 09 - 顔作成テクスチャデータ09
	/// DESC: Face creation texture data 09 - 顔作成テクスチャデータ09
	pub faceTexData09:f32,

	/// NAME: Face creation texture data 10 - 顔作成テクスチャデータ10
	/// DESC: Face creation texture data 10 - 顔作成テクスチャデータ10
	pub faceTexData10:f32,

	/// NAME: Face creation texture data 11 - 顔作成テクスチャデータ11
	/// DESC: Face creation texture data 11 - 顔作成テクスチャデータ11
	pub faceTexData11:f32,

	/// NAME: Face creation texture data 12 - 顔作成テクスチャデータ12
	/// DESC: Face creation texture data 12 - 顔作成テクスチャデータ12
	pub faceTexData12:f32,

	/// NAME: Face creation texture data 13 - 顔作成テクスチャデータ13
	/// DESC: Face creation texture data 13 - 顔作成テクスチャデータ13
	pub faceTexData13:f32,

	/// NAME: Face creation texture data 14 - 顔作成テクスチャデータ14
	/// DESC: Face creation texture data 14 - 顔作成テクスチャデータ14
	pub faceTexData14:f32,

	/// NAME: Face creation texture data 15 - 顔作成テクスチャデータ15
	/// DESC: Face creation texture data 15 - 顔作成テクスチャデータ15
	pub faceTexData15:f32,

	/// NAME: Face creation texture data 16 - 顔作成テクスチャデータ16
	/// DESC: Face creation texture data 16 - 顔作成テクスチャデータ16
	pub faceTexData16:f32,

	/// NAME: Face creation texture data 17 - 顔作成テクスチャデータ17
	/// DESC: Face creation texture data 17 - 顔作成テクスチャデータ17
	pub faceTexData17:f32,

	/// NAME: Face creation texture data 18 - 顔作成テクスチャデータ18
	/// DESC: Face creation texture data 18 - 顔作成テクスチャデータ18
	pub faceTexData18:f32,

	/// NAME: Face creation texture data 19 - 顔作成テクスチャデータ19
	/// DESC: Face creation texture data 19 - 顔作成テクスチャデータ19
	pub faceTexData19:f32,

	/// NAME: Face creation texture data 20 - 顔作成テクスチャデータ20
	/// DESC: Face creation texture data 20 - 顔作成テクスチャデータ20
	pub faceTexData20:f32,

	/// NAME: Face creation texture data 21 - 顔作成テクスチャデータ21
	/// DESC: Face creation texture data 21 - 顔作成テクスチャデータ21
	pub faceTexData21:f32,

	/// NAME: Face creation texture data 22 - 顔作成テクスチャデータ22
	/// DESC: Face creation texture data 22 - 顔作成テクスチャデータ22
	pub faceTexData22:f32,

	/// NAME: Face creation texture data 23 - 顔作成テクスチャデータ23
	/// DESC: Face creation texture data 23 - 顔作成テクスチャデータ23
	pub faceTexData23:f32,

	/// NAME: Face creation texture data 24 - 顔作成テクスチャデータ24
	/// DESC: Face creation texture data 24 - 顔作成テクスチャデータ24
	pub faceTexData24:f32,

	/// NAME: Face creation texture data 25 - 顔作成テクスチャデータ25
	/// DESC: Face creation texture data 25 - 顔作成テクスチャデータ25
	pub faceTexData25:f32,

	/// NAME: Face creation texture data 26 - 顔作成テクスチャデータ26
	/// DESC: Face creation texture data 26 - 顔作成テクスチャデータ26
	pub faceTexData26:f32,

	/// NAME: Face creation texture data 27 - 顔作成テクスチャデータ27
	/// DESC: Face creation texture data 27 - 顔作成テクスチャデータ27
	pub faceTexData27:f32,

	/// NAME: Face creation texture data 28 - 顔作成テクスチャデータ28
	/// DESC: Face creation texture data 28 - 顔作成テクスチャデータ28
	pub faceTexData28:f32,

	/// NAME: Face creation texture data 29 - 顔作成テクスチャデータ29
	/// DESC: Face creation texture data 29 - 顔作成テクスチャデータ29
	pub faceTexData29:f32,

	/// NAME: Face creation texture data 30 - 顔作成テクスチャデータ30
	/// DESC: Face creation texture data 30 - 顔作成テクスチャデータ30
	pub faceTexData30:f32,

	/// NAME: Face creation texture data 31 - 顔作成テクスチャデータ31
	/// DESC: Face creation texture data 31 - 顔作成テクスチャデータ31
	pub faceTexData31:f32,

	/// NAME: Face creation texture data 32 - 顔作成テクスチャデータ32
	/// DESC: Face creation texture data 32 - 顔作成テクスチャデータ32
	pub faceTexData32:f32,

	/// NAME: Face creation texture data 33 - 顔作成テクスチャデータ33
	/// DESC: Face creation texture data 33 - 顔作成テクスチャデータ33
	pub faceTexData33:f32,

	/// NAME: Face creation texture data 34 - 顔作成テクスチャデータ34
	/// DESC: Face creation texture data 34 - 顔作成テクスチャデータ34
	pub faceTexData34:f32,

	/// NAME: Face creation texture data 35 - 顔作成テクスチャデータ35
	/// DESC: Face creation texture data 35 - 顔作成テクスチャデータ35
	pub faceTexData35:f32,

	/// NAME: Burn scars - 火傷跡
	/// DESC: Burn scars - 火傷跡
	pub burn_scar:f32,
}

impl Paramdef for FACE_RANGE_PARAM_ST {
const NAME: &'static str = "FACE_RANGE_PARAM_ST";
const VERSION: u16 = 3;
}
