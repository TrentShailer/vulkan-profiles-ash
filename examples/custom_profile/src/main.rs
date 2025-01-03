use ash::{prelude::VkResult, vk};
use vulkan_profiles_ash::vp;

pub struct VulkanContext {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
    pub queue_family_index: u32,
}

impl VulkanContext {
    pub unsafe fn new() -> VkResult<Self> {
        let entry = ash::Entry::linked();
        let vk_profiles = vulkan_profiles_ash::VulkanProfiles::linked();

        let profile = vp::ProfileProperties::default()
            .profile_name(c"VP_VULKAN_PROFILES_ASH_custom_profile")
            .unwrap()
            .spec_version(1);
        let enabled_full_profiles = [profile];

        // Instance creation
        if !vk_profiles.get_instance_profile_support(None, &profile)? {
            panic!("Instance does not support the profile {:?}.", profile)
        }

        let api_version = vk_profiles.get_profile_api_version(&profile);

        let app_info = vk::ApplicationInfo::default()
            .api_version(api_version)
            .application_name(c"custom_profile");

        let vk_instance_create_info = vk::InstanceCreateInfo::default().application_info(&app_info);

        let vp_instance_create_info = vp::InstanceCreateInfo::default()
            .create_info(&vk_instance_create_info)
            .enabled_full_profiles(&enabled_full_profiles);

        let instance = vk_profiles.create_instance(&entry, &vp_instance_create_info, None)?;

        // Device selection
        let physical_device = instance
            .enumerate_physical_devices()
            .unwrap()
            .into_iter()
            .filter(|&device| {
                vk_profiles
                    .get_physical_device_profile_support(&instance, device, &profile)
                    .unwrap()
            })
            .min_by_key(|&device| {
                let properties = instance.get_physical_device_properties(device);

                match properties.device_type {
                    vk::PhysicalDeviceType::DISCRETE_GPU => 0,
                    vk::PhysicalDeviceType::INTEGRATED_GPU => 1,
                    vk::PhysicalDeviceType::VIRTUAL_GPU => 2,
                    vk::PhysicalDeviceType::CPU => 3,
                    vk::PhysicalDeviceType::OTHER => 4,
                    _ => 5,
                }
            })
            .expect("No physical device supports the profile");

        // Get the queue family index
        let queue_properties =
            instance.get_physical_device_queue_family_properties(physical_device);

        let (queue_family_index, _queue_family_properties) = queue_properties
            .into_iter()
            .enumerate()
            .find_map(|(index, properties)| {
                if properties.queue_count >= 1
                    && properties.queue_flags.contains(vk::QueueFlags::TRANSFER)
                {
                    Some((index as u32, properties))
                } else {
                    None
                }
            })
            .unwrap();

        // Create device
        let queue_create_infos = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&[1.0; 1])];

        let vk_device_create_info =
            vk::DeviceCreateInfo::default().queue_create_infos(&queue_create_infos);

        let vp_device_create_info = vp::DeviceCreateInfo::default()
            .create_info(&vk_device_create_info)
            .enabled_full_profiles(&enabled_full_profiles);

        let device =
            vk_profiles.create_device(&instance, physical_device, &vp_device_create_info, None)?;

        Ok(Self {
            entry,
            instance,
            physical_device,
            device,
            queue_family_index,
        })
    }
}

fn main() {
    let vulkan_context = unsafe { VulkanContext::new().unwrap() };

    let device_properties = unsafe {
        vulkan_context
            .instance
            .get_physical_device_properties(vulkan_context.physical_device)
    };

    println!(
        "Created Vulkan Context using physical device {:?} running Vulkan {}.{}.{}",
        device_properties
            .device_name_as_c_str()
            .unwrap_or(c"Invalid Name"),
        vk::api_version_major(device_properties.api_version),
        vk::api_version_minor(device_properties.api_version),
        vk::api_version_patch(device_properties.api_version)
    )
}
