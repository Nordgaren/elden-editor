/* This file was automatically generated from XML paramdefs. */
use crate::param::traits::Paramdef;
/// Data Version: 3
/// Is Big Endian: False
/// Is Unicode: True
/// Format Version: 203
#[repr(C)]
pub struct NETWORK_MSG_PARAM_ST {
    /// NAME: priority - 優先度
    /// DESC: priority - 優先度
    pub priority: u16,

    /// NAME: Forced interrupt - 強制割り込み
    /// DESC: Forced interrupt - 強制割り込み
    pub forcePlay: u8,

    /// NAME: reserve - 予約
    /// DESC: reserve - 予約
    pub pad1: [u8; 1],

    /// NAME: White spirit (white sign) - 白霊（白サイン）
    /// DESC: White spirit (white sign) - 白霊（白サイン）
    pub normalWhite: i32,

    /// NAME: Sun spirit (white sign) - 太陽霊（白サイン）
    /// DESC: Sun spirit (white sign) - 太陽霊（白サイン）
    pub umbasaWhite: i32,

    /// NAME: Berserker spirit (white sign) - バーサーカー霊（白サイン）
    /// DESC: Berserker spirit (white sign) - バーサーカー霊（白サイン）
    pub berserkerWhite: i32,

    /// NAME: Sinner Hero Spirit (white sign) - 罪人英雄霊（白サイン ）
    /// DESC: Sinner Hero Spirit (white sign) - 罪人英雄霊（白サイン ）
    pub sinnerHeroWhite: i32,

    /// NAME: Dark spirit (red sign) - 闇霊（赤サイン）
    /// DESC: Dark spirit (red sign) - 闇霊（赤サイン）
    pub normalBlack: i32,

    /// NAME: Sun spirit (red sign) - 太陽霊（赤サイン）
    /// DESC: Sun spirit (red sign) - 太陽霊（赤サイン）
    pub umbasaBlack: i32,

    /// NAME: Berserker spirit (red sign) - バーサーカー霊（赤サイン）
    /// DESC: Berserker spirit (red sign) - バーサーカー霊（赤サイン）
    pub berserkerBlack: i32,

    /// NAME: Intrusion_A - 侵入_A
    /// DESC: Intrusion_A - 侵入_A
    pub forceJoinBlack: i32,

    /// NAME: Sun spirit (intrusion) - 太陽霊（乱入）
    /// DESC: Sun spirit (intrusion) - 太陽霊（乱入）
    pub forceJoinUmbasaBlack: i32,

    /// NAME: Berserker spirit (intrusion) - バーサーカー霊（乱入）
    /// DESC: Berserker spirit (intrusion) - バーサーカー霊（乱入）
    pub forceJoinBerserkerBlack: i32,

    /// NAME: Sinner hunting spirit (visit) - 罪人狩り霊（訪問）
    /// DESC: Sinner hunting spirit (visit) - 罪人狩り霊（訪問）
    pub sinnerHunterVisitor: i32,

    /// NAME: Red Scare Spirit (Visit) - 赤狩り霊（訪問）
    /// DESC: Red Scare Spirit (Visit) - 赤狩り霊（訪問）
    pub redHunterVisitor: i32,

    /// NAME: Boss guardian spirit (visit) - ボス守護霊（訪問）
    /// DESC: Boss guardian spirit (visit) - ボス守護霊（訪問）
    pub guardianOfBossVisitor: i32,

    /// NAME: Map Guardian Spirit_Forest (Visit) - マップ守護霊_森（訪問）
    /// DESC: Map Guardian Spirit_Forest (Visit) - マップ守護霊_森（訪問）
    pub guardianOfForestMapVisitor: i32,

    /// NAME: Map Guardian_Anor (Visit) - マップ守護霊_アノール（訪問）
    /// DESC: Map Guardian_Anor (Visit) - マップ守護霊_アノール（訪問）
    pub guardianOfAnolisVisitor: i32,

    /// NAME: Rosalia spirit (red sign) - ロザリア霊（赤サイン）
    /// DESC: Rosalia spirit (red sign) - ロザリア霊（赤サイン）
    pub rosaliaBlack: i32,

    /// NAME: Rosalia spirit (intrusion) - ロザリア霊（乱入）
    /// DESC: Rosalia spirit (intrusion) - ロザリア霊（乱入）
    pub forceJoinRosaliaBlack: i32,

    /// NAME: Red Scare Spirit 2 (Visit) - 赤狩り霊2（訪問）
    /// DESC: Red Scare Spirit 2 (Visit) - 赤狩り霊2（訪問）
    pub redHunterVisitor2: i32,

