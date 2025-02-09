use core::ffi;

use ash::vk;

use crate::vp::{
    BlockProperties, Capabilities, CapabilitiesCreateInfo, DeviceCreateInfo, InstanceCreateInfo,
    ProfileProperties,
};

#[allow(non_camel_case_types)]
pub type PFN_vpCreateCapabilities = unsafe extern "system" fn(
    pCreateInfo: *const CapabilitiesCreateInfo,
    pAllocator: *const vk::AllocationCallbacks<'_>,
    pCapabilities: *mut Capabilities,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpDestroyCapabilities = unsafe extern "system" fn(
    capabilities: Capabilities,
    pAllocator: *const vk::AllocationCallbacks<'_>,
) -> ffi::c_void;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfiles = unsafe extern "system" fn(
    capabilities: Capabilities,
    pPropertyCount: *mut u32,
    pProperties: *mut ProfileProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileRequiredProfiles = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
    pPropertyCount: *mut u32,
    pProperties: *mut ProfileProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileAPIVersion = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
) -> u32;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFallbacks = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
    pPropertyCount: *mut u32,
    pProperties: *mut ProfileProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpHasMultipleVariantsProfile = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
    pHasMultipleVariants: *mut vk::Bool32,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetInstanceProfileSupport = unsafe extern "system" fn(
    capabilities: Capabilities,
    pLayerName: *const ffi::c_char,
    pProfile: *const ProfileProperties,
    pSupported: *mut vk::Bool32,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetInstanceProfileVariantsSupport = unsafe extern "system" fn(
    capabilities: Capabilities,
    pLayerName: *const ffi::c_char,
    pProfile: *const ProfileProperties,
    pSupported: *mut vk::Bool32,
    pPropertyCount: *mut u32,
    pProperties: *mut BlockProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpCreateInstance = unsafe extern "system" fn(
    capabilities: Capabilities,
    pCreateInfo: *const InstanceCreateInfo<'_>,
    pAllocator: *const vk::AllocationCallbacks<'_>,
    pInstance: *mut vk::Instance,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetPhysicalDeviceProfileSupport = unsafe extern "system" fn(
    capabilities: Capabilities,
    instance: vk::Instance,
    physicalDevice: vk::PhysicalDevice,
    pProfile: *const ProfileProperties,
    pSupported: *mut vk::Bool32,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetPhysicalDeviceProfileVariantsSupport = unsafe extern "system" fn(
    capabilities: Capabilities,
    instance: vk::Instance,
    physicalDevice: vk::PhysicalDevice,
    pProfile: *const ProfileProperties,
    pSupported: *mut vk::Bool32,
    pPropertyCount: *mut u32,
    pProperties: *mut BlockProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpCreateDevice = unsafe extern "system" fn(
    capabilities: Capabilities,
    physicalDevice: vk::PhysicalDevice,
    pCreateInfo: *const DeviceCreateInfo<'_>,
    pAllocator: *const vk::AllocationCallbacks<'_>,
    pDevice: *mut vk::Device,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileInstanceExtensionProperties = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::ExtensionProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileDeviceExtensionProperties = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::ExtensionProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFeatures = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pNext: *mut ffi::c_void,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFeatureStructureTypes = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileProperties = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pNext: *mut ffi::c_void,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfilePropertyStructureTypes = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFormats = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pFormatCount: *mut u32,
    pFormats: *mut vk::Format,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFormatProperties = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    format: vk::Format,
    pNext: *mut ffi::c_void,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFormatStructureTypes = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileQueueFamilyProperties = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::QueueFamilyProperties2KHR<'_>,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileQueueFamilyStructureTypes = unsafe extern "system" fn(
    capabilities: Capabilities,
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

extern "system" {
    pub(crate) fn vpCreateCapabilities(
        pCreateInfo: *const CapabilitiesCreateInfo,
        pAllocator: *const vk::AllocationCallbacks<'_>,
        pCapabilities: *mut Capabilities,
    ) -> vk::Result;

    pub(crate) fn vpDestroyCapabilities(
        capabilities: Capabilities,
        pAllocator: *const vk::AllocationCallbacks<'_>,
    ) -> ffi::c_void;

    pub(crate) fn vpGetProfiles(
        capabilities: Capabilities,
        pPropertyCount: *mut u32,
        pProperties: *mut ProfileProperties,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileRequiredProfiles(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
        pPropertyCount: *mut u32,
        pProperties: *mut ProfileProperties,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileAPIVersion(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
    ) -> u32;

    pub(crate) fn vpGetProfileFallbacks(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
        pPropertyCount: *mut u32,
        pProperties: *mut ProfileProperties,
    ) -> vk::Result;

    pub(crate) fn vpHasMultipleVariantsProfile(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
        pHasMultipleVariants: *mut vk::Bool32,
    ) -> vk::Result;

    pub(crate) fn vpGetInstanceProfileSupport(
        capabilities: Capabilities,
        pLayerName: *const ffi::c_char,
        pProfile: *const ProfileProperties,
        pSupported: *mut vk::Bool32,
    ) -> vk::Result;

    pub(crate) fn vpGetInstanceProfileVariantsSupport(
        capabilities: Capabilities,
        pLayerName: *const ffi::c_char,
        pProfile: *const ProfileProperties,
        pSupported: *mut vk::Bool32,
        pPropertyCount: *mut u32,
        pProperties: *mut BlockProperties,
    ) -> vk::Result;

    pub(crate) fn vpCreateInstance(
        capabilities: Capabilities,
        pCreateInfo: *const InstanceCreateInfo<'_>,
        pAllocator: *const vk::AllocationCallbacks<'_>,
        pInstance: *mut vk::Instance,
    ) -> vk::Result;

    pub(crate) fn vpGetPhysicalDeviceProfileSupport(
        capabilities: Capabilities,
        instance: vk::Instance,
        physicalDevice: vk::PhysicalDevice,
        pProfile: *const ProfileProperties,
        pSupported: *mut vk::Bool32,
    ) -> vk::Result;

    pub(crate) fn vpGetPhysicalDeviceProfileVariantsSupport(
        capabilities: Capabilities,
        instance: vk::Instance,
        physicalDevice: vk::PhysicalDevice,
        pProfile: *const ProfileProperties,
        pSupported: *mut vk::Bool32,
        pPropertyCount: *mut u32,
        pProperties: *mut BlockProperties,
    ) -> vk::Result;

    pub(crate) fn vpCreateDevice(
        capabilities: Capabilities,
        physicalDevice: vk::PhysicalDevice,
        pCreateInfo: *const DeviceCreateInfo<'_>,
        pAllocator: *const vk::AllocationCallbacks<'_>,
        pDevice: *mut vk::Device,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileInstanceExtensionProperties(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::ExtensionProperties,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileDeviceExtensionProperties(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::ExtensionProperties,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileFeatures(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pNext: *mut ffi::c_void,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileFeatureStructureTypes(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileProperties(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pNext: *mut ffi::c_void,
    ) -> vk::Result;

    pub(crate) fn vpGetProfilePropertyStructureTypes(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileFormats(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pFormatCount: *mut u32,
        pFormats: *mut vk::Format,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileFormatProperties(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        format: vk::Format,
        pNext: *mut ffi::c_void,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileFormatStructureTypes(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileQueueFamilyProperties(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::QueueFamilyProperties2KHR<'_>,
    ) -> vk::Result;

    pub(crate) fn vpGetProfileQueueFamilyStructureTypes(
        capabilities: Capabilities,
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;
}
