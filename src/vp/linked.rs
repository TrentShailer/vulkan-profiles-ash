use core::ffi;

use ash::vk;

use crate::vp;

extern "system" {
    pub(crate) fn vpCreateCapabilities(
        pCreateInfo: *const vp::CapabilitiesCreateInfo<'_>,
        pAllocator: *const vk::AllocationCallbacks<'_>,
        pCapabilities: *mut vp::Capabilities,
    ) -> vk::Result;

    pub(crate) fn vpDestroyCapabilities(
        capabilities: vp::Capabilities,
        pAllocator: *const vk::AllocationCallbacks<'_>,
    ) -> ffi::c_void;

    pub(crate) fn vpGetProfiles(
        capabilities: vp::Capabilities,
        pPropertyCount: *mut u32,
        pProperties: *mut vp::ProfileProperties,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileRequiredProfiles(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
        pPropertyCount: *mut u32,
        pProperties: *mut vp::ProfileProperties,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileAPIVersion(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
    ) -> u32;

    pub(crate) fn vpGetProfileFallbacks(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
        pPropertyCount: *mut u32,
        pProperties: *mut vp::ProfileProperties,
    ) -> vk::Result;

    pub(crate) fn vpHasMultipleVariantsProfile(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
        pHasMultipleVariants: *mut vk::Bool32,
    ) -> vk::Result;

    pub(crate) fn vpGetInstanceProfileSupport(
        capabilities: vp::Capabilities,
        pLayerName: *const ffi::c_char,
        pProfile: *const vp::ProfileProperties,
        pSupported: *mut vk::Bool32,
    ) -> vk::Result;

    pub(crate) fn vpGetInstanceProfileVariantsSupport(
        capabilities: vp::Capabilities,
        pLayerName: *const ffi::c_char,
        pProfile: *const vp::ProfileProperties,
        pSupported: *mut vk::Bool32,
        pPropertyCount: *mut u32,
        pProperties: *mut vp::BlockProperties,
    ) -> vk::Result;

    pub(crate) fn vpCreateInstance(
        capabilities: vp::Capabilities,
        pCreateInfo: *const vp::InstanceCreateInfo<'_>,
        pAllocator: *const vk::AllocationCallbacks<'_>,
        pInstance: *mut vk::Instance,
    ) -> vk::Result;

    pub(crate) fn vpGetPhysicalDeviceProfileSupport(
        capabilities: vp::Capabilities,
        instance: vk::Instance,
        physicalDevice: vk::PhysicalDevice,
        pProfile: *const vp::ProfileProperties,
        pSupported: *mut vk::Bool32,
    ) -> vk::Result;

    pub(crate) fn vpGetPhysicalDeviceProfileVariantsSupport(
        capabilities: vp::Capabilities,
        instance: vk::Instance,
        physicalDevice: vk::PhysicalDevice,
        pProfile: *const vp::ProfileProperties,
        pSupported: *mut vk::Bool32,
        pPropertyCount: *mut u32,
        pProperties: *mut vp::BlockProperties,
    ) -> vk::Result;

    pub(crate) fn vpCreateDevice(
        capabilities: vp::Capabilities,
        physicalDevice: vk::PhysicalDevice,
        pCreateInfo: *const vp::DeviceCreateInfo<'_>,
        pAllocator: *const vk::AllocationCallbacks<'_>,
        pDevice: *mut vk::Device,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileInstanceExtensionProperties(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
        pBlockName: *const ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::ExtensionProperties,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileDeviceExtensionProperties(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
        pBlockName: *const ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::ExtensionProperties,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileFeatures(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
        pBlockName: *const ffi::c_char,
        pNext: *mut ffi::c_void,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileFeatureStructureTypes(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
        pBlockName: *const ffi::c_char,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileProperties(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
        pBlockName: *const ffi::c_char,
        pNext: *mut ffi::c_void,
    ) -> vk::Result;

    pub(crate) fn vpGetProfilePropertyStructureTypes(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
        pBlockName: *const ffi::c_char,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileFormats(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
        pBlockName: *const ffi::c_char,
        pFormatCount: *mut u32,
        pFormats: *mut vk::Format,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileFormatProperties(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
        pBlockName: *const ffi::c_char,
        format: vk::Format,
        pNext: *mut ffi::c_void,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileFormatStructureTypes(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
        pBlockName: *const ffi::c_char,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileQueueFamilyProperties(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
        pBlockName: *const ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::QueueFamilyProperties2KHR<'_>,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileQueueFamilyStructureTypes(
        capabilities: vp::Capabilities,
        pProfile: *const vp::ProfileProperties,
        pBlockName: *const ffi::c_char,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;
}
