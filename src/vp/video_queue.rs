use alloc::vec::Vec;
use core::ffi::{self, CStr};

use ash::{prelude::VkResult, vk};

use crate::{utils::read_into_uninitialized_vector, vp};

pub struct Capabilities {
    handle: vp::Capabilities,
    fp: CapabilitiesFn,
}

impl Capabilities {
    pub fn linked(handle: vp::Capabilities) -> Self {
        Self {
            handle,
            fp: CapabilitiesFn::linked(),
        }
    }

    pub fn handle(&self) -> vp::Capabilities {
        self.handle
    }

    /// Returns the raw function pointer table
    pub fn fp(&self) -> &CapabilitiesFn {
        &self.fp
    }

    /// Query the list of video profiles specified by the profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-video-profiles>
    pub unsafe fn get_profile_video_profiles(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vp::VideoProfileProperties>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_profile_video_profiles)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }

    /// Query the video profile info structures for a video profile defined by a profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-video-profiles>
    pub unsafe fn get_profile_video_profile_info(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
        video_profile_index: u32,
        video_profile_info: &mut vk::VideoProfileInfoKHR<'_>,
    ) -> VkResult<()> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        (self.fp.get_profile_video_profile_info)(
            self.handle,
            profile_properties,
            block_name_ptr,
            video_profile_index,
            video_profile_info,
        )
        .result()
    }

    /// Query the list of video profiles specified by the profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-video-profiles>
    pub unsafe fn get_profile_video_profile_info_structure_types(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
        video_profile_index: u32,
    ) -> VkResult<Vec<vk::StructureType>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_profile_video_profile_info_structure_types)(
                self.handle,
                profile_properties,
                block_name_ptr,
                video_profile_index,
                count,
                data,
            )
        })
    }

    /// Query the video capabilities requirements for a video profile defined by a profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-format-properties>
    pub unsafe fn get_profile_video_capabilities(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
        video_profile_index: u32,
        capabilities: &mut vk::VideoCapabilitiesKHR<'_>,
    ) -> VkResult<()> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        (self.fp.get_profile_video_capabilities)(
            self.handle,
            profile_properties,
            block_name_ptr,
            video_profile_index,
            <*mut _>::cast(capabilities),
        )
        .result()
    }

    /// Query the list of video capability structure types specified by the profile for a video
    /// profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-video-profiles>
    pub unsafe fn get_profile_video_capability_structure_types(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
        video_profile_index: u32,
    ) -> VkResult<Vec<vk::StructureType>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_profile_video_capability_structure_types)(
                self.handle,
                profile_properties,
                block_name_ptr,
                video_profile_index,
                count,
                data,
            )
        })
    }

    /// Query the video format property requirements for a video profile defined by a profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-video-profiles>
    pub unsafe fn get_profile_video_format_properties(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
        video_profile_index: u32,
    ) -> VkResult<Vec<vk::VideoFormatPropertiesKHR<'_>>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_profile_video_format_properties)(
                self.handle,
                profile_properties,
                block_name_ptr,
                video_profile_index,
                count,
                data,
            )
        })
    }

    /// Query the list of video format property structure types specified by the profile for a video
    /// profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-video-profiles>
    pub unsafe fn get_profile_video_format_structure_types(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
        video_profile_index: u32,
    ) -> VkResult<Vec<vk::StructureType>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_profile_video_format_structure_types)(
                self.handle,
                profile_properties,
                block_name_ptr,
                video_profile_index,
                count,
                data,
            )
        })
    }
}

pub struct CapabilitiesFn {
    pub get_profile_video_profiles: PFN_vpGetProfileVideoProfiles,
    pub get_profile_video_profile_info: PFN_vpGetProfileVideoProfileInfo,
    pub get_profile_video_profile_info_structure_types:
        PFN_vpGetProfileVideoProfileInfoStructureTypes,
    pub get_profile_video_capabilities: PFN_vpGetProfileVideoCapabilities,
    pub get_profile_video_capability_structure_types: PFN_vpGetProfileVideoCapabilityStructureTypes,
    pub get_profile_video_format_properties: PFN_vpGetProfileVideoFormatProperties,
    pub get_profile_video_format_structure_types: PFN_vpGetProfileVideoFormatStructureTypes,
}

