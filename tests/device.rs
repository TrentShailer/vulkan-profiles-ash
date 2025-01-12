mod common;

use ash::vk;
use common::{
    blocks_match, setup, setup_instance, SUPPORTED, UNSUPPORTED_DEVICE,
    VARIANTS_DEVICE_UNSUPPORTED, VARIANTS_SUPPORTED,
};
use vp_ash::vp;

#[test]
fn supported_device() {
    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();
    let (_, instance) = setup_instance(&capabilities, profile);

    let supported = unsafe {
        capabilities
            .get_physical_device_profile_support(&instance, vk::PhysicalDevice::null(), &profile)
            .unwrap()
    };
    assert!(supported);
}

#[test]
fn unsupported_device() {
    let profile = vp::ProfileProperties::default()
        .profile_name(UNSUPPORTED_DEVICE)
        .unwrap();

    let (_, _, capabilities) = setup();
    let (_, instance) = setup_instance(&capabilities, profile);

    let supported = unsafe {
        capabilities
            .get_physical_device_profile_support(&instance, vk::PhysicalDevice::null(), &profile)
            .unwrap()
    };
    assert!(!supported);
}

#[test]
fn supported_device_variants() {
    let profile = vp::ProfileProperties::default()
        .profile_name(VARIANTS_SUPPORTED)
        .unwrap();

    let expected_blocks = [
        vp::BlockProperties::default()
            .block_name(c"supported_a")
            .unwrap()
            .profiles(profile),
        vp::BlockProperties::default()
            .block_name(c"supported_b")
            .unwrap()
            .profiles(profile),
    ];

    let (_, _, capabilities) = setup();
    let (_, instance) = setup_instance(&capabilities, profile);

    let (supported, blocks) = unsafe {
        capabilities
            .get_physical_device_profile_variants_support(
                &instance,
                vk::PhysicalDevice::null(),
                &profile,
            )
            .unwrap()
    };

    assert!(supported);
    assert_eq!(blocks.len(), expected_blocks.len(), "{:#?}", blocks);

    for index in 0..expected_blocks.len() {
        let expected = expected_blocks[index];
        let block = blocks[index];
        assert!(
            blocks_match(block, expected),
            "Real:\n{:#?}\nExpected:\n{:#?}",
            block,
            expected
        );
    }
}

#[test]
fn unsupported_device_variants() {
    let profile = vp::ProfileProperties::default()
        .profile_name(VARIANTS_DEVICE_UNSUPPORTED)
        .unwrap();

    let expected_blocks = [
        vp::BlockProperties::default()
            .block_name(c"device_unsupported_a")
            .unwrap()
            .profiles(profile),
        vp::BlockProperties::default()
            .block_name(c"device_unsupported_b")
            .unwrap()
            .profiles(profile),
    ];

    let (_, _, capabilities) = setup();
    let (_, instance) = setup_instance(&capabilities, profile);

    let (supported, blocks) = unsafe {
        capabilities
            .get_physical_device_profile_variants_support(
                &instance,
                vk::PhysicalDevice::null(),
                &profile,
            )
            .unwrap()
    };

    assert!(!supported, "{:#?}", blocks);
    assert_eq!(blocks.len(), expected_blocks.len(), "{:#?}", blocks);

    for index in 0..expected_blocks.len() {
        let expected = expected_blocks[index];
        let block = blocks[index];
        assert!(
            blocks_match(block, expected),
            "Real:\n{:#?}\nExpected:\n{:#?}",
            block,
            expected
        );
    }
}

#[test]
fn device_creation() {
    let profiles = [vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap()];

    let (_, _, capabilities) = setup();
    let (_, instance) = setup_instance(&capabilities, profiles[0]);

    let physical_device = unsafe {
        instance
            .enumerate_physical_devices()
            .unwrap()
            .first()
            .unwrap()
            .to_owned()
    };

    let queue_create_infos = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(0)
        .queue_priorities(&[1.0])];

    let vk_create_info = vk::DeviceCreateInfo::default().queue_create_infos(&queue_create_infos);

    let create_info = vp::DeviceCreateInfo::default()
        .enabled_full_profiles(&profiles)
        .create_info(&vk_create_info);

    let device = unsafe {
        capabilities
            .create_device(&instance, physical_device, &create_info, None)
            .unwrap()
    };

    unsafe { device.destroy_device(None) };
}

#[test]
fn device_creation_blocks() {
    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();
    let blocks = [vp::BlockProperties::default()
        .block_name(c"baseline")
        .unwrap()
        .profiles(profile)
        .api_version(vk::make_api_version(0, 1, 2, 0))];

    let (_, _, capabilities) = setup();
    let (_, instance) = setup_instance(&capabilities, profile);

    let physical_device = unsafe {
        instance
            .enumerate_physical_devices()
            .unwrap()
            .first()
            .unwrap()
            .to_owned()
    };

    let queue_create_infos = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(0)
        .queue_priorities(&[1.0])];

    let vk_create_info = vk::DeviceCreateInfo::default().queue_create_infos(&queue_create_infos);

    let create_info = vp::DeviceCreateInfo::default()
        .enabled_profile_blocks(&blocks)
        .create_info(&vk_create_info);

    let device = unsafe {
        capabilities
            .create_device(&instance, physical_device, &create_info, None)
            .unwrap()
    };

    unsafe { device.destroy_device(None) };
}
