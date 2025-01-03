use core::ffi;

use ash::vk;

use super::{BlockProperties, DeviceCreateInfo, InstanceCreateInfo, ProfileProperties};

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfiles = unsafe extern "system" fn(
    pPropertyCount: *mut u32,
    pProperties: *mut ProfileProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileRequiredProfiles = unsafe extern "system" fn(
    pProfile: *const ProfileProperties,
    pPropertyCount: *mut u32,
    pProperties: *mut ProfileProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileAPIVersion =
    unsafe extern "system" fn(pProfile: *const ProfileProperties) -> u32;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFallbacks = unsafe extern "system" fn(
    pProfile: *const ProfileProperties,
    pPropertyCount: *mut u32,
    pProperties: *mut ProfileProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpHasMultipleVariantsProfile = unsafe extern "system" fn(
    pProfile: *const ProfileProperties,
    pHasMultipleVariants: *mut vk::Bool32,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetInstanceProfileSupport = unsafe extern "system" fn(
    pLayerName: *const ffi::c_char,
    pProfile: *const ProfileProperties,
    pSupported: *mut vk::Bool32,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetInstanceProfileVariantsSupport = unsafe extern "system" fn(
    pLayerName: *const ffi::c_char,
    pProfile: *const ProfileProperties,
    pSupported: *mut vk::Bool32,
    pPropertyCount: *mut u32,
    pProperties: *mut BlockProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpCreateInstance = unsafe extern "system" fn(
    pCreateInfo: *const InstanceCreateInfo<'_>,
    pAllocator: *const vk::AllocationCallbacks<'_>,
    pInstance: *mut vk::Instance,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetPhysicalDeviceProfileSupport = unsafe extern "system" fn(
    instance: vk::Instance,
    physicalDevice: vk::PhysicalDevice,
    pProfile: *const ProfileProperties,
    pSupported: *mut vk::Bool32,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetPhysicalDeviceProfileVariantsSupport = unsafe extern "system" fn(
    instance: vk::Instance,
    physicalDevice: vk::PhysicalDevice,
    pProfile: *const ProfileProperties,
    pSupported: *mut vk::Bool32,
    pPropertyCount: *mut u32,
    pProperties: *mut BlockProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpCreateDevice = unsafe extern "system" fn(
    physicalDevice: vk::PhysicalDevice,
    pCreateInfo: *const DeviceCreateInfo<'_>,
    pAllocator: *const vk::AllocationCallbacks<'_>,
    pDevice: *mut vk::Device,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileInstanceExtensionProperties = unsafe extern "system" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::ExtensionProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileDeviceExtensionProperties = unsafe extern "system" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::ExtensionProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFeatures = unsafe extern "system" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pNext: *mut ffi::c_void,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFeatureStructureTypes = unsafe extern "system" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileProperties = unsafe extern "system" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pNext: *mut ffi::c_void,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfilePropertyStructureTypes = unsafe extern "system" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFormats = unsafe extern "system" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pFormatCount: *mut u32,
    pFormats: *mut vk::Format,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFormatProperties = unsafe extern "system" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    format: vk::Format,
    pNext: *mut ffi::c_void,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileFormatStructureTypes = unsafe extern "system" fn(
    pProfile: *const ProfileProperties,
    pBlockName: *const ffi::c_char,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

pub struct ProfilesFn {
    pub get_profiles: PFN_vpGetProfiles,
    pub get_profile_required_profiles: PFN_vpGetProfileRequiredProfiles,
    pub get_profile_api_version: PFN_vpGetProfileAPIVersion,
    pub get_profile_fallbacks: PFN_vpGetProfileFallbacks,
    pub has_multiple_variants_profile: PFN_vpHasMultipleVariantsProfile,
    pub get_instance_profile_support: PFN_vpGetInstanceProfileSupport,
    pub get_instance_profile_variants_support: PFN_vpGetInstanceProfileVariantsSupport,
    pub create_instance: PFN_vpCreateInstance,
    pub get_physical_device_profile_support: PFN_vpGetPhysicalDeviceProfileSupport,
    pub get_physical_device_profile_variants_support: PFN_vpGetPhysicalDeviceProfileVariantsSupport,
    pub create_device: PFN_vpCreateDevice,
    pub get_profile_instance_extension_properties: PFN_vpGetProfileInstanceExtensionProperties,
    pub get_profile_device_extension_properties: PFN_vpGetProfileDeviceExtensionProperties,
    pub get_profile_features: PFN_vpGetProfileFeatures,
    pub get_profile_feature_structure_types: PFN_vpGetProfileFeatureStructureTypes,
    pub get_profile_properties: PFN_vpGetProfileProperties,
    pub get_profile_property_structure_types: PFN_vpGetProfilePropertyStructureTypes,
    pub get_profile_formats: PFN_vpGetProfileFormats,
    pub get_profile_format_properties: PFN_vpGetProfileFormatProperties,
    pub get_profile_format_structure_types: PFN_vpGetProfileFormatStructureTypes,
}

impl ProfilesFn {
    /// Initializes the table from a statically linked library
    #[cfg(feature = "linked")]
    pub fn load_static() -> Self {
        Self {
            get_profiles: vpGetProfiles,
            get_profile_required_profiles: vpGetProfileRequiredProfiles,
            get_profile_api_version: vpGetProfileAPIVersion,
            get_profile_fallbacks: vpGetProfileFallbacks,
            has_multiple_variants_profile: vpHasMultipleVariantsProfile,
            get_instance_profile_support: vpGetInstanceProfileSupport,
            get_instance_profile_variants_support: vpGetInstanceProfileVariantsSupport,
            create_instance: vpCreateInstance,
            get_physical_device_profile_support: vpGetPhysicalDeviceProfileSupport,
            get_physical_device_profile_variants_support: vpGetPhysicalDeviceProfileVariantsSupport,
            create_device: vpCreateDevice,
            get_profile_instance_extension_properties: vpGetProfileInstanceExtensionProperties,
            get_profile_device_extension_properties: vpGetProfileDeviceExtensionProperties,
            get_profile_features: vpGetProfileFeatures,
            get_profile_feature_structure_types: vpGetProfileFeatureStructureTypes,
            get_profile_properties: vpGetProfileProperties,
            get_profile_property_structure_types: vpGetProfilePropertyStructureTypes,
            get_profile_formats: vpGetProfileFormats,
            get_profile_format_properties: vpGetProfileFormatProperties,
            get_profile_format_structure_types: vpGetProfileFormatStructureTypes,
        }
    }
}

#[cfg(feature = "linked")]
extern "system" {
    fn vpGetProfiles(pPropertyCount: *mut u32, pProperties: *mut ProfileProperties) -> vk::Result;

    fn vpGetProfileRequiredProfiles(
        pProfile: *const ProfileProperties,
        pPropertyCount: *mut u32,
        pProperties: *mut ProfileProperties,
    ) -> vk::Result;

    fn vpGetProfileAPIVersion(pProfile: *const ProfileProperties) -> u32;

    fn vpGetProfileFallbacks(
        pProfile: *const ProfileProperties,
        pPropertyCount: *mut u32,
        pProperties: *mut ProfileProperties,
    ) -> vk::Result;

    fn vpHasMultipleVariantsProfile(
        pProfile: *const ProfileProperties,
        pHasMultipleVariants: *mut vk::Bool32,
    ) -> vk::Result;

    fn vpGetInstanceProfileSupport(
        pLayerName: *const ffi::c_char,
        pProfile: *const ProfileProperties,
        pSupported: *mut vk::Bool32,
    ) -> vk::Result;

    fn vpGetInstanceProfileVariantsSupport(
        pLayerName: *const ffi::c_char,
        pProfile: *const ProfileProperties,
        pSupported: *mut vk::Bool32,
        pPropertyCount: *mut u32,
        pProperties: *mut BlockProperties,
    ) -> vk::Result;

    fn vpCreateInstance(
        pCreateInfo: *const InstanceCreateInfo<'_>,
        pAllocator: *const vk::AllocationCallbacks<'_>,
        pInstance: *mut vk::Instance,
    ) -> vk::Result;

    fn vpGetPhysicalDeviceProfileSupport(
        instance: vk::Instance,
        physicalDevice: vk::PhysicalDevice,
        pProfile: *const ProfileProperties,
        pSupported: *mut vk::Bool32,
    ) -> vk::Result;

    fn vpGetPhysicalDeviceProfileVariantsSupport(
        instance: vk::Instance,
        physicalDevice: vk::PhysicalDevice,
        pProfile: *const ProfileProperties,
        pSupported: *mut vk::Bool32,
        pPropertyCount: *mut u32,
        pProperties: *mut BlockProperties,
    ) -> vk::Result;

    fn vpCreateDevice(
        physicalDevice: vk::PhysicalDevice,
        pCreateInfo: *const DeviceCreateInfo<'_>,
        pAllocator: *const vk::AllocationCallbacks<'_>,
        pDevice: *mut vk::Device,
    ) -> vk::Result;

    fn vpGetProfileInstanceExtensionProperties(
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::ExtensionProperties,
    ) -> vk::Result;

    fn vpGetProfileDeviceExtensionProperties(
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::ExtensionProperties,
    ) -> vk::Result;

    fn vpGetProfileFeatures(
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pNext: *mut ffi::c_void,
    ) -> vk::Result;

    fn vpGetProfileFeatureStructureTypes(
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;

    fn vpGetProfileProperties(
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pNext: *mut ffi::c_void,
    ) -> vk::Result;

    fn vpGetProfilePropertyStructureTypes(
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;

    fn vpGetProfileFormats(
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pFormatCount: *mut u32,
        pFormats: *mut vk::Format,
    ) -> vk::Result;

    fn vpGetProfileFormatProperties(
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        format: vk::Format,
        pNext: *mut ffi::c_void,
    ) -> vk::Result;

    fn vpGetProfileFormatStructureTypes(
        pProfile: *const ProfileProperties,
        pBlockName: *const ffi::c_char,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;
}
