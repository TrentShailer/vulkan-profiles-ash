use ash::{prelude::VkResult, vk, RawPtr};

use crate::vp;

/// The Vulkan Profiles entry, roughly equivalent to an [ash::Entry].
pub struct Entry {
    entry_fn: EntryFn,
}

impl Entry {
    /// Create the [`Entry`] object using a statically linked function pointers.
    #[cfg(feature = "linked")]
    pub fn linked() -> Self {
        Self {
            entry_fn: EntryFn::linked(),
        }
    }

    /// Creates allocator object.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#basic-usage>
    #[cfg(feature = "linked")]
    pub unsafe fn create_capabilities(
        &self,
        capabilities_create_info: &vp::CapabilitiesCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<crate::Capabilities> {
        let mut handle = core::mem::MaybeUninit::uninit();
        let handle = (self.entry_fn.create_capabilities)(
            capabilities_create_info,
            allocation_callbacks.as_raw_ptr(),
            handle.as_mut_ptr(),
        )
        .assume_init_on_success(handle)?;

        Ok(crate::Capabilities::linked(handle))
    }
}

#[derive(Clone)]
/// Function pointer table for the [`Entry`].
pub struct EntryFn {
    pub create_capabilities: vp::PFN_vpCreateCapabilities,
}

impl EntryFn {
    /// Load the function pointers from the statically linked library.
    #[cfg(feature = "linked")]
    pub(crate) fn linked() -> Self {
        Self {
            create_capabilities: vp::linked::vpCreateCapabilities,
        }
    }
}
