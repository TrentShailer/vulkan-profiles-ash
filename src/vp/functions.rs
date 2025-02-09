#![allow(non_camel_case_types)]

use core::ffi;

use ash::vk;

use crate::vp;

pub type PFN_vpCreateCapabilities = unsafe extern "system" fn(
    pCreateInfo: *const vp::CapabilitiesCreateInfo<'_>,
    pAllocator: *const vk::AllocationCallbacks<'_>,
    pCapabilities: *mut vp::Capabilities,
) -> vk::Result;

pub type PFN_vpDestroyCapabilities = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pAllocator: *const vk::AllocationCallbacks<'_>,
) -> ffi::c_void;

pub type PFN_vpGetProfiles = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pPropertyCount: *mut u32,
    pProperties: *mut vp::ProfileProperties,
) -> vk::Result;

pub type PFN_vpGetProfileRequiredProfiles = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pPropertyCount: *mut u32,
    pProperties: *mut vp::ProfileProperties,
) -> vk::Result;

pub type PFN_vpGetProfileAPIVersion = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
) -> u32;

pub type PFN_vpGetProfileFallbacks = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pPropertyCount: *mut u32,
    pProperties: *mut vp::ProfileProperties,
) -> vk::Result;

pub type PFN_vpHasMultipleVariantsProfile = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pHasMultipleVariants: *mut vk::Bool32,
) -> vk::Result;

pub type PFN_vpGetInstanceProfileSupport = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pLayerName: *const ffi::c_char,
    pProfile: *const vp::ProfileProperties,
    pSupported: *mut vk::Bool32,
) -> vk::Result;

pub type PFN_vpGetInstanceProfileVariantsSupport = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pLayerName: *const ffi::c_char,
    pProfile: *const vp::ProfileProperties,
    pSupported: *mut vk::Bool32,
    pPropertyCount: *mut u32,
    pProperties: *mut vp::BlockProperties,
) -> vk::Result;

pub type PFN_vpCreateInstance = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pCreateInfo: *const vp::InstanceCreateInfo<'_>,
    pAllocator: *const vk::AllocationCallbacks<'_>,
    pInstance: *mut vk::Instance,
) -> vk::Result;

pub type PFN_vpGetPhysicalDeviceProfileSupport = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    instance: vk::Instance,
    physicalDevice: vk::PhysicalDevice,
    pProfile: *const vp::ProfileProperties,
    pSupported: *mut vk::Bool32,
) -> vk::Result;

pub type PFN_vpGetPhysicalDeviceProfileVariantsSupport = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    instance: vk::Instance,
    physicalDevice: vk::PhysicalDevice,
    pProfile: *const vp::ProfileProperties,
    pSupported: *mut vk::Bool32,
    pPropertyCount: *mut u32,
    pProperties: *mut vp::BlockProperties,
) -> vk::Result;

pub type PFN_vpCreateDevice = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    physicalDevice: vk::PhysicalDevice,
    pCreateInfo: *const vp::DeviceCreateInfo<'_>,
    pAllocator: *const vk::AllocationCallbacks<'_>,
    pDevice: *mut vk::Device,
) -> vk::Result;

pub type PFN_vpGetProfileInstanceExtensionProperties = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::ExtensionProperties,
) -> vk::Result;

pub type PFN_vpGetProfileDeviceExtensionProperties = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::ExtensionProperties,
) -> vk::Result;

pub type PFN_vpGetProfileFeatures = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    pNext: *mut ffi::c_void,
) -> vk::Result;

pub type PFN_vpGetProfileFeatureStructureTypes = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

pub type PFN_vpGetProfileProperties = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    pNext: *mut ffi::c_void,
) -> vk::Result;

pub type PFN_vpGetProfilePropertyStructureTypes = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

pub type PFN_vpGetProfileFormats = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    pFormatCount: *mut u32,
    pFormats: *mut vk::Format,
) -> vk::Result;

pub type PFN_vpGetProfileFormatProperties = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    format: vk::Format,
    pNext: *mut ffi::c_void,
) -> vk::Result;

pub type PFN_vpGetProfileFormatStructureTypes = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

pub type PFN_vpGetProfileQueueFamilyProperties = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::QueueFamilyProperties2KHR<'_>,
) -> vk::Result;

pub type PFN_vpGetProfileQueueFamilyStructureTypes = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;
