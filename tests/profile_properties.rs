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
    let expected_properties = [
        vk::ExtensionProperties::default()
            .extension_name(khr::synchronization2::NAME)
            .unwrap()
            .spec_version(1),
        vk::ExtensionProperties::default()
            .extension_name(khr::video_queue::NAME)
            .unwrap()
            .spec_version(1),
        vk::ExtensionProperties::default()
            .extension_name(khr::video_decode_queue::NAME)
            .unwrap()
            .spec_version(1),
        vk::ExtensionProperties::default()
            .extension_name(khr::video_decode_av1::NAME)
            .unwrap()
            .spec_version(1),
    ];

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

    for expected in expected_properties {
        assert!(
            properties
                .iter()
                .any(|property| property.extension_name == expected.extension_name),
            "{:#?}",
            expected
        )
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

    for expected in expected_s_types {
        assert!(
            s_types.iter().any(|&s_type| s_type == expected),
            "{:#?}",
            expected
        )
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

    for expected in expected_s_types {
        assert!(
            s_types.iter().any(|&s_type| s_type == expected),
            "{:#?}",
            expected
        )
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
fn profile_queue_family_s_types() {
    let expected_s_types = [
        vk::StructureType::QUEUE_FAMILY_VIDEO_PROPERTIES_KHR,
        vk::StructureType::QUEUE_FAMILY_PROPERTIES_2_KHR,
    ];

    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    let s_types = unsafe {
        capabilities
            .get_profile_queue_family_structure_types(&profile, None)
            .unwrap()
    };

    assert_eq!(s_types.len(), expected_s_types.len(), "{:#?}", s_types);

    for expected in expected_s_types {
        assert!(
            s_types.iter().any(|&s_type| s_type == expected),
            "{:#?}",
            expected
        )
    }
}

#[test]
fn profile_queue_family_properties_count() {
    let expected = 2;

    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    let mut count = 0;
    let result = unsafe {
        capabilities.get_profile_queue_family_properties(&profile, None, &mut count, None)
    };

    assert!(result.is_ok());
    assert_eq!(count, expected);
}

#[test]
fn profile_queue_family_properties_incomplete() {
    let expected = vk::QueueFamilyProperties::default()
        .queue_count(2)
        .queue_flags(vk::QueueFlags::COMPUTE | vk::QueueFlags::GRAPHICS);

    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    let mut count = 1;
    let mut properties = vk::QueueFamilyProperties2KHR::default();
    let result = unsafe {
        capabilities.get_profile_queue_family_properties(
            &profile,
            None,
            &mut count,
            Some(core::slice::from_mut(&mut properties)),
        )
    };

    assert_eq!(result, Err(vk::Result::INCOMPLETE));
    assert_eq!(
        properties.queue_family_properties.queue_count,
        expected.queue_count
    );
    assert_eq!(
        properties.queue_family_properties.queue_flags,
        expected.queue_flags
    );
}

#[test]
fn profile_queue_family_properties() {
    let expected_0 = vk::QueueFamilyProperties2KHR::default().queue_family_properties(
        vk::QueueFamilyProperties::default()
            .queue_count(2)
            .queue_flags(vk::QueueFlags::COMPUTE | vk::QueueFlags::GRAPHICS),
    );

    let mut expected_1_video = vk::QueueFamilyVideoPropertiesKHR::default()
        .video_codec_operations(vk::VideoCodecOperationFlagsKHR::DECODE_AV1);
    let expected_1 = vk::QueueFamilyProperties2KHR::default()
        .queue_family_properties(
            vk::QueueFamilyProperties::default()
                .queue_count(1)
                .queue_flags(vk::QueueFlags::VIDEO_DECODE_KHR),
        )
        .push_next(&mut expected_1_video);

    let profile = vp::ProfileProperties::default()
        .profile_name(SUPPORTED)
        .unwrap();

    let (_, _, capabilities) = setup();

    // Setup properties
    let mut count: u32 = 2;
    let mut extra_properties: Vec<_> = (0..count)
        .map(|_| vk::QueueFamilyVideoPropertiesKHR::default())
        .collect();

    let mut properties: Vec<_> = extra_properties
        .iter_mut()
        .map(|property| vk::QueueFamilyProperties2KHR::default().push_next(property))
        .collect();

    unsafe {
        capabilities
            .get_profile_queue_family_properties(&profile, None, &mut count, Some(&mut properties))
            .unwrap()
    };

    assert_eq!(
        properties[0].queue_family_properties.queue_count,
        expected_0.queue_family_properties.queue_count
    );
    assert_eq!(
        properties[0].queue_family_properties.queue_flags,
        expected_0.queue_family_properties.queue_flags
    );

    assert_eq!(
        properties[1].queue_family_properties.queue_count,
        expected_1.queue_family_properties.queue_count
    );
    assert_eq!(
        properties[1].queue_family_properties.queue_flags,
        expected_1.queue_family_properties.queue_flags
    );
    assert_eq!(
        extra_properties[1].video_codec_operations,
        expected_1_video.video_codec_operations
    );
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
