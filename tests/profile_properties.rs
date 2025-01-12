use ash::{ext, khr, vk};
use common::{setup, SUPPORTED};
use vp_ash::vp;

mod common;

#[test]
fn instance_extension_properties() {
    let expected_properties = [vk::ExtensionProperties::default()
        .extension_name(ext::debug_utils::NAME)
        .unwrap()
        .spec_version(1)];

    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    let properties = unsafe {
        capabilities
            .get_profile_instance_extension_properties(&profile, None)
            .unwrap()
    };

    assert_eq!(properties.len(), expected_properties.len());

    for index in 0..expected_properties.len() {
        let expected = expected_properties[index];
        let property = properties[index];

        assert!(expected.extension_name == property.extension_name);
        assert!(expected.spec_version == property.spec_version);
    }
}

#[test]
fn device_extension_properties() {
    let expected_properties = [vk::ExtensionProperties::default()
        .extension_name(khr::synchronization2::NAME)
        .unwrap()
        .spec_version(1)];

    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    let properties = unsafe {
        capabilities
            .get_profile_device_extension_properties(&profile, None)
            .unwrap()
    };

    assert_eq!(properties.len(), expected_properties.len());

    for index in 0..expected_properties.len() {
        let expected = expected_properties[index];
        let property = properties[index];

        assert!(
            expected.extension_name == property.extension_name,
            "{:#?}",
            property
        );
        assert!(expected.spec_version == property.spec_version);
    }
}

#[test]
fn profile_features_s_types() {
    let expected_s_types = [
        vk::StructureType::PHYSICAL_DEVICE_FEATURES_2,
        vk::StructureType::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES,
    ];

    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    let s_types = unsafe {
        capabilities
            .get_profile_feature_structure_types(&profile, None)
            .unwrap()
    };

    assert_eq!(s_types.len(), expected_s_types.len());

    for index in 0..expected_s_types.len() {
        let expected = expected_s_types[index];
        let s_type = s_types[index];

        assert!(expected == s_type, "{:#?}", s_type);
    }
}

#[test]
fn profile_features() {
    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    let mut float16 = vk::PhysicalDeviceShaderFloat16Int8Features::default();
    let mut features2 = vk::PhysicalDeviceFeatures2::default().push_next(&mut float16);

    unsafe {
        capabilities
            .get_profile_features(&profile, None, &mut features2)
            .unwrap()
    };

    assert!(features2.features.shader_float64 == vk::TRUE);
    assert!(float16.shader_float16 == vk::TRUE);
}

#[test]
fn profile_property_s_types() {
    let expected_s_types = [
        vk::StructureType::PHYSICAL_DEVICE_PROPERTIES_2,
        vk::StructureType::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES,
    ];

    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    let s_types = unsafe {
        capabilities
            .get_profile_property_structure_types(&profile, None)
            .unwrap()
    };

    assert_eq!(s_types.len(), expected_s_types.len());

    for index in 0..expected_s_types.len() {
        let expected = expected_s_types[index];
        let s_type = s_types[index];

        assert!(expected == s_type, "{:#?}", s_type);
    }
}

#[test]
fn profile_properties() {
    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    let mut subgroup = vk::PhysicalDeviceSubgroupProperties::default();
    let mut properties2 = vk::PhysicalDeviceProperties2::default().push_next(&mut subgroup);

    unsafe {
        capabilities
            .get_profile_properties(&profile, None, &mut properties2)
            .unwrap()
    };

    assert!(properties2.properties.limits.max_image_dimension2_d == 16384);
    assert!(subgroup
        .supported_stages
        .contains(vk::ShaderStageFlags::COMPUTE));
}

#[test]
fn profile_format_s_types() {
    let expected_s_types = [
        vk::StructureType::FORMAT_PROPERTIES_2,
        vk::StructureType::FORMAT_PROPERTIES_3,
    ];

    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    let s_types = unsafe {
        capabilities
            .get_profile_format_structure_types(&profile, None)
            .unwrap()
    };

    assert_eq!(s_types.len(), expected_s_types.len(), "{:#?}", s_types);

    for index in 0..expected_s_types.len() {
        let expected = expected_s_types[index];
        let s_type = s_types[index];

        assert!(expected == s_type, "{:#?}", s_type);
    }
}

#[test]
fn profile_formats() {
    let expected_formats = [vk::Format::R8G8B8A8_UNORM];

    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    let formats = unsafe { capabilities.get_profile_formats(&profile, None).unwrap() };

    assert_eq!(formats.len(), expected_formats.len(), "{:#?}", formats);

    for index in 0..expected_formats.len() {
        let expected = expected_formats[index];
        let format = formats[index];

        assert!(expected == format, "{:#?}", format);
    }
}

#[test]
fn profile_format_properties() {
    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    let mut properties2 = vk::FormatProperties2::default();

    unsafe {
        capabilities
            .get_profile_format_properties(
                &profile,
                None,
                vk::Format::R8G8B8A8_UNORM,
                &mut properties2,
            )
            .unwrap()
    };

    assert!(
        properties2.format_properties.optimal_tiling_features
            == vk::FormatFeatureFlags::TRANSFER_SRC
    );
    assert!(
        properties2.format_properties.buffer_features
            == vk::FormatFeatureFlags::STORAGE_TEXEL_BUFFER
    );
    assert!(
        properties2.format_properties.linear_tiling_features
            == vk::FormatFeatureFlags::TRANSFER_DST
    );
}
