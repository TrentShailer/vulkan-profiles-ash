#![allow(dead_code)]

pub mod mock_vulkan_api;

use std::ffi::CStr;

use ash::vk;
use vp_ash::vp;

pub const SUPPORTED: &CStr = c"VP_VPA_test_supported";
pub const UNSUPPORTED_DEVICE: &CStr = c"VP_VPA_test_unsupported_device";
pub const UNSUPPORTED_INSTANCE: &CStr = c"VP_VPA_test_unsupported_instance";
pub const REQUIRES: &CStr = c"VP_VPA_test_requires";
pub const FALLBACK: &CStr = c"VP_VPA_test_fallback";
pub const FALLBACK_FALLBACK: &CStr = c"VP_VPA_test_fallback_fallback";
pub const VARIANTS_SUPPORTED: &CStr = c"VP_VPA_test_variants_supported";
pub const VARIANTS_DEVICE_UNSUPPORTED: &CStr = c"VP_VPA_test_variants_device_unsupported";
pub const VARIANTS_INSTANCE_UNSUPPORTED: &CStr = c"VP_VPA_test_variants_instance_unsupported";

pub fn all_expected_profiles_exist(
    expected: &[vp::ProfileProperties],
    real: &[vp::ProfileProperties],
) -> bool {
    expected.iter().all(|expected_profile| {
        real.iter().any(|profile| {
            profile.profile_name == expected_profile.profile_name
                && profile.spec_version == expected_profile.spec_version
        })
    })
}

pub fn blocks_match(block: vp::BlockProperties, expected: vp::BlockProperties) -> bool {
    block.block_name == expected.block_name
        // && block.api_version == expected.api_version
        && block.profiles.profile_name == expected.profiles.profile_name
        && block.profiles.spec_version == expected.profiles.spec_version
}

pub fn setup() -> (vp::VulkanFunctions, vp_ash::Entry, vp_ash::Capabilities) {
    let vulkan_functions = vp::VulkanFunctions {
        enumerate_instance_version: mock_vulkan_api::vkEnumerateInstanceVersion_MOCK,
        get_instance_proc_addr: mock_vulkan_api::vkGetInstanceProcAddr,
        get_device_proc_addr: mock_vulkan_api::vkGetDeviceProcAddr,
        enumerate_instance_extension_properties:
            mock_vulkan_api::vkEnumerateInstanceExtensionProperties_MOCK,
        enumerate_device_extension_properties:
            mock_vulkan_api::vkEnumerateDeviceExtensionProperties_MOCK,
        get_physical_device_features2: mock_vulkan_api::vkGetPhysicalDeviceFeatures2_MOCK,
        get_physical_device_properties2: mock_vulkan_api::vkGetPhysicalDeviceProperties2_MOCK,
        get_physical_device_format_properties2:
            mock_vulkan_api::vkGetPhysicalDeviceFormatProperties2_MOCK,
        get_physical_device_queue_family_properties2:
            mock_vulkan_api::vkGetPhysicalDeviceQueueFamilyProperties2_MOCK,
        create_instance: mock_vulkan_api::vkCreateInstance,
        create_device: mock_vulkan_api::vkCreateDevice,
    };

    let capabilities_create_info = vp::CapabilitiesCreateInfo::default()
        .flags(vp::CapabilitiesCreateFlags::STATIC)
        .vulkan_functions(&vulkan_functions)
        .api_version(vk::make_api_version(0, 1, 2, 0));
    let entry = vp_ash::Entry::linked();
    let capabilities = unsafe {
        entry
            .create_capabilities(&capabilities_create_info, None)
            .unwrap()
    };

    (vulkan_functions, entry, capabilities)
}

pub fn setup_instance(
    capabilities: &vp_ash::Capabilities,
    profile: vp::ProfileProperties,
) -> (ash::Entry, ash::Instance) {
    let profiles = [profile];

    let entry = ash::Entry::linked();

    let application_info = vk::ApplicationInfo::default();

    let vk_instance_create_info =
        vk::InstanceCreateInfo::default().application_info(&application_info);

    let vp_instance_create_info = vp::InstanceCreateInfo::default()
        .enabled_full_profiles(&profiles)
        .create_info(&vk_instance_create_info);

    let instance = unsafe {
        capabilities
            .create_instance(&entry, &vp_instance_create_info, None)
            .unwrap()
    };

    (entry, instance)
}
