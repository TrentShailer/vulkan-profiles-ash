use core::mem;

use ash::{prelude::VkResult, vk, RawPtr};

use crate::vp;

pub struct Entry {
    entry_fn: EntryFn,
}

impl Entry {
    pub fn linked() -> Self {
        Self {
            entry_fn: EntryFn::linked(),
        }
    }

    pub unsafe fn create_capabilities(
        &self,
        capabilities_create_info: &vp::CapabilitiesCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<crate::Capabilities> {
        let mut handle = mem::MaybeUninit::uninit();
        let handle = (self.entry_fn.create_capabilities)(
            capabilities_create_info,
            allocation_callbacks.as_raw_ptr(),
            handle.as_mut_ptr(),
        )
        .assume_init_on_success(handle)?;

        Ok(crate::Capabilities::linked(handle))
    }
}

pub struct EntryFn {
    pub create_capabilities: vp::PFN_vpCreateCapabilities,
}
impl EntryFn {
    pub fn linked() -> Self {
        Self {
            create_capabilities: vp::linked::vpCreateCapabilities,
        }
    }
}
