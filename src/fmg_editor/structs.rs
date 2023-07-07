#[derive(Copy, Clone)]
pub enum FmgId {
    TalkMsg = 1,
    BloodMsg = 2,
    MovieSubtitle = 3,
    GoodsName = 10,
    WeaponName = 11,
    ProtectorName = 12,
    AccessoryName = 13,
    NpcName = 18,
    PlaceName = 19,
    WeaponInfo = 20,
    GoodsInfo = 21,
    ProtectorInfo = 22,
    AccessoryInfo = 23,
    GoodsCaption = 24,
    WeaponCaption = 25,
    ProtectorCaption = 26,
    AccessoryCaption = 27,
    MagicInfo = 28,
    MagicCaption = 29,
    NetworkMessage = 31,
    ActionButtonText = 32,
    EventTextForTalk = 33,
    EventTextForMap = 34,
    GemName = 35,
    GemInfo = 36,
    GemCaption = 37,
    ArtsName = 42,
    ArtsCaption = 43,
    WeaponEffect = 44,
    GemEffect = 45,
    GoodsInfo2 = 46,
    GrMenuText = 200,
    GrLineHelp = 201,
    GrKeyGuide = 202,
    GrSystemMessageWin64 = 203,
    GrDialogues = 204,
    LoadingTitle = 205,
    LoadingText = 206,
    TutorialTitle = 207,
    TutorialBody = 208,
    TextEmbedImageNameWin64 = 209,
    ToSWin64 = 210,
}
#[repr(C)]
pub(super) struct MsgRepositoryImp {
    pub vtable: usize,
    pub categories: *mut *mut &'static mut MsgRepositoryCategory,
    pub version_count: u32,
    pub category_capacity: u32,
    pub unk0x18: usize,
    pub unk0x20: usize,
    pub unk0x28: usize,
    pub unk0x30: usize,
    pub unk0x38: u32,
    pub unk0x3c: u32,
    pub unk0x40: u32,
    pub unk0x44: u32,
}

impl MsgRepositoryImp {
    pub(super) unsafe fn get_version_array(
        &self,
    ) -> &'static [*mut &'static MsgRepositoryCategory] {
        std::slice::from_raw_parts(self.categories as _, self.version_count as usize)
    }
    pub(super) unsafe fn get_version_array_mut(
        &self,
    ) -> &'static mut [*mut &'static mut MsgRepositoryCategory] {
        std::slice::from_raw_parts_mut(self.categories, self.version_count as usize)
    }
    pub(super) unsafe fn get_category_array(
        &self,
        version: usize,
    ) -> &'static [&'static MsgRepositoryCategory] {
        std::slice::from_raw_parts(
            self.get_version_array()[version],
            self.category_capacity as usize,
        )
    }
    pub(super) unsafe fn get_category_array_mut(
        &self,
        version: usize,
    ) -> &'static mut [&'static mut MsgRepositoryCategory] {
        std::slice::from_raw_parts_mut(
            self.get_version_array_mut()[version],
            self.category_capacity as usize,
        )
    }
    pub unsafe fn get_category(
        &self,
        version: usize,
        fmg: FmgId,
    ) -> &'static MsgRepositoryCategory {
        self.get_category_array(version)[fmg as usize]
    }
    pub unsafe fn get_category_mut(
        &self,
        version: usize,
        fmg: FmgId,
    ) -> &'static mut MsgRepositoryCategory {
        self.get_category_array_mut(version)[fmg as usize]
    }
}

#[repr(C)]
pub struct MsgRepositoryCategory {
    pub undefined0x0: u8,
    pub big_endian: bool,
    pub version: u8,
    pub undefined0x3: u8,
    pub file_size: u32,
    pub undefined0x8: u8,
    pub undefined0x9: u8,
    pub undefined0xa: u8,
    pub undefined0xb: u8,
    pub group_count: u32,
    pub string_count: u32,
    pub is_0xff: u32,
    pub string_offset: *mut usize,
    pub undefined0x20: u8,
    pub undefined0x21: u8,
    pub undefined0x22: u8,
    pub undefined0x23: u8,
    pub undefined0x24: u8,
    pub undefined0x25: u8,
    pub undefined0x26: u8,
    pub undefined0x27: u8,
}
#[repr(C)]
pub struct MsgRepositoryGroup {
    pub index: i32,
    pub first_id: i32,
    pub last_id: i32,
    pub pad0xc: i32,
}

#[cfg(test)]
mod tests {
    use crate::fmg_editor::structs::*;
    use std::mem::size_of;

    #[test]
    fn size_check() {
        assert_eq!(size_of::<MsgRepositoryImp>(), 0x48);
        assert_eq!(size_of::<MsgRepositoryCategory>(), 0x28);
        assert_eq!(size_of::<MsgRepositoryGroup>(), 0x10);
    }
}
