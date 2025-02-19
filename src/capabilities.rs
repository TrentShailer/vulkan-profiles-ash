use core::ffi::CStr;

use alloc::vec::Vec;

use ash::{prelude::VkResult, vk, RawPtr};

use crate::{
    utils::{read_into_uninitialized_vector, read_into_uninitialized_vector_mut},
    vp,
};

#[derive(Clone)]
/// The Vulkan Profiles capabilities, roughly equivalent to an [`ash::Device`].
pub struct Capabilities {
    handle: vp::Capabilities,
    fp: CapabilitiesFn,
}

impl Capabilities {
    /// Create the [`Capabilities`] object using a handle and statically linked function pointers.
    #[cfg(feature = "linked")]
    pub fn linked(handle: vp::Capabilities) -> Self {
        Self {
            handle,
            fp: CapabilitiesFn::linked(),
        }
    }

    /// Returns the underlying [`vp::Capabilities`] handle.
    pub fn handle(&self) -> vp::Capabilities {
        self.handle
    }

    /// Returns the raw function pointer table
    pub fn fp(&self) -> &CapabilitiesFn {
        &self.fp
    }

    /// Destroys allocator object.
    pub unsafe fn destroy_capabilities(
        &self,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_capabilities)(self.handle, allocation_callbacks.as_raw_ptr());
    }

    /// Query the list of available profiles in the library.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profiles>
    pub unsafe fn get_profiles(&self) -> VkResult<Vec<vp::ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_profiles)(self.handle, count, data)
        })
    }

    /// List the required profiles of a profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-required-profiles>
    pub unsafe fn get_profile_required_profiles(
        &self,
        profile_properties: &vp::ProfileProperties,
    ) -> VkResult<Vec<vp::ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_profile_required_profiles)(self.handle, profile_properties, count, data)
        })
    }

    /// Query the profile required Vulkan API version.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-vulkan-api-version>
    pub unsafe fn get_profile_api_version(
        &self,
        profile_properties: &vp::ProfileProperties,
    ) -> u32 {
        (self.fp.get_profile_api_version)(self.handle, profile_properties)
    }

    /// List the recommended fallback profiles of a profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-fallbacks>
    pub unsafe fn get_profile_fallbacks(
        &self,
        profile_properties: &vp::ProfileProperties,
    ) -> VkResult<Vec<vp::ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_profile_fallbacks)(self.handle, profile_properties, count, data)
        })
    }

    /// Query whether the profile has multiple variants. Profiles with multiple variants can only
    /// use vpGetInstanceProfileSupport and vpGetPhysicalDeviceProfileSupport capabilities of the
    /// library. Other function will return a VK_ERROR_UNKNOWN error.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-with-multiple-variants>
    pub unsafe fn has_multiple_variants_profile(
        &self,
        profile_properties: &vp::ProfileProperties,
    ) -> VkResult<bool> {
        let mut has_multiple_variants = vk::FALSE;
        (self.fp.has_multiple_variants_profile)(
            self.handle,
            profile_properties,
            &mut has_multiple_variants,
        )
        .result()?;

        Ok(has_multiple_variants == vk::TRUE)
    }

    /// Check whether a profile is supported at the instance level.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#checking-instance-level-support>
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
        (self.fp.get_instance_profile_support)(
            self.handle,
            layer_name_ptr,
            profile_properties,
            &mut supported,
        )
        .result()?;

        Ok(supported == vk::TRUE)
    }

    /// Check whether a variant of a profile is supported at the instance level and report this list
    /// of blocks used to validate the profiles.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#checking-instance-level-support>
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
            (self.fp.get_instance_profile_variants_support)(
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

    /// Create a VkInstance with the profile instance extensions enabled.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#creating-instance-with-profile>
    pub unsafe fn create_instance(
        &self,
        entry: &ash::Entry,
        instance_create_info: &vp::InstanceCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<ash::Instance> {
        let mut instance = core::mem::zeroed();
        (self.fp.create_instance)(
            self.handle,
            instance_create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut instance,
        )
        .result()?;
        Ok(ash::Instance::load(entry.static_fn(), instance))
    }

    /// Check whether a profile is supported by the physical device.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#checking-device-level-support>
    pub unsafe fn get_physical_device_profile_support(
        &self,
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        profile_properties: &vp::ProfileProperties,
    ) -> VkResult<bool> {
        let mut supported = vk::FALSE;
        (self.fp.get_physical_device_profile_support)(
            self.handle,
            instance.handle(),
            physical_device,
            profile_properties,
            &mut supported,
        )
        .result()?;
        Ok(supported == vk::TRUE)
    }

    /// Check whether a variant of a profile is supported by the physical device and report this
    /// list of blocks used to validate the profiles.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#checking-device-level-support>
    pub unsafe fn get_physical_device_profile_variants_support(
        &self,
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        profile_properties: &vp::ProfileProperties,
    ) -> VkResult<(bool, Vec<vp::BlockProperties>)> {
        let mut supported = vk::FALSE;
        let blocks = read_into_uninitialized_vector_mut(|count, data| {
            (self.fp.get_physical_device_profile_variants_support)(
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

    /// Create a VkDevice with the profile features and device extensions enabled.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#creating-device-with-profile>
    pub unsafe fn create_device(
        &self,
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        device_create_info: &vp::DeviceCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<ash::Device> {
        let mut device = vk::Device::null();
        (self.fp.create_device)(
            self.handle,
            physical_device,
            device_create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut device,
        )
        .result()?;
        Ok(ash::Device::load(instance.fp_v1_0(), device))
    }

    /// Query the list of instance extensions of a profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-instance-extensions>
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
            (self.fp.get_profile_instance_extension_properties)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }

    /// Query the list of device extensions of a profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-device-extensions>
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
            (self.fp.get_profile_device_extension_properties)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }

    /// Fill the feature structures with the requirements of a profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-features>
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

        (self.fp.get_profile_features)(
            self.handle,
            profile_properties,
            block_name_ptr,
            <*mut _>::cast(features),
        )
        .result()
    }

    /// Query the list of feature structure types specified by the profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-features>
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
            (self.fp.get_profile_feature_structure_types)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }

    /// Fill the property structures with the requirements of a profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-device-properties>
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

        (self.fp.get_profile_properties)(
            self.handle,
            profile_properties,
            block_name_ptr,
            <*mut _>::cast(properties),
        )
        .result()
    }

    /// Query the list of property structure types specified by the profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-device-properties>
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
            (self.fp.get_profile_property_structure_types)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }

    /// Fill the queue family property structures with the requirements of a profile.
    ///
    /// Returns the number of queue family properties.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-queue-family-properties>
    pub unsafe fn get_profile_queue_family_properties(
        &self,
        profile_properties: &vp::ProfileProperties,
        block_name: Option<&CStr>,
        property_count: &mut u32,
        queue_family_properties: Option<&mut [vk::QueueFamilyProperties2KHR<'_>]>,
    ) -> VkResult<()> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        let queue_family_properties_ptr = match queue_family_properties {
            Some(properties) => properties.as_mut_ptr(),
            None => core::ptr::null_mut(),
        };

        (self.fp.get_profile_queue_family_properties)(
            self.handle,
            profile_properties,
            block_name_ptr,
            property_count,
            queue_family_properties_ptr,
        )
        .result()
    }

    /// Query the list of queue family property structure types specified by the profile.
    ///
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
            (self.fp.get_profile_queue_family_structure_types)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }

    /// Query the list of formats with specified requirements by a profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-format-properties>
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
            (self.fp.get_profile_formats)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }

    /// Query the requirements of a format for a profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-format-properties>
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

        (self.fp.get_profile_format_properties)(
            self.handle,
            profile_properties,
            block_name_ptr,
            format,
            <*mut _>::cast(properties),
        )
        .result()
    }

    /// Query the list of format structure types specified by the profile.
    ///
    /// <https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html#query-profile-format-properties>
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
            (self.fp.get_profile_format_structure_types)(
                self.handle,
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }
}

