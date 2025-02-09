use core::{
    ffi::{self, CStr, FromBytesUntilNulError},
    fmt,
    marker::PhantomData,
};

use ash::{
    define_handle,
    vk::{self, CStrTooLargeForStaticArray, Handle, ObjectType},
};

use crate::utils::{wrap_c_str_slice_until_nul, write_c_str_slice_with_nul};

pub const VP_MAX_PROFILE_NAME_SIZE: usize = 256;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
/// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#checking-instance-level-support>
pub struct ProfileProperties {
    pub profile_name: [ffi::c_char; VP_MAX_PROFILE_NAME_SIZE],
    pub spec_version: u32,
}

impl Default for ProfileProperties {
    fn default() -> Self {
        Self {
            profile_name: unsafe { core::mem::zeroed() },
            spec_version: 1,
        }
    }
}

impl ProfileProperties {
    pub fn profile_name(mut self, name: &CStr) -> Result<Self, CStrTooLargeForStaticArray> {
        write_c_str_slice_with_nul(self.profile_name.as_mut_slice(), name)?;
        Ok(self)
    }

    pub fn spec_version(mut self, version: u32) -> Self {
        self.spec_version = version;
        self
    }

    pub fn profile_name_as_c_str(&self) -> Result<&CStr, FromBytesUntilNulError> {
        wrap_c_str_slice_until_nul(&self.profile_name)
    }
}

#[cfg(feature = "debug")]
impl fmt::Debug for ProfileProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProfileProperties")
            .field("profile_name", &unsafe {
                CStr::from_ptr(self.profile_name.as_ptr())
            })
            .field("spec_version", &self.spec_version)
            .finish()
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
/// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#checking-instance-level-support>
pub struct BlockProperties {
    pub profiles: ProfileProperties,
    pub api_version: u32,
    pub block_name: [ffi::c_char; VP_MAX_PROFILE_NAME_SIZE],
}

impl Default for BlockProperties {
    fn default() -> Self {
        Self {
            profiles: Default::default(),
            api_version: Default::default(),
            block_name: unsafe { core::mem::zeroed() },
        }
    }
}

impl BlockProperties {
    pub fn profiles(mut self, profiles: ProfileProperties) -> Self {
        self.profiles = profiles;
        self
    }

    pub fn api_version(mut self, version: u32) -> Self {
        self.api_version = version;
        self
    }

    pub fn block_name(mut self, name: &CStr) -> Result<Self, CStrTooLargeForStaticArray> {
        write_c_str_slice_with_nul(self.block_name.as_mut_slice(), name)?;
        Ok(self)
    }

    pub fn block_name_as_c_str(&self) -> Result<&CStr, FromBytesUntilNulError> {
        wrap_c_str_slice_until_nul(&self.block_name)
    }
}

#[cfg(feature = "debug")]
impl fmt::Debug for BlockProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlockProperties")
            .field("profiles", &self.profiles)
            .field("api_version", &self.api_version)
            .field("block_name", &unsafe {
                CStr::from_ptr(self.block_name.as_ptr())
            })
            .finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#creating-instance-with-profile>
pub struct InstanceCreateFlags(pub(crate) vk::Flags);
ash::vk_bitflags_wrapped!(InstanceCreateFlags, vk::Flags);
impl InstanceCreateFlags {}

#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy)]
/// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#creating-instance-with-profile>
pub struct InstanceCreateInfo<'a> {
    pub p_create_info: *const vk::InstanceCreateInfo<'a>,
    pub flags: InstanceCreateFlags,
    pub enabled_full_profile_count: u32,
    pub p_enabled_full_profiles: *const ProfileProperties,
    pub enabled_profile_block_count: u32,
    pub p_enabled_profile_blocks: *const BlockProperties,
}

impl Default for InstanceCreateInfo<'_> {
    fn default() -> Self {
        Self {
            p_create_info: core::ptr::null(),
            flags: Default::default(),
            enabled_full_profile_count: Default::default(),
            p_enabled_full_profiles: core::ptr::null(),
            enabled_profile_block_count: Default::default(),
            p_enabled_profile_blocks: core::ptr::null(),
        }
    }
}

impl<'a> InstanceCreateInfo<'a> {
    pub fn create_info(mut self, create_info: &'a vk::InstanceCreateInfo<'_>) -> Self {
        self.p_create_info = create_info;
        self
    }

    pub fn enabled_full_profiles(mut self, enabled_full_profiles: &'a [ProfileProperties]) -> Self {
        self.enabled_full_profile_count = enabled_full_profiles.len() as _;
        self.p_enabled_full_profiles = enabled_full_profiles.as_ptr();
        self
    }

