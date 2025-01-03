#![warn(
    clippy::alloc_instead_of_core,
    clippy::use_self,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    deprecated_in_future,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications
)]
#![allow(
    clippy::missing_safety_doc,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms
)]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use core::ffi::CStr;

use alloc::vec::Vec;
use ash::{prelude::VkResult, vk, RawPtr};
use utils::{read_into_uninitialized_vector, read_into_uninitialized_vector_mut};
use vp::{BlockProperties, DeviceCreateInfo, InstanceCreateInfo, ProfileProperties};

#[cfg(feature = "debug")]
mod enum_debugs;
mod utils;
pub mod vp;

/// Holds the Vulkan Profiles functions.
pub struct VulkanProfiles {
    profiles_fn: vp::ProfilesFn,
}

impl VulkanProfiles {
    /// Loads the function pointers when the Vulkan Profiles library is statically linked.
    #[cfg(feature = "linked")]
    #[cfg_attr(docsrs, doc(cfg(feature = "linked")))]
    pub fn linked() -> Self {
        Self {
            profiles_fn: vp::ProfilesFn::load_static(),
        }
    }

    /// Returns the raw function pointer table
    pub fn profiles_fn(&self) -> &vp::ProfilesFn {
        &self.profiles_fn
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profiles>
    pub unsafe fn get_profiles(&self) -> VkResult<Vec<ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| (self.profiles_fn.get_profiles)(count, data))
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-required-profiles>
    pub unsafe fn get_profile_required_profiles(
        &self,
        profile_properties: &ProfileProperties,
    ) -> VkResult<Vec<ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_required_profiles)(profile_properties, count, data)
        })
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-vulkan-api-version>
    pub unsafe fn get_profile_api_version(&self, profile_properties: &ProfileProperties) -> u32 {
        (self.profiles_fn.get_profile_api_version)(profile_properties)
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-fallbacks>
    pub unsafe fn get_profile_fallbacks(
        &self,
        profile_properties: &ProfileProperties,
    ) -> VkResult<Vec<ProfileProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_fallbacks)(profile_properties, count, data)
        })
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-with-multiple-variants>
    pub unsafe fn has_multiple_variants_profile(
        &self,
        profile_properties: &ProfileProperties,
    ) -> VkResult<bool> {
        let mut has_multiple_variants = vk::FALSE;
        (self.profiles_fn.has_multiple_variants_profile)(
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
        profile_properties: &ProfileProperties,
    ) -> VkResult<bool> {
        let layer_name_ptr = match layer_name {
            Some(layer_name) => layer_name.as_ptr(),
            _ => core::ptr::null(),
        };

        let mut supported = vk::FALSE;
        (self.profiles_fn.get_instance_profile_support)(
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
        profile_properties: &ProfileProperties,
    ) -> VkResult<Option<Vec<BlockProperties>>> {
        let layer_name_ptr = match layer_name {
            Some(layer_name) => layer_name.as_ptr(),
            _ => core::ptr::null(),
        };

        // Preliminary support check
        let mut supported = vk::FALSE;
        let mut count = 0;
        (self.profiles_fn.get_instance_profile_variants_support)(
            layer_name_ptr,
            profile_properties,
            &mut supported,
            &mut count,
            core::ptr::null_mut(),
        )
        .result()?;

        if supported == vk::FALSE {
            return Ok(None);
        }

        // Read blocks
        let blocks = read_into_uninitialized_vector_mut(|count, data| {
            (self.profiles_fn.get_instance_profile_variants_support)(
                layer_name_ptr,
                profile_properties,
                &mut supported,
                count,
                data,
            )
        })?;

        match supported {
            vk::TRUE => Ok(Some(blocks)),
            _ => Ok(None),
        }
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#creating-instance-with-profile>
    pub unsafe fn create_instance(
        &self,
        entry: &ash::Entry,
        instance_create_info: &InstanceCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<ash::Instance> {
        let mut instance = core::mem::zeroed();
        (self.profiles_fn.create_instance)(
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
        profile_properties: &ProfileProperties,
    ) -> VkResult<bool> {
        let mut supported = vk::FALSE;
        (self.profiles_fn.get_physical_device_profile_support)(
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
        profile_properties: &ProfileProperties,
    ) -> VkResult<Option<Vec<BlockProperties>>> {
        // Preliminary support check
        let mut supported = vk::FALSE;
        let mut count = 0;
        (self
            .profiles_fn
            .get_physical_device_profile_variants_support)(
            instance.handle(),
            physical_device,
            profile_properties,
            &mut supported,
            &mut count,
            core::ptr::null_mut(),
        )
        .result()?;

        if supported == vk::FALSE {
            return Ok(None);
        }

        // Read blocks
        let blocks = read_into_uninitialized_vector_mut(|count, data| {
            (self
                .profiles_fn
                .get_physical_device_profile_variants_support)(
                instance.handle(),
                physical_device,
                profile_properties,
                &mut supported,
                count,
                data,
            )
        })?;

        match supported {
            vk::TRUE => Ok(Some(blocks)),
            _ => Ok(None),
        }
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#creating-device-with-profile>
    pub unsafe fn create_device(
        &self,
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        device_create_info: &DeviceCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<ash::Device> {
        let mut device = vk::Device::null();
        (self.profiles_fn.create_device)(
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
        profile_properties: &ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::ExtensionProperties>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_instance_extension_properties)(
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
        profile_properties: &ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::ExtensionProperties>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_instance_extension_properties)(
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
        profile_properties: &ProfileProperties,
        block_name: Option<&CStr>,
        features: &mut vk::PhysicalDeviceFeatures2<'_>,
    ) -> VkResult<()> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        (self.profiles_fn.get_profile_features)(
            profile_properties,
            block_name_ptr,
            <*mut _>::cast(features),
        )
        .result()
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-features>
    pub unsafe fn get_profile_feature_structure_types(
        &self,
        profile_properties: &ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::StructureType>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_feature_structure_types)(
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
        profile_properties: &ProfileProperties,
        block_name: Option<&CStr>,
        properties: &mut vk::PhysicalDeviceProperties2<'_>,
    ) -> VkResult<()> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        (self.profiles_fn.get_profile_properties)(
            profile_properties,
            block_name_ptr,
            <*mut _>::cast(properties),
        )
        .result()
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-device-properties>
    pub unsafe fn get_profile_property_structure_types(
        &self,
        profile_properties: &ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::StructureType>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_property_structure_types)(
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
        profile_properties: &ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::Format>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_formats)(profile_properties, block_name_ptr, count, data)
        })
    }

    /// <https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#query-profile-format-properties>
    pub unsafe fn get_profile_format_properties(
        &self,
        profile_properties: &ProfileProperties,
        block_name: Option<&CStr>,
        format: vk::Format,
        properties: &mut vk::FormatProperties2<'_>,
    ) -> VkResult<()> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        (self.profiles_fn.get_profile_format_properties)(
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
        profile_properties: &ProfileProperties,
        block_name: Option<&CStr>,
    ) -> VkResult<Vec<vk::StructureType>> {
        let block_name_ptr = match block_name {
            Some(name) => name.as_ptr(),
            None => core::ptr::null(),
        };

        read_into_uninitialized_vector(|count, data| {
            (self.profiles_fn.get_profile_format_structure_types)(
                profile_properties,
                block_name_ptr,
                count,
                data,
            )
        })
    }
}