    /// NAME: NPC Pseudo Multi 1 - NPC擬似マルチ1
    /// DESC: NPC Pseudo Multi 1 - NPC擬似マルチ1
    pub npc1: i32,

    /// NAME: NPC Pseudo Multi 2 - NPC擬似マルチ2
    /// DESC: NPC Pseudo Multi 2 - NPC擬似マルチ2
    pub npc2: i32,

    /// NAME: NPC Pseudo Multi 3 - NPC擬似マルチ3
    /// DESC: NPC Pseudo Multi 3 - NPC擬似マルチ3
    pub npc3: i32,

    /// NAME: NPC Pseudo Multi 4 - NPC擬似マルチ4
    /// DESC: NPC Pseudo Multi 4 - NPC擬似マルチ4
    pub npc4: i32,

    /// NAME: Battle royale - バトルロイヤル
    /// DESC: Battle royale - バトルロイヤル
    pub battleRoyal: i32,

    /// NAME: NPC Pseudo Multi 5 - NPC擬似マルチ5
    /// DESC: NPC Pseudo Multi 5 - NPC擬似マルチ5
    pub npc5: i32,

    /// NAME: NPC Pseudo Multi 6 - NPC擬似マルチ6
    /// DESC: NPC Pseudo Multi 6 - NPC擬似マルチ6
    pub npc6: i32,

    /// NAME: NPC Pseudo Multi 7 - NPC擬似マルチ7
    /// DESC: NPC Pseudo Multi 7 - NPC擬似マルチ7
    pub npc7: i32,

    /// NAME: NPC Pseudo Multi 8 - NPC擬似マルチ8
    /// DESC: NPC Pseudo Multi 8 - NPC擬似マルチ8
    pub npc8: i32,

    /// NAME: NPC Pseudo Multi 9 - NPC擬似マルチ9
    /// DESC: NPC Pseudo Multi 9 - NPC擬似マルチ9
    pub npc9: i32,

    /// NAME: NPC Pseudo Multi 10 - NPC擬似マルチ10
    /// DESC: NPC Pseudo Multi 10 - NPC擬似マルチ10
    pub npc10: i32,

    /// NAME: NPC Pseudo Multi 11 - NPC擬似マルチ11
    /// DESC: NPC Pseudo Multi 11 - NPC擬似マルチ11
    pub npc11: i32,

    /// NAME: NPC Pseudo Multi 12 - NPC擬似マルチ12
    /// DESC: NPC Pseudo Multi 12 - NPC擬似マルチ12
    pub npc12: i32,

    /// NAME: NPC Pseudo Multi 13 - NPC擬似マルチ13
    /// DESC: NPC Pseudo Multi 13 - NPC擬似マルチ13
    pub npc13: i32,

    /// NAME: NPC Pseudo Multi 14 - NPC擬似マルチ14
    /// DESC: NPC Pseudo Multi 14 - NPC擬似マルチ14
    pub npc14: i32,

    /// NAME: NPC Pseudo Multi 15 - NPC擬似マルチ15
    /// DESC: NPC Pseudo Multi 15 - NPC擬似マルチ15
    pub npc15: i32,

    /// NAME: NPC Pseudo Multi 16 - NPC擬似マルチ16
    /// DESC: NPC Pseudo Multi 16 - NPC擬似マルチ16
    pub npc16: i32,

    /// NAME: Intrusion_B - 侵入_B
    /// DESC: Intrusion_B - 侵入_B
    pub forceJoinBlack_B: i32,

    /// NAME: White spirit (white sign) for NPCs - 白霊（白サイン）_NPC用
    /// DESC: White spirit (white sign) for NPCs - 白霊（白サイン）_NPC用
    pub normalWhite_Npc: i32,

    /// NAME: For intrusion_A_NPCs - 侵入_A_NPC用
    /// DESC: For intrusion_A_NPCs - 侵入_A_NPC用
    pub forceJoinBlack_Npc: i32,

    /// NAME: For intrusion_B_NPCs - 侵入_B_NPC用
    /// DESC: For intrusion_B_NPCs - 侵入_B_NPC用
    pub forceJoinBlack_B_Npc: i32,

    /// NAME: For intrusion_C_NPCs - 侵入_C_NPC用
    /// DESC: For intrusion_C_NPCs - 侵入_C_NPC用
    pub forceJoinBlack_C_Npc: i32,

    /// NAME: reserve - 予約
    /// DESC: reserve - 予約
    pub pad2: [u8; 28],
}

impl Paramdef for NETWORK_MSG_PARAM_ST {
    const NAME: &'static str = "NETWORK_MSG_PARAM_ST";
    const VERSION: u16 = 3;
}
