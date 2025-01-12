use ash::vk;

use crate::{
    utils::debug_flags,
    vp::{CapabilitiesCreateFlags, DeviceCreateFlags, InstanceCreateFlags},
};

impl core::fmt::Debug for InstanceCreateFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        const KNOWN: &[(vk::Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}

impl core::fmt::Debug for DeviceCreateFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        const KNOWN: &[(vk::Flags, &str)] = &[
            (
                DeviceCreateFlags::DISABLE_ROBUST_ACCESS.0,
                "DISABLE_ROBUST_ACCESS",
            ),
            (
                DeviceCreateFlags::DISABLE_ROBUST_BUFFER_ACCESS.0,
                "DISABLE_ROBUST_BUFFER_ACCESS",
            ),
            (
                DeviceCreateFlags::DISABLE_ROBUST_IMAGE_ACCESS.0,
                "DISABLE_ROBUST_IMAGE_ACCESS",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}

impl core::fmt::Debug for CapabilitiesCreateFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        const KNOWN: &[(vk::Flags, &str)] = &[(CapabilitiesCreateFlags::STATIC.0, "STATIC")];
        debug_flags(f, KNOWN, self.0)
    }
}
