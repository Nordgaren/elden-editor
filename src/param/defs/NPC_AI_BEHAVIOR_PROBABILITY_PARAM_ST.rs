/* This file was automatically generated from XML paramdefs. */
/// Data Version: 2
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST {

	/// NAME: Right hand proximity_R1 combo - 右手近接_R1コンボ
	/// DESC: Right-handed proximity behavior - 右手近接行動
	pub param000:i16,

	/// NAME: Right hand proximity_R2 combo - 右手近接_R2コンボ
	/// DESC: Right-handed proximity behavior - 右手近接行動
	pub param001:i16,

	/// NAME: Right-handed melee_dash attack - 右手近接_ダッシュ攻撃
	/// DESC: Right-handed proximity behavior - 右手近接行動
	pub param002:i16,

	/// NAME: Right-handed melee_front rolling attack - 右手近接_前ローリング攻撃
	/// DESC: Right-handed proximity behavior - 右手近接行動
	pub param003:i16,

	/// NAME: Right hand proximity_left and right rolling attack - 右手近接_左右ローリング攻撃
	/// DESC: Right-handed proximity behavior - 右手近接行動
	pub param004:i16,

	/// NAME: Right-handed melee_rear rolling attack - 右手近接_後ローリング攻撃
	/// DESC: Right-handed proximity behavior - 右手近接行動
	pub param005:i16,

	/// NAME: Right-handed melee_backstep attack - 右手近接_バックステップ攻撃
	/// DESC: Right-handed proximity behavior - 右手近接行動
	pub param006:i16,

	/// NAME: Right-handed melee_jump attack - 右手近接_ジャンプ攻撃
	/// DESC: Right-handed proximity behavior - 右手近接行動
	pub param007:i16,

	/// NAME: Right-handed melee_dash jump attack - 右手近接_ダッシュジャンプ攻撃
	/// DESC: Right-handed proximity behavior - 右手近接行動
	pub param008:i16,

	/// NAME: unused - 未使用
	pub param009:i16,

	/// NAME: Right hand long range _R1 shooting - 右手遠距離_R1射撃
	/// DESC: Right-handed long-range weapon action - 右手遠距離武器行動
	pub param010:i16,

	/// NAME: Right hand long range _R2 shooting - 右手遠距離_R2射撃
	/// DESC: Right-handed long-range weapon action - 右手遠距離武器行動
	pub param011:i16,

	/// NAME: unused - 未使用
	pub param012:i16,

	/// NAME: Right hand magic _ long distance magic _ move forward - 右手魔法_遠距離魔法_前移動
	/// DESC: Right hand magic action - 右手魔法行動
	pub param013:i16,

	/// NAME: Right-handed magic_long-distance magic_moving backwards - 右手魔法_遠距離魔法_後ろ移動
	/// DESC: Right hand magic action - 右手魔法行動
	pub param014:i16,

	/// NAME: Move left / right [Close Draw Bow] - 左右移動[CloseDrawBow]
	/// DESC: Interrupt approached while holding a bow - 弓構え中に近づかれたインタラプト
	pub param015:i16,

	/// NAME: Left and right rolling [Close Draw Bow] - 左右ローリング[CloseDrawBow]
	/// DESC: Interrupt approached while holding a bow - 弓構え中に近づかれたインタラプト
	pub param016:i16,

	/// NAME: Do not interrupt [Close Draw Bow] - 割り込まない[CloseDrawBow]
	/// DESC: Interrupt approached while holding a bow - 弓構え中に近づかれたインタラプト
	pub param017:i16,

	/// NAME: unused - 未使用
	pub param018:i16,

	/// NAME: unused - 未使用
	pub param019:i16,

	/// NAME: Right hand magic_Melee magic_Left and right - 右手魔法_近接魔法_左右
	/// DESC: Right hand magic action - 右手魔法行動
	pub param020:i16,

	/// NAME: Right hand magic_medium range magic_left and right - 右手魔法_中距離魔法_左右
	/// DESC: Right hand magic action - 右手魔法行動
	pub param021:i16,

	/// NAME: Right hand magic _ long distance magic _ left and right - 右手魔法_遠距離魔法_左右
	/// DESC: Right hand magic action - 右手魔法行動
	pub param022:i16,

	/// NAME: Right hand magic _HP recovery magic - 右手魔法_HP回復魔法
	/// DESC: Right hand magic action - 右手魔法行動
	pub param023:i16,

	/// NAME: Right hand magic_buff magic - 右手魔法_バフ魔法
	/// DESC: Right hand magic action - 右手魔法行動
	pub param024:i16,

	/// NAME: Right hand magic_Melee magic_Move forward - 右手魔法_近接魔法_前移動
	/// DESC: Right hand magic action - 右手魔法行動
	pub param025:i16,

	/// NAME: Right hand magic_Melee magic_Move backwards - 右手魔法_近接魔法_後ろ移動
	/// DESC: Right hand magic action - 右手魔法行動
	pub param026:i16,

	/// NAME: Right-handed magic_Medium-range magic_Move forward - 右手魔法_中距離魔法_前移動
	/// DESC: Right hand magic action - 右手魔法行動
	pub param027:i16,

	/// NAME: Right-handed magic_Medium-range magic_Move backwards - 右手魔法_中距離魔法_後ろ移動
	/// DESC: Right hand magic action - 右手魔法行動
	pub param028:i16,

	/// NAME: unused - 未使用
	pub param029:i16,

	/// NAME: Right hand shield_R1 attack - 右手盾_R1攻撃
	/// DESC: Right hand shield action - 右手盾行動
	pub param030:i16,

	/// NAME: Right hand shield_R2 attack - 右手盾_R2攻撃
	/// DESC: Right hand shield action - 右手盾行動
	pub param031:i16,

	/// NAME: unused - 未使用
	pub param032:i16,

	/// NAME: unused - 未使用
	pub param033:i16,

	/// NAME: unused - 未使用
	pub param034:i16,

	/// NAME: Backstep [SuccessGuard] - バックステップ[SuccessGuard]
	/// DESC: Guard success interrupt - ガード成功インタラプト
	pub param035:i16,

	/// NAME: Do not interrupt [SuccessGuard] - 割り込まない[SuccessGuard]
	/// DESC: Guard success interrupt - ガード成功インタラプト
	pub param036:i16,

	/// NAME: Guard Counter Attack [SuccessGuard] - ガードカウンター攻撃[SuccessGuard]
	/// DESC: Guard success interrupt - ガード成功インタラプト
	pub param037:i16,

	/// NAME: unused - 未使用
	pub param038:i16,

	/// NAME: unused - 未使用
	pub param039:i16,

	/// NAME: Left hand proximity_R1 combo (both hands only) - 左手近接_R1コンボ(両手持ちのみ)
	/// DESC: Left hand proximity behavior - 左手近接行動
	pub param040:i16,

	/// NAME: Left hand proximity_R2 combo (both hands only) - 左手近接_R2コンボ(両手持ちのみ)
	/// DESC: Left hand proximity behavior - 左手近接行動
	pub param041:i16,

	/// NAME: Left-handed proximity_L1 combo (one-handed only) - 左手近接_L1コンボ(片手持ちのみ)
	/// DESC: Left hand proximity behavior - 左手近接行動
	pub param042:i16,

	/// NAME: Left-handed melee_dash attack (both hands only) - 左手近接_ダッシュ攻撃(両手持ちのみ)
	/// DESC: Left hand proximity behavior - 左手近接行動
	pub param043:i16,

	/// NAME: Left hand proximity_front rolling attack (both hands only) - 左手近接_前ローリング攻撃(両手持ちのみ)
	/// DESC: Left hand proximity behavior - 左手近接行動
	pub param044:i16,

	/// NAME: Left hand proximity_left and right rolling attack (only with both hands) - 左手近接_左右ローリング攻撃(両手持ちのみ)
	/// DESC: Left hand proximity behavior - 左手近接行動
	pub param045:i16,

	/// NAME: Left hand proximity_rear rolling attack (only with both hands) - 左手近接_後ローリング攻撃(両手持ちのみ)
	/// DESC: Left hand proximity behavior - 左手近接行動
	pub param046:i16,

	/// NAME: Left-handed melee_backstep attack (only with both hands) - 左手近接_バックステップ攻撃(両手持ちのみ)
	/// DESC: Left hand proximity behavior - 左手近接行動
	pub param047:i16,

	/// NAME: Left-handed melee_jump attack (only with both hands) - 左手近接_ジャンプ攻撃(両手持ちのみ)
	/// DESC: Left hand proximity behavior - 左手近接行動
	pub param048:i16,

	/// NAME: Left-handed melee_dash jump attack (only with both hands) - 左手近接_ダッシュジャンプ攻撃(両手持ちのみ)
	/// DESC: Left hand proximity behavior - 左手近接行動
	pub param049:i16,

	/// NAME: Left hand long range _R1 shooting - 左手遠距離_R1射撃
	/// DESC: Left-handed long-range weapon action - 左手遠距離武器行動
	pub param050:i16,

	/// NAME: Left hand long range _R2 shooting - 左手遠距離_R2射撃
	/// DESC: Left-handed long-range weapon action - 左手遠距離武器行動
	pub param051:i16,

	/// NAME: unused - 未使用
	pub param052:i16,

	/// NAME: Left hand magic _ long distance magic _ move forward - 左手魔法_遠距離魔法_前移動
	/// DESC: Left hand magic action - 左手魔法行動
	pub param053:i16,

	/// NAME: Left hand magic _ long distance magic _ move backward - 左手魔法_遠距離魔法_後ろ移動
	/// DESC: Left hand magic action - 左手魔法行動
	pub param054:i16,

	/// NAME: Fatal Attack [Success Parry] - 致命攻撃[SuccessParry]
	/// DESC: Parry Success Interrupt - パリィ成功インタラプト
	pub param055:i16,

	/// NAME: Do not interrupt [Success Parry] - 割り込まない[SuccessParry]
	/// DESC: Parry Success Interrupt - パリィ成功インタラプト
	pub param056:i16,

	/// NAME: unused - 未使用
	pub param057:i16,

	/// NAME: unused - 未使用
	pub param058:i16,

	/// NAME: unused - 未使用
	pub param059:i16,

	/// NAME: Left hand magic_Melee magic_Left and right - 左手魔法_近接魔法_左右
	/// DESC: Left hand magic action - 左手魔法行動
	pub param060:i16,

	/// NAME: Left hand magic_medium range magic_left and right - 左手魔法_中距離魔法_左右
	/// DESC: Left hand magic action - 左手魔法行動
	pub param061:i16,

	/// NAME: Left hand magic _ long distance magic _ left and right - 左手魔法_遠距離魔法_左右
	/// DESC: Left hand magic action - 左手魔法行動
	pub param062:i16,

	/// NAME: Left hand magic _HP recovery magic - 左手魔法_HP回復魔法
	/// DESC: Left hand magic action - 左手魔法行動
	pub param063:i16,

	/// NAME: Left hand magic_buff magic - 左手魔法_バフ魔法
	/// DESC: Left hand magic action - 左手魔法行動
	pub param064:i16,

	/// NAME: Left hand magic_Melee magic_Move forward - 左手魔法_近接魔法_前移動
	/// DESC: Left hand magic action - 左手魔法行動
	pub param065:i16,

	/// NAME: Left hand magic_Melee magic_Move backward - 左手魔法_近接魔法_後ろ移動
	/// DESC: Left hand magic action - 左手魔法行動
	pub param066:i16,

	/// NAME: Left hand magic_medium range magic_move forward - 左手魔法_中距離魔法_前移動
	/// DESC: Left hand magic action - 左手魔法行動
	pub param067:i16,

	/// NAME: Left hand magic_medium range magic_moving backwards - 左手魔法_中距離魔法_後ろ移動
	/// DESC: Left hand magic action - 左手魔法行動
	pub param068:i16,

	/// NAME: unused - 未使用
	pub param069:i16,

	/// NAME: backstep - バックステップ
	/// DESC: Common behavior - 共通行動
	pub param070:i16,

	/// NAME: Front rolling - 前ローリング
	/// DESC: Common behavior - 共通行動
	pub param071:i16,

	/// NAME: Left and right rolling - 左右ローリング
	/// DESC: Common behavior - 共通行動
	pub param072:i16,

	/// NAME: Rolling behind - 後ろローリング
	/// DESC: Common behavior - 共通行動
	pub param073:i16,

	/// NAME: Move left and right - 左右移動
	/// DESC: Common behavior - 共通行動
	pub param074:i16,

	/// NAME: Recession - 後退
	/// DESC: Common behavior - 共通行動
	pub param075:i16,

	/// NAME: Dash approach - ダッシュ接近
	/// DESC: Common behavior - 共通行動
	pub param076:i16,

	/// NAME: stand-by - 待機
	/// DESC: Common behavior - 共通行動
	pub param077:i16,

	/// NAME: Switch to two-handed right hand - 右手両手持ちに持ち替え
	/// DESC: Common behavior - 共通行動
	pub param078:i16,

	/// NAME: Switch to two-handed left hand - 左手両手持ちに持ち替え
	/// DESC: Common behavior - 共通行動
	pub param079:i16,

	/// NAME: Switch to one-handed - 片手持ちに持ち替え
	/// DESC: Common behavior - 共通行動
	pub param080:i16,

	/// NAME: Approach to combat distance - 戦闘距離までアプローチ
	/// DESC: Common behavior - 共通行動
	pub param081:i16,

	/// NAME: unused - 未使用
	pub param082:i16,

	/// NAME: unused - 未使用
	pub param083:i16,

	/// NAME: unused - 未使用
	pub param084:i16,

	/// NAME: Deadly Attack [Guard Break] - 致命攻撃[GuardBreak]
	/// DESC: Guard break interrupt - ガードブレイクインタラプト
	pub param085:i16,

	/// NAME: Dash Attack [Guard Break] - ダッシュ攻撃[GuardBreak]
	/// DESC: Guard break interrupt - ガードブレイクインタラプト
	pub param086:i16,

	/// NAME: Magic_Melee Magic [GuardBreak] - 魔法_近接魔法[GuardBreak]
	/// DESC: Guard break interrupt - ガードブレイクインタラプト
	pub param087:i16,

	/// NAME: Do not interrupt [Guard Break] - 割り込まない[GuardBreak]
	/// DESC: Guard break interrupt - ガードブレイクインタラプト
	pub param088:i16,

	/// NAME: unused - 未使用
	pub param089:i16,

	/// NAME: Shield Chiku - 盾チク
	/// DESC: Special behavior - 特殊行動
	pub param090:i16,

	/// NAME: unused - 未使用
	pub param091:i16,

	/// NAME: unused - 未使用
	pub param092:i16,

	/// NAME: unused - 未使用
	pub param093:i16,

	/// NAME: unused - 未使用
	pub param094:i16,

	/// NAME: Dash Attack [Estus] - ダッシュ攻撃[Estus]
	/// DESC: Interrupt who drank Est - エストを飲んだインタラプト
	pub param095:i16,

	/// NAME: Throwing item [Estus] - 投擲アイテム[Estus]
	/// DESC: Interrupt who drank Est - エストを飲んだインタラプト
	pub param096:i16,

	/// NAME: Do not interrupt [Estus] - 割り込まない[Estus]
	/// DESC: Interrupt who drank Est - エストを飲んだインタラプト
	pub param097:i16,

	/// NAME: unused - 未使用
	pub param098:i16,

	/// NAME: unused - 未使用
	pub param099:i16,

	/// NAME: Parry time - パリィ[Parrytime]
	/// DESC: Parry Timing Interrupt - パリィタイミングインタラプト
	pub param100:i16,

	/// NAME: Front rolling [Parrytime] - 前ローリング[Parrytime]
	/// DESC: Parry Timing Interrupt - パリィタイミングインタラプト
	pub param101:i16,

	/// NAME: Left and right rolling [Parrytime] - 左右ローリング[Parrytime]
	/// DESC: Parry Timing Interrupt - パリィタイミングインタラプト
	pub param102:i16,

	/// NAME: Rolling behind [Parrytime] - 後ろローリング[Parrytime]
	/// DESC: Parry Timing Interrupt - パリィタイミングインタラプト
	pub param103:i16,

	/// NAME: Backstep attack [Parrytime] - バックステップ攻撃[Parrytime]
	/// DESC: Parry Timing Interrupt - パリィタイミングインタラプト
	pub param104:i16,

	/// NAME: On-the-spot guard [Parrytime] - その場ガード[Parrytime]
	/// DESC: Parry Timing Interrupt - パリィタイミングインタラプト
	pub param105:i16,

	/// NAME: Do not interrupt [Parrytime] - 割り込まない[Parrytime]
	/// DESC: Parry Timing Interrupt - パリィタイミングインタラプト
	pub param106:i16,

	/// NAME: unused - 未使用
	pub param107:i16,

	/// NAME: unused - 未使用
	pub param108:i16,

	/// NAME: unused - 未使用
	pub param109:i16,

	/// NAME: Rolling behind [Damaged] - 後ろローリング[Damaged]
	/// DESC: Damaged interrupt - 被ダメージインタラプト
	pub param110:i16,

	/// NAME: Left and right rolling [Damaged] - 左右ローリング[Damaged]
	/// DESC: Damaged interrupt - 被ダメージインタラプト
	pub param111:i16,

	/// NAME: Back guard move [Damaged] - 後ろガード移動[Damaged]
	/// DESC: Damaged interrupt - 被ダメージインタラプト
	pub param112:i16,

	/// NAME: Left and right guard movement [Damaged] - 左右ガード移動[Damaged]
	/// DESC: Damaged interrupt - 被ダメージインタラプト
	pub param113:i16,

	/// NAME: Do not interrupt [Damaged] - 割り込まない[Damaged]
	/// DESC: Damaged interrupt - 被ダメージインタラプト
	pub param114:i16,

	/// NAME: R1 Combo [Damaged] - R1コンボ[Damaged]
	/// DESC: Damaged interrupt - 被ダメージインタラプト
	pub param115:i16,

	/// NAME: Front rolling [Damaged] - 前ローリング[Damaged]
	/// DESC: Damaged interrupt - 被ダメージインタラプト
	pub param116:i16,

	/// NAME: Backstep [Damaged] - バックステップ[Damaged]
	/// DESC: Damaged interrupt - 被ダメージインタラプト
	pub param117:i16,

	/// NAME: Front guard move [Damaged] - 前ガード移動[Damaged]
	/// DESC: Damaged interrupt - 被ダメージインタラプト
	pub param118:i16,

	/// NAME: unused - 未使用
	pub param119:i16,

	/// NAME: Front rolling [Shoot] - 前ローリング[Shoot]
	/// DESC: Projectile discovery interrupt - 飛び道具発見インタラプト
	pub param120:i16,

	/// NAME: Left and right rolling [Shoot] - 左右ローリング[Shoot]
	/// DESC: Projectile discovery interrupt - 飛び道具発見インタラプト
	pub param121:i16,

	/// NAME: Front guard move [Shoot] - 前ガード移動[Shoot]
	/// DESC: Projectile discovery interrupt - 飛び道具発見インタラプト
	pub param122:i16,

	/// NAME: Left and right guard movement [Shoot] - 左右ガード移動[Shoot]
	/// DESC: Projectile discovery interrupt - 飛び道具発見インタラプト
	pub param123:i16,

	/// NAME: Do not interrupt [Shoot] - 割り込まない[Shoot]
	/// DESC: Projectile discovery interrupt - 飛び道具発見インタラプト
	pub param124:i16,

	/// NAME: After rolling [Shoot] - 後ローリング[Shoot]
	/// DESC: Projectile discovery interrupt - 飛び道具発見インタラプト
	pub param125:i16,

	/// NAME: Dash approach [Shoot] - ダッシュ接近[Shoot]
	/// DESC: Projectile discovery interrupt - 飛び道具発見インタラプト
	pub param126:i16,

	/// NAME: unused - 未使用
	pub param127:i16,

	/// NAME: unused - 未使用
	pub param128:i16,

	/// NAME: unused - 未使用
	pub param129:i16,

	/// NAME: Backstep [Find Attack] - バックステップ[FindAttack]
	/// DESC: Attack detection interrupt - 攻撃発見インタラプト
	pub param130:i16,

	/// NAME: Front rolling [Find Attack] - 前ローリング[FindAttack]
	/// DESC: Attack detection interrupt - 攻撃発見インタラプト
	pub param131:i16,

	/// NAME: Left and right rolling [Find Attack] - 左右ローリング[FindAttack]
	/// DESC: Attack detection interrupt - 攻撃発見インタラプト
	pub param132:i16,

	/// NAME: After rolling [Find Attack] - 後ローリング[FindAttack]
	/// DESC: Attack detection interrupt - 攻撃発見インタラプト
	pub param133:i16,

	/// NAME: Front guard move [Find Attack] - 前ガード移動[FindAttack]
	/// DESC: Attack detection interrupt - 攻撃発見インタラプト
	pub param134:i16,

	/// NAME: Left and right guard movement [Find Attack] - 左右ガード移動[FindAttack]
	/// DESC: Attack detection interrupt - 攻撃発見インタラプト
	pub param135:i16,

	/// NAME: On-the-spot guard [Find Attack] - その場ガード[FindAttack]
	/// DESC: Attack detection interrupt - 攻撃発見インタラプト
	pub param136:i16,

	/// NAME: Shield Chiku [Find Attack] - 盾チク[FindAttack]
	/// DESC: Attack detection interrupt - 攻撃発見インタラプト
	pub param137:i16,

	/// NAME: Do not interrupt [Find Attack] - 割り込まない[FindAttack]
	/// DESC: Attack detection interrupt - 攻撃発見インタラプト
	pub param138:i16,

	/// NAME: On-the-spot avoidance arts [Find Attack] - その場回避アーツ[FindAttack]
	/// DESC: Attack detection interrupt - 攻撃発見インタラプト
	pub param139:i16,

	/// NAME: Probability of holding both hands at R1 combo - R1コンボ時両手持ち確率
	/// DESC: Other probability corrections - その他の確率補正
	pub param140:i16,

	/// NAME: Probability of holding both hands at R2 combo - R2コンボ時両手持ち確率
	/// DESC: Other probability corrections - その他の確率補正
	pub param141:i16,

	/// NAME: Guard probability when moving during planning action - プランニング行動時の移動時ガード確率
	/// DESC: Other probability corrections - その他の確率補正
	pub param142:i16,

	/// NAME: Probability of replacing R1 attack with dual wield attack - R1攻撃を二刀攻撃に置き換える確率
	/// DESC: Other probability corrections - その他の確率補正
	pub param143:i16,

	/// NAME: Probability of replacing evasion with special rolling arts - 回避を特殊ローリングアーツに置き換える確率
	/// DESC: Other probability corrections - その他の確率補正
	pub param144:i16,

	/// NAME: Use of throwing items - 投擲系アイテムの使用
	/// DESC: Item usage system - アイテム使用系
	pub param145:i16,

	/// NAME: Use of HP recovery items - HP回復アイテムの使用
	/// DESC: Item usage system - アイテム使用系
	pub param146:i16,

	/// NAME: Use of buff items - バフアイテムの使用
	/// DESC: Item usage system - アイテム使用系
	pub param147:i16,

	/// NAME: Drug package combo attack - 薬包コンボ攻撃
	/// DESC: Item usage system - アイテム使用系
	pub param148:i16,

	/// NAME: unused - 未使用
	pub param149:i16,

	/// NAME: Arts_Short Range Attack - アーツ_近距離攻撃
	/// DESC: Arts behavior system - アーツ行動系
	pub param150:i16,

	/// NAME: Arts_Medium Range Attack - アーツ_中距離攻撃
	/// DESC: Arts behavior system - アーツ行動系
	pub param151:i16,

	/// NAME: Arts_range attack - アーツ_遠距離攻撃
	/// DESC: Arts behavior system - アーツ行動系
	pub param152:i16,

	/// NAME: Arts_Recovery - アーツ_回復
	/// DESC: Arts behavior system - アーツ行動系
	pub param153:i16,

	/// NAME: Arts_buff - アーツ_バフ
	/// DESC: Arts behavior system - アーツ行動系
	pub param154:i16,

	/// NAME: unused - 未使用
	/// DESC: Arts behavior system - アーツ行動系
	pub param155:i16,

	/// NAME: unused - 未使用
	pub param156:i16,

	/// NAME: unused - 未使用
	pub param157:i16,

	/// NAME: unused - 未使用
	pub param158:i16,

	/// NAME: unused - 未使用
	pub param159:i16,

	/// NAME: unused - 未使用
	pub param160:i16,

	/// NAME: unused - 未使用
	pub param161:i16,

	/// NAME: unused - 未使用
	pub param162:i16,

	/// NAME: unused - 未使用
	pub param163:i16,

	/// NAME: unused - 未使用
	pub param164:i16,

	/// NAME: unused - 未使用
	pub param165:i16,

	/// NAME: unused - 未使用
	pub param166:i16,

	/// NAME: unused - 未使用
	pub param167:i16,

	/// NAME: unused - 未使用
	pub param168:i16,

	/// NAME: unused - 未使用
	pub param169:i16,

	/// NAME: unused - 未使用
	pub param170:i16,

	/// NAME: unused - 未使用
	pub param171:i16,

	/// NAME: unused - 未使用
	pub param172:i16,

	/// NAME: unused - 未使用
	pub param173:i16,

	/// NAME: unused - 未使用
	pub param174:i16,

	/// NAME: unused - 未使用
	pub param175:i16,

	/// NAME: unused - 未使用
	pub param176:i16,

	/// NAME: unused - 未使用
	pub param177:i16,

	/// NAME: unused - 未使用
	pub param178:i16,

	/// NAME: unused - 未使用
	pub param179:i16,

	/// NAME: unused - 未使用
	pub param180:i16,

	/// NAME: unused - 未使用
	pub param181:i16,

	/// NAME: unused - 未使用
	pub param182:i16,

	/// NAME: unused - 未使用
	pub param183:i16,

	/// NAME: unused - 未使用
	pub param184:i16,

	/// NAME: unused - 未使用
	pub param185:i16,

	/// NAME: unused - 未使用
	pub param186:i16,

	/// NAME: unused - 未使用
	pub param187:i16,

	/// NAME: unused - 未使用
	pub param188:i16,

	/// NAME: unused - 未使用
	pub param189:i16,

	/// NAME: unused - 未使用
	pub param190:i16,

	/// NAME: unused - 未使用
	pub param191:i16,

	/// NAME: unused - 未使用
	pub param192:i16,

	/// NAME: unused - 未使用
	pub param193:i16,

	/// NAME: unused - 未使用
	pub param194:i16,

	/// NAME: unused - 未使用
	pub param195:i16,

	/// NAME: unused - 未使用
	pub param196:i16,

	/// NAME: unused - 未使用
	pub param197:i16,

	/// NAME: unused - 未使用
	pub param198:i16,

	/// NAME: unused - 未使用
	pub param199:i16,
}