#[derive(Clone)]
/// Function pointer table for [Capabilities].
pub struct CapabilitiesFn {
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

impl CapabilitiesFn {
    /// Load the function pointers from the statically linked library.
    #[cfg(feature = "linked")]
    pub(crate) fn linked() -> Self {
        Self {
            destroy_capabilities: vp::linked::vpDestroyCapabilities,
            get_profiles: vp::linked::vpGetProfiles,
            get_profile_required_profiles: vp::linked::vpGetProfileRequiredProfiles,
            get_profile_api_version: vp::linked::vpGetProfileAPIVersion,
            get_profile_fallbacks: vp::linked::vpGetProfileFallbacks,
            has_multiple_variants_profile: vp::linked::vpHasMultipleVariantsProfile,
            get_instance_profile_support: vp::linked::vpGetInstanceProfileSupport,
            get_instance_profile_variants_support: vp::linked::vpGetInstanceProfileVariantsSupport,
            create_instance: vp::linked::vpCreateInstance,
            get_physical_device_profile_support: vp::linked::vpGetPhysicalDeviceProfileSupport,
            get_physical_device_profile_variants_support:
                vp::linked::vpGetPhysicalDeviceProfileVariantsSupport,
            create_device: vp::linked::vpCreateDevice,
            get_profile_instance_extension_properties:
                vp::linked::vpGetProfileInstanceExtensionProperties,
            get_profile_device_extension_properties:
                vp::linked::vpGetProfileDeviceExtensionProperties,
            get_profile_features: vp::linked::vpGetProfileFeatures,
            get_profile_feature_structure_types: vp::linked::vpGetProfileFeatureStructureTypes,
            get_profile_properties: vp::linked::vpGetProfileProperties,
            get_profile_property_structure_types: vp::linked::vpGetProfilePropertyStructureTypes,
            get_profile_formats: vp::linked::vpGetProfileFormats,
            get_profile_format_properties: vp::linked::vpGetProfileFormatProperties,
            get_profile_format_structure_types: vp::linked::vpGetProfileFormatStructureTypes,
            get_profile_queue_family_properties: vp::linked::vpGetProfileQueueFamilyProperties,
            get_profile_queue_family_structure_types:
                vp::linked::vpGetProfileQueueFamilyStructureTypes,
        }
    }
}
