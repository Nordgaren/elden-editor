use std::ops::{Deref, DerefMut};
use crate::param_editor::structs::{IdRepositoryInfo, IdRepositoryInfoPtr, ParamHeader, ParamHeaderPtr, ParamInfo, ParamInfoPtr, ParamResCap, ParamResCapPtr, SoloParamRepository, SoloParamRepositoryPtr};

impl Default for SoloParamRepositoryPtr {
    fn default() -> Self {
        SoloParamRepositoryPtr {
            address: 0 as *mut SoloParamRepository,
        }
    }
}

impl Deref for SoloParamRepositoryPtr {
    type Target = SoloParamRepository;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.address }
    }
}

impl DerefMut for SoloParamRepositoryPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.address }
    }
}

impl Default for ParamHeaderPtr {
    fn default() -> Self {
        ParamHeaderPtr {
            address: 0 as *mut ParamHeader,
        }
    }
}

impl Deref for ParamHeaderPtr {
    type Target = ParamHeader;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.address }
    }
}

impl DerefMut for ParamHeaderPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.address }
    }
}

impl From<*mut ParamHeader> for ParamHeaderPtr {
    fn from(address: *mut ParamHeader) -> Self {
        ParamHeaderPtr { address }
    }
}

impl From<*const ParamHeader> for ParamHeaderPtr {
    fn from(address: *const ParamHeader) -> Self {
        ParamHeaderPtr {
            address: address as *mut ParamHeader,
        }
    }
}

impl Default for IdRepositoryInfoPtr {
    fn default() -> Self {
        IdRepositoryInfoPtr {
            address: 0 as *mut IdRepositoryInfo,
        }
    }
}

impl Deref for IdRepositoryInfoPtr {
    type Target = IdRepositoryInfo;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.address }
    }
}

impl DerefMut for IdRepositoryInfoPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.address }
    }
}

impl From<ParamHeaderPtr> for IdRepositoryInfoPtr {
    fn from(param_header: ParamHeaderPtr) -> Self {
        unsafe {
            let addr = param_header.address as *mut IdRepositoryInfo;
            IdRepositoryInfoPtr { address: addr.sub(1) }
        }
    }
}

impl Deref for ParamInfoPtr {
    type Target = ParamInfo;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.address }
    }
}

impl DerefMut for ParamInfoPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.address }
    }
}

impl Deref for ParamResCapPtr {
    type Target = ParamResCap;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.address }
    }
}

impl DerefMut for ParamResCapPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.address }
    }
}


