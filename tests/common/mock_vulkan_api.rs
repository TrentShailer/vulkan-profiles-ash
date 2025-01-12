use core::ffi;

use ash::vk;

#[link(name = "mock_vulkan_api", kind = "static")]
extern "system" {
    pub fn vkEnumerateInstanceVersion_MOCK(pApiVersion: *mut u32) -> vk::Result;

    pub fn vkEnumerateInstanceExtensionProperties_MOCK(
        pLayerName: *const ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::ExtensionProperties,
    ) -> vk::Result;

    pub fn vkEnumerateDeviceExtensionProperties_MOCK(
        physicalDevice: vk::PhysicalDevice,
        pLayerName: *const ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::ExtensionProperties,
    ) -> vk::Result;

    pub fn vkGetPhysicalDeviceFeatures2_MOCK(
        physicalDevice: vk::PhysicalDevice,
        pFeatures: *mut vk::PhysicalDeviceFeatures2<'_>,
    );

    pub fn vkGetPhysicalDeviceProperties2_MOCK(
        physicalDevice: vk::PhysicalDevice,
        pProperties: *mut vk::PhysicalDeviceProperties2,
    );

    pub fn vkGetPhysicalDeviceFormatProperties2_MOCK(
        physicalDevice: vk::PhysicalDevice,
        format: vk::Format,
        pFormatProperties: *mut vk::FormatProperties2,
    );

    pub fn vkGetPhysicalDeviceQueueFamilyProperties2_MOCK(
        physicalDevice: vk::PhysicalDevice,
        pQueueFamilyPropertyCount: *mut u32,
        pQueueFamilyProperties: *mut vk::QueueFamilyProperties2,
    );

}

extern "system" {
    pub fn vkGetInstanceProcAddr(
        instance: vk::Instance,
        name: *const ffi::c_char,
    ) -> vk::PFN_vkVoidFunction;

    pub fn vkGetDeviceProcAddr(
        device: vk::Device,
        name: *const ffi::c_char,
    ) -> vk::PFN_vkVoidFunction;

    pub fn vkCreateInstance(
        pCreateInfo: *const vk::InstanceCreateInfo,
        pAllocator: *const vk::AllocationCallbacks,
        pInstance: *mut vk::Instance,
    ) -> vk::Result;

    pub fn vkCreateDevice(
        physicalDevice: vk::PhysicalDevice,
        pCreateInfo: *const vk::DeviceCreateInfo,
        pAllocator: *const vk::AllocationCallbacks,
        pDevice: *mut vk::Device,
    ) -> vk::Result;
}
