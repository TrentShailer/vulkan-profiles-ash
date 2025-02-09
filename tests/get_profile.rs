mod common;

use ash::vk;
use common::{
    all_expected_profiles_exist, setup, supported_profile, video_profile, FALLBACK,
    FALLBACK_FALLBACK, REQUIRES, UNSUPPORTED_DEVICE, UNSUPPORTED_INSTANCE,
    VARIANTS_DEVICE_UNSUPPORTED, VARIANTS_INSTANCE_UNSUPPORTED, VARIANTS_SUPPORTED,
};
use vp_ash::vp;

#[test]
fn get_all_profiles() {
    let expected_profiles = vec![
        supported_profile(),
        video_profile(),
        vp::ProfileProperties::default()
            .profile_name(UNSUPPORTED_DEVICE)
            .unwrap(),
        vp::ProfileProperties::default()
            .profile_name(UNSUPPORTED_INSTANCE)
            .unwrap(),
        vp::ProfileProperties::default()
            .profile_name(REQUIRES)
            .unwrap(),
        vp::ProfileProperties::default()
            .profile_name(FALLBACK)
            .unwrap(),
        vp::ProfileProperties::default()
            .profile_name(FALLBACK_FALLBACK)
            .unwrap(),
        vp::ProfileProperties::default()
            .profile_name(VARIANTS_SUPPORTED)
            .unwrap(),
        vp::ProfileProperties::default()
            .profile_name(VARIANTS_DEVICE_UNSUPPORTED)
            .unwrap(),
        vp::ProfileProperties::default()
            .profile_name(VARIANTS_INSTANCE_UNSUPPORTED)
            .unwrap(),
    ];

    let (_, _, capabilities) = setup();

    let profiles = unsafe { capabilities.get_profiles().unwrap() };
    assert_eq!(profiles.len(), expected_profiles.len());

    assert!(
        all_expected_profiles_exist(&expected_profiles, &profiles),
        "Expected:\n{:#?}\nReal:\n{:#?}",
        expected_profiles,
        profiles
    );
}

#[test]
pub fn get_profile_required_profiles() {
    let profile = vp::ProfileProperties::default()
        .profile_name(REQUIRES)
        .unwrap();

    let expected_profiles = vec![supported_profile()];

    let (_, _, capabilities) = setup();

    let profiles = unsafe {
        capabilities
            .get_profile_required_profiles(&profile)
            .unwrap()
    };
    assert_eq!(profiles.len(), expected_profiles.len());

    assert!(
        all_expected_profiles_exist(&expected_profiles, &profiles),
        "Expected:\n{:#?}\nReal:\n{:#?}",
        expected_profiles,
        profiles
    );
}

#[test]
pub fn get_profile_required_profiles_none() {
    let profile = supported_profile();

    let (_, _, capabilities) = setup();

    let profiles = unsafe {
        capabilities
            .get_profile_required_profiles(&profile)
            .unwrap()
    };
    assert_eq!(profiles.len(), 0);
}

#[test]
fn get_profile_api_version() {
    let profile = supported_profile();

    let (_, _, capabilities) = setup();

    let api_version = unsafe { capabilities.get_profile_api_version(&profile) };
    assert_eq!(api_version, vk::make_api_version(0, 1, 2, 0));
}

#[test]
fn get_profile_fallbacks() {
    let profile = vp::ProfileProperties::default()
        .profile_name(FALLBACK)
        .unwrap();

    let expected_profiles = vec![vp::ProfileProperties::default()
        .profile_name(FALLBACK_FALLBACK)
        .unwrap()];

    let (_, _, capabilities) = setup();

    let profiles = unsafe { capabilities.get_profile_fallbacks(&profile).unwrap() };
    assert_eq!(profiles.len(), expected_profiles.len());

    assert!(
        all_expected_profiles_exist(&expected_profiles, &profiles),
        "Expected:\n{:#?}\nReal:\n{:#?}",
        expected_profiles,
        profiles
    );
}

#[test]
fn get_profile_fallbacks_none() {
    let profile = supported_profile();

    let (_, _, capabilities) = setup();

    let profiles = unsafe {
        capabilities
            .get_profile_required_profiles(&profile)
            .unwrap()
    };
    assert_eq!(profiles.len(), 0);
}

#[test]
fn has_multiple_variants() {
    let profile = vp::ProfileProperties::default()
        .profile_name(VARIANTS_SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    assert!(unsafe {
        capabilities
            .has_multiple_variants_profile(&profile)
            .unwrap()
    });
}

#[test]
fn has_multiple_variants_2() {
    let profile = vp::ProfileProperties::default()
        .profile_name(VARIANTS_DEVICE_UNSUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    assert!(unsafe {
        capabilities
            .has_multiple_variants_profile(&profile)
            .unwrap()
    });
}

#[test]
fn has_multiple_variants_3() {
    let profile = vp::ProfileProperties::default()
        .profile_name(VARIANTS_INSTANCE_UNSUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    assert!(unsafe {
        capabilities
            .has_multiple_variants_profile(&profile)
            .unwrap()
    });
}

#[test]
fn has_multiple_variants_none() {
    let profile = supported_profile();

    let (_, _, capabilities) = setup();

    assert!(!unsafe {
        capabilities
            .has_multiple_variants_profile(&profile)
            .unwrap()
    });
}