impl CapabilitiesFn {
    pub fn linked() -> Self {
        Self {
            get_profile_video_profiles: linked::vpGetProfileVideoProfiles,
            get_profile_video_profile_info: linked::vpGetProfileVideoProfileInfo,
            get_profile_video_profile_info_structure_types:
                linked::vpGetProfileVideoProfileInfoStructureTypes,
            get_profile_video_capabilities: linked::vpGetProfileVideoCapabilities,
            get_profile_video_capability_structure_types:
                linked::vpGetProfileVideoCapabilityStructureTypes,
            get_profile_video_format_properties: linked::vpGetProfileVideoFormatProperties,
            get_profile_video_format_structure_types: linked::vpGetProfileVideoFormatStructureTypes,
        }
    }
}

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileVideoProfiles = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    pVideoProfileCount: *mut u32,
    pVideoProfiles: *mut vp::VideoProfileProperties,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileVideoProfileInfo = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    videoProfileIndex: u32,
    pVideoProfileInfo: *mut vk::VideoProfileInfoKHR<'_>,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileVideoProfileInfoStructureTypes = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    videoProfileIndex: u32,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileVideoCapabilities = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    videoProfileIndex: u32,
    pNext: *mut ffi::c_void,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileVideoCapabilityStructureTypes = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    videoProfileIndex: u32,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileVideoFormatProperties = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    videoProfileIndex: u32,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VideoFormatPropertiesKHR<'_>,
) -> vk::Result;

#[allow(non_camel_case_types)]
pub type PFN_vpGetProfileVideoFormatStructureTypes = unsafe extern "system" fn(
    capabilities: vp::Capabilities,
    pProfile: *const vp::ProfileProperties,
    pBlockName: *const ffi::c_char,
    videoProfileIndex: u32,
    pStructureTypeCount: *mut u32,
    pStructureTypes: *mut vk::StructureType,
) -> vk::Result;

mod linked {
    use core::ffi;

    use ash::vk;

    use crate::vp;

    extern "system" {
        pub fn vpGetProfileVideoProfiles(
            capabilities: vp::Capabilities,
            pProfile: *const vp::ProfileProperties,
            pBlockName: *const ffi::c_char,
            pVideoProfileCount: *mut u32,
            pVideoProfiles: *mut vp::VideoProfileProperties,
        ) -> vk::Result;

        pub fn vpGetProfileVideoProfileInfo(
            capabilities: vp::Capabilities,
            pProfile: *const vp::ProfileProperties,
            pBlockName: *const ffi::c_char,
            videoProfileIndex: u32,
            pVideoProfileInfo: *mut vk::VideoProfileInfoKHR<'_>,
        ) -> vk::Result;

        pub fn vpGetProfileVideoProfileInfoStructureTypes(
            capabilities: vp::Capabilities,
            pProfile: *const vp::ProfileProperties,
            pBlockName: *const ffi::c_char,
            videoProfileIndex: u32,
            pStructureTypeCount: *mut u32,
            pStructureTypes: *mut vk::StructureType,
        ) -> vk::Result;

        pub fn vpGetProfileVideoCapabilities(
            capabilities: vp::Capabilities,
            pProfile: *const vp::ProfileProperties,
            pBlockName: *const ffi::c_char,
            videoProfileIndex: u32,
            pNext: *mut ffi::c_void,
        ) -> vk::Result;

        pub fn vpGetProfileVideoCapabilityStructureTypes(
            capabilities: vp::Capabilities,
            pProfile: *const vp::ProfileProperties,
            pBlockName: *const ffi::c_char,
            videoProfileIndex: u32,
            pStructureTypeCount: *mut u32,
            pStructureTypes: *mut vk::StructureType,
        ) -> vk::Result;

        pub fn vpGetProfileVideoFormatProperties(
            capabilities: vp::Capabilities,
            pProfile: *const vp::ProfileProperties,
            pBlockName: *const ffi::c_char,
            videoProfileIndex: u32,
            pPropertyCount: *mut u32,
            pProperties: *mut vk::VideoFormatPropertiesKHR<'_>,
        ) -> vk::Result;

        pub fn vpGetProfileVideoFormatStructureTypes(
            capabilities: vp::Capabilities,
            pProfile: *const vp::ProfileProperties,
            pBlockName: *const ffi::c_char,
            videoProfileIndex: u32,
            pStructureTypeCount: *mut u32,
            pStructureTypes: *mut vk::StructureType,
        ) -> vk::Result;
    }
}