    pub fn enabled_profile_blocks(mut self, enabled_profile_blocks: &'a [BlockProperties]) -> Self {
        self.enabled_profile_block_count = enabled_profile_blocks.len() as _;
        self.p_enabled_profile_blocks = enabled_profile_blocks.as_ptr();
        self
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#creating-device-with-profile>
pub struct DeviceCreateFlags(pub(crate) vk::Flags);
ash::vk_bitflags_wrapped!(DeviceCreateFlags, vk::Flags);

impl DeviceCreateFlags {
    pub const DISABLE_ROBUST_BUFFER_ACCESS: Self = Self(0x0000001);
    pub const DISABLE_ROBUST_IMAGE_ACCESS: Self = Self(0x0000002);
    pub const DISABLE_ROBUST_ACCESS: Self = Self(0x0000001 | 0x0000002);
}

#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy)]
/// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#creating-device-with-profile>
pub struct DeviceCreateInfo<'a> {
    pub p_create_info: *const vk::DeviceCreateInfo<'a>,
    pub flags: DeviceCreateFlags,
    pub enabled_full_profile_count: u32,
    pub p_enabled_full_profiles: *const ProfileProperties,
    pub enabled_profile_block_count: u32,
    pub p_enabled_profile_blocks: *const BlockProperties,
}

impl Default for DeviceCreateInfo<'_> {
    fn default() -> Self {
        Self {
            p_create_info: core::ptr::null(),
            flags: Default::default(),
            enabled_full_profile_count: Default::default(),
            p_enabled_full_profiles: core::ptr::null(),
            enabled_profile_block_count: Default::default(),
            p_enabled_profile_blocks: core::ptr::null(),
        }
    }
}

impl<'a> DeviceCreateInfo<'a> {
    pub fn create_info(mut self, create_info: &'a vk::DeviceCreateInfo<'_>) -> Self {
        self.p_create_info = create_info;
        self
    }

    pub fn flags(mut self, flags: DeviceCreateFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn enabled_full_profiles(mut self, enabled_full_profiles: &'a [ProfileProperties]) -> Self {
        self.enabled_full_profile_count = enabled_full_profiles.len() as _;
        self.p_enabled_full_profiles = enabled_full_profiles.as_ptr();
        self
    }

    pub fn enabled_profile_blocks(mut self, enabled_profile_blocks: &'a [BlockProperties]) -> Self {
        self.enabled_profile_block_count = enabled_profile_blocks.len() as _;
        self.p_enabled_profile_blocks = enabled_profile_blocks.as_ptr();
        self
    }
}

define_handle!(Capabilities, DEVICE);

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilitiesCreateFlags(pub(crate) vk::Flags);
ash::vk_bitflags_wrapped!(CapabilitiesCreateFlags, vk::Flags);

impl CapabilitiesCreateFlags {
    pub const STATIC: Self = Self(0x0000001);
    // pub const DYNAMIC: Self = Self(0x0000002);
}

#[repr(C)]
pub struct VulkanFunctions {
    pub get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    pub get_device_proc_addr: vk::PFN_vkGetDeviceProcAddr,
    pub enumerate_instance_version: vk::PFN_vkEnumerateInstanceVersion,
    pub enumerate_instance_extension_properties: vk::PFN_vkEnumerateInstanceExtensionProperties,
    pub enumerate_device_extension_properties: vk::PFN_vkEnumerateDeviceExtensionProperties,
    pub get_physical_device_features2: vk::PFN_vkGetPhysicalDeviceFeatures2,
    pub get_physical_device_properties2: vk::PFN_vkGetPhysicalDeviceProperties2,
    pub get_physical_device_format_properties2: vk::PFN_vkGetPhysicalDeviceFormatProperties2,
    pub get_physical_device_queue_family_properties2:
        vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
    pub create_instance: vk::PFN_vkCreateInstance,
    pub create_device: vk::PFN_vkCreateDevice,
}

#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy)]
pub struct CapabilitiesCreateInfo<'a> {
    pub flags: CapabilitiesCreateFlags,
    pub api_version: u32,
    pub vulkan_functions: *const VulkanFunctions,
    pub _marker: PhantomData<&'a ()>,
}

impl Default for CapabilitiesCreateInfo<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            api_version: Default::default(),
            vulkan_functions: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}

impl<'a> CapabilitiesCreateInfo<'a> {
    pub fn flags(mut self, flags: CapabilitiesCreateFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn api_version(mut self, version: u32) -> Self {
        self.api_version = version;
        self
    }

    pub fn vulkan_functions(mut self, functions: &'a VulkanFunctions) -> Self {
        self.vulkan_functions = functions;
        self
    }
}
