use core::ffi::CStr;

use alloc::vec::Vec;

use ash::{prelude::VkResult, vk, RawPtr};

use crate::{
    utils::{read_into_uninitialized_vector, read_into_uninitialized_vector_mut},
    vp,
};

pub struct Capabilities {
    handle: vp::Capabilities,
    profiles_fn: ProfilesFn,
}
impl Capabilities {
    pub fn load(handle: vp::Capabilities) -> Self {
        Self {
            handle,
            profiles_fn: ProfilesFn::load_static(),
        }
    }

    pub fn handle(&self) -> vp::Capabilities {
        self.handle
    }

    /// Returns the raw function pointer table
    pub fn profiles_fn(&self) -> &ProfilesFn {
        &self.profiles_fn
    }

    pub unsafe fn destroy_capabilities(
        &self,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.profiles_fn.destroy_capabilities)(self.handle, allocation_callbacks.as_raw_ptr());
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profiles>
    pub unsafe fn get_profiles(&self) -> VkResult<Vec<vp::ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profiles)(self.handle, count, data)
        })
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-required-profiles>
    pub unsafe fn get_profile_required_profiles(
        &self,
        profile_properties: &vp::ProfileProperties,
    ) -> VkResult<Vec<vp::ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_required_profiles)(
                self.handle,
                profile_properties,
                count,
                data,
            )
        })
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-vulkan-api-version>
    pub unsafe fn get_profile_api_version(
        &self,
        profile_properties: &vp::ProfileProperties,
    ) -> u32 {
        (self.profiles_fn.get_profile_api_version)(self.handle, profile_properties)
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-fallbacks>
    pub unsafe fn get_profile_fallbacks(
        &self,
        profile_properties: &vp::ProfileProperties,
    ) -> VkResult<Vec<vp::ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_fallbacks)(self.handle, profile_properties, count, data)
        })
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-with-multiple-variants>
    pub unsafe fn has_multiple_variants_profile(
        &self,
        profile_properties: &vp::ProfileProperties,
    ) -> VkResult<bool> {
        let mut has_multiple_variants = vk::FALSE;
        (self.profiles_fn.has_multiple_variants_profile)(
            self.handle,
            profile_properties,
            &mut has_multiple_variants,
        )
        .result()?;

        Ok(has_multiple_variants == vk::TRUE)
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#checking-instance-level-support>
    pub unsafe fn get_instance_profile_support(
        &self,
        layer_name: Option<&CStr>,
        profile_properties: &vp::ProfileProperties,
    ) -> VkResult<bool> {
        let layer_name_ptr = match layer_name {
            Some(layer_name) => layer_name.as_ptr(),
            _ => core::ptr::null(),
        };

        let mut supported = vk::FALSE;
        (self.profiles_fn.get_instance_profile_support)(
            self.handle,
            layer_name_ptr,
            profile_properties,
            &mut supported,
        )
        .result()?;

        Ok(supported == vk::TRUE)
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#checking-instance-level-support>
    pub unsafe fn get_instance_profile_variants_support(
        &self,
        layer_name: Option<&CStr>,
        profile_properties: &vp::ProfileProperties,
    ) -> VkResult<(bool, Vec<vp::BlockProperties>)> {
        let layer_name_ptr = match layer_name {
            Some(layer_name) => layer_name.as_ptr(),
            _ => core::ptr::null(),
        };

        let mut supported = vk::FALSE;
        let blocks = read_into_uninitialized_vector_mut(|count, data| {
            (self.profiles_fn.get_instance_profile_variants_support)(
                self.handle,
                layer_name_ptr,
                profile_properties,
                &mut supported,
                count,
                data,
            )
        })?;

        match supported {
            vk::TRUE => Ok((true, blocks)),
            vk::FALSE => Ok((false, blocks)),
            _ => panic!("Unexpected result for vk::Bool32: {}", supported),
        }
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#creating-instance-with-profile>
    pub unsafe fn create_instance(
        &self,
        entry: &ash::Entry,
        instance_create_info: &vp::InstanceCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<ash::Instance> {
        let mut instance = core::mem::zeroed();
        (self.profiles_fn.create_instance)(
            self.handle,
            instance_create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut instance,
        )
        .result()?;
        Ok(ash::Instance::load(entry.static_fn(), instance))
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#checking-device-level-support>
    pub unsafe fn get_physical_device_profile_support(
        &self,
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        profile_properties: &vp::ProfileProperties,
    ) -> VkResult<bool> {
        let mut supported = vk::FALSE;
        (self.profiles_fn.get_physical_device_profile_support)(
            self.handle,
            instance.handle(),
            physical_device,
            profile_properties,
            &mut supported,
        )
        .result()?;
        Ok(supported == vk::TRUE)
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#checking-device-level-support>
    pub unsafe fn get_physical_device_profile_variants_support(
        &self,
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        profile_properties: &vp::ProfileProperties,
    ) -> VkResult<(bool, Vec<vp::BlockProperties>)> {
        let mut supported = vk::FALSE;
        let blocks = read_into_uninitialized_vector_mut(|count, data| {
            (self
                .profiles_fn
                .get_physical_device_profile_variants_support)(
                self.handle,
                instance.handle(),
                physical_device,
                profile_properties,
                &mut supported,
                count,
                data,
            )
        })?;

        match supported {
            vk::TRUE => Ok((true, blocks)),
            _ => Ok((false, blocks)),
        }
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#creating-device-with-profile>
    pub unsafe fn create_device(
        &self,
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        device_create_info: &vp::DeviceCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<ash::Device> {
        let mut device = vk::Device::null();
        (self.profiles_fn.create_device)(
            self.handle,
            physical_device,
            device_create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut device,
        )
        .result()?;
        Ok(ash::Device::load(instance.fp_v1_0(), device))
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-instance-extensions>
    pub unsafe fn get_profile_instance_extension_properties(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::ExtensionProperties>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_instance_extension_properties)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-device-extensions>
    pub unsafe fn get_profile_device_extension_properties(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::ExtensionProperties>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_device_extension_properties)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-features>
    pub unsafe fn get_profile_features(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
        features: &mut vk::PhysicalDeviceFeatures2<'_>,
    ) -> VkResult<()> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        (self.profiles_fn.get_profile_features)(
            self.handle,
            profile_properties,
            block_name_ptr,
            <*mut _>::cast(features),
        )
        .result()
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-features>
    pub unsafe fn get_profile_feature_structure_types(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::StructureType>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_feature_structure_types)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-device-properties>
    pub unsafe fn get_profile_properties(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
        properties: &mut vk::PhysicalDeviceProperties2<'_>,
    ) -> VkResult<()> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        (self.profiles_fn.get_profile_properties)(
            self.handle,
            profile_properties,
            block_name_ptr,
            <*mut _>::cast(properties),
        )
        .result()
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-device-properties>
    pub unsafe fn get_profile_property_structure_types(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::StructureType>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_property_structure_types)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-format-properties>
    pub unsafe fn get_profile_formats(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::Format>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_formats)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-format-properties>
    pub unsafe fn get_profile_format_properties(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
        format: vk::Format,
        properties: &mut vk::FormatProperties2<'_>,
    ) -> VkResult<()> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        (self.profiles_fn.get_profile_format_properties)(
            self.handle,
            profile_properties,
            block_name_ptr,
            format,
            <*mut _>::cast(properties),
        )
        .result()
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-format-properties>
    pub unsafe fn get_profile_format_structure_types(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::StructureType>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_format_structure_types)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }

    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-queue-family-properties>
    pub unsafe fn get_profile_queue_family_properties(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::QueueFamilyProperties2KHR<'_>>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_queue_family_properties)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }

    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-queue-family-properties>
    pub unsafe fn get_profile_queue_family_structure_types(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::StructureType>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_queue_family_structure_types)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }
}

pub struct ProfilesFn {
    pub destroy_capabilities: vp::PFN_vpDestroyCapabilities,
    pub get_profiles: vp::PFN_vpGetProfiles,
    pub get_profile_required_profiles: vp::PFN_vpGetProfileRequiredProfiles,
    pub get_profile_api_version: vp::PFN_vpGetProfileAPIVersion,
    pub get_profile_fallbacks: vp::PFN_vpGetProfileFallbacks,
    pub has_multiple_variants_profile: vp::PFN_vpHasMultipleVariantsProfile,
    pub get_instance_profile_support: vp::PFN_vpGetInstanceProfileSupport,
    pub get_instance_profile_variants_support: vp::PFN_vpGetInstanceProfileVariantsSupport,
    pub create_instance: vp::PFN_vpCreateInstance,
    pub get_physical_device_profile_support: vp::PFN_vpGetPhysicalDeviceProfileSupport,
    pub get_physical_device_profile_variants_support:
        vp::PFN_vpGetPhysicalDeviceProfileVariantsSupport,
    pub create_device: vp::PFN_vpCreateDevice,
    pub get_profile_instance_extension_properties: vp::PFN_vpGetProfileInstanceExtensionProperties,
    pub get_profile_device_extension_properties: vp::PFN_vpGetProfileDeviceExtensionProperties,
    pub get_profile_features: vp::PFN_vpGetProfileFeatures,
    pub get_profile_feature_structure_types: vp::PFN_vpGetProfileFeatureStructureTypes,
    pub get_profile_properties: vp::PFN_vpGetProfileProperties,
    pub get_profile_property_structure_types: vp::PFN_vpGetProfilePropertyStructureTypes,
    pub get_profile_formats: vp::PFN_vpGetProfileFormats,
    pub get_profile_format_properties: vp::PFN_vpGetProfileFormatProperties,
    pub get_profile_format_structure_types: vp::PFN_vpGetProfileFormatStructureTypes,
    pub get_profile_queue_family_properties: vp::PFN_vpGetProfileQueueFamilyProperties,
    pub get_profile_queue_family_structure_types: vp::PFN_vpGetProfileQueueFamilyStructureTypes,
}

impl ProfilesFn {
    /// Initializes the table from the statically linked library
    pub(crate) fn load_static() -> Self {
        Self {
            destroy_capabilities: vp::vpDestroyCapabilities,
            get_profiles: vp::vpGetProfiles,
            get_profile_required_profiles: vp::vpGetProfileRequiredProfiles,
            get_profile_api_version: vp::vpGetProfileAPIVersion,
            get_profile_fallbacks: vp::vpGetProfileFallbacks,
            has_multiple_variants_profile: vp::vpHasMultipleVariantsProfile,
            get_instance_profile_support: vp::vpGetInstanceProfileSupport,
            get_instance_profile_variants_support: vp::vpGetInstanceProfileVariantsSupport,
            create_instance: vp::vpCreateInstance,
            get_physical_device_profile_support: vp::vpGetPhysicalDeviceProfileSupport,
            get_physical_device_profile_variants_support:
                vp::vpGetPhysicalDeviceProfileVariantsSupport,
            create_device: vp::vpCreateDevice,
            get_profile_instance_extension_properties: vp::vpGetProfileInstanceExtensionProperties,
            get_profile_device_extension_properties: vp::vpGetProfileDeviceExtensionProperties,
            get_profile_features: vp::vpGetProfileFeatures,
            get_profile_feature_structure_types: vp::vpGetProfileFeatureStructureTypes,
            get_profile_properties: vp::vpGetProfileProperties,
            get_profile_property_structure_types: vp::vpGetProfilePropertyStructureTypes,
            get_profile_formats: vp::vpGetProfileFormats,
            get_profile_format_properties: vp::vpGetProfileFormatProperties,
            get_profile_format_structure_types: vp::vpGetProfileFormatStructureTypes,
            get_profile_queue_family_properties: vp::vpGetProfileQueueFamilyProperties,
            get_profile_queue_family_structure_types: vp::vpGetProfileQueueFamilyStructureTypes,
        }
    }
}
