mod common;

use ash::vk;
use common::{
    blocks_match, setup, supported_profile, UNSUPPORTED_INSTANCE, VARIANTS_INSTANCE_UNSUPPORTED,
    VARIANTS_SUPPORTED,
};
use vp_ash::vp;

#[test]
fn supported_instance() {
    let profile = supported_profile();

    let (_, _, capabilities) = setup();

    assert!(unsafe {
        capabilities
            .get_instance_profile_support(None, &profile)
            .unwrap()
    });
}

#[test]
fn unsupported_instance() {
    let profile = vp::ProfileProperties::default()
        .profile_name(UNSUPPORTED_INSTANCE)
        .unwrap();

    let (_, _, capabilities) = setup();

    assert!(!unsafe {
        capabilities
            .get_instance_profile_support(None, &profile)
            .unwrap()
    });
}

#[test]
fn supported_instance_variants() {
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
        vp::BlockProperties::default()
            .block_name(c"device_unsupported_a")
            .unwrap()
            .profiles(profile),
    ];

    let (_, _, capabilities) = setup();

    let (supported, blocks) = unsafe {
        capabilities
            .get_instance_profile_variants_support(None, &profile)
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
fn unsupported_instance_variants() {
    let profile = vp::ProfileProperties::default()
        .profile_name(VARIANTS_INSTANCE_UNSUPPORTED)
        .unwrap();

    let expected_blocks = [
        vp::BlockProperties::default()
            .block_name(c"instance_unsupported_a")
            .unwrap()
            .profiles(profile),
        vp::BlockProperties::default()
            .block_name(c"instance_unsupported_b")
            .unwrap()
            .profiles(profile),
    ];

    let (_, _, capabilities) = setup();

    let (supported, blocks) = unsafe {
        capabilities
            .get_instance_profile_variants_support(None, &profile)
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
fn instance_creation() {
    let profiles = [supported_profile()];

    let entry = ash::Entry::linked();
    let (_, _, capabilities) = setup();

    let create_info = vk::InstanceCreateInfo::default();
    let instance_create_info = vp::InstanceCreateInfo::default()
        .create_info(&create_info)
        .enabled_full_profiles(&profiles);
    let instance = unsafe {
        capabilities
            .create_instance(&entry, &instance_create_info, None)
            .unwrap()
    };

    unsafe { instance.destroy_instance(None) };
}

#[test]
fn instance_creation_blocks() {
    let profile = supported_profile();
    let blocks = [vp::BlockProperties::default()
        .block_name(c"baseline")
        .unwrap()
        .profiles(profile)
        .api_version(vk::make_api_version(0, 1, 2, 0))];

    let entry = ash::Entry::linked();
    let (_, _, capabilities) = setup();

    let create_info = vk::InstanceCreateInfo::default();
    let instance_create_info = vp::InstanceCreateInfo::default()
        .create_info(&create_info)
        .enabled_profile_blocks(&blocks);
    let instance = unsafe {
        capabilities
            .create_instance(&entry, &instance_create_info, None)
            .unwrap()
    };

    unsafe { instance.destroy_instance(None) };
}
