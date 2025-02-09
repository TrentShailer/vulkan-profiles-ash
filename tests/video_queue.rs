use ash::vk;
use common::{setup, video_profile};
use vp_ash::vp;

mod common;

#[test]
fn get_video_profiles() {
    let expected_profiles = [vp::VideoProfileProperties::default()
        .name(c"H.264 Decode (4:2:0 8-bit) Main progressive")
        .unwrap()];

    let profile = video_profile();
    let (_, _, capabilities) = setup();
    let vq_capabilities = vp::video_queue::Capabilities::linked(capabilities.handle());

    let profiles = unsafe {
        vq_capabilities
            .get_profile_video_profiles(&profile, None)
            .unwrap()
    };

    assert_eq!(profiles.len(), expected_profiles.len(), "{:#?}", profiles);
    for expected in expected_profiles {
        assert!(profiles.contains(&expected), "{:#?}", profiles);
    }
}

#[test]
fn get_video_profile_info() {
    let expected_h264 = vk::VideoDecodeH264ProfileInfoKHR::default()
        .picture_layout(vk::VideoDecodeH264PictureLayoutFlagsKHR::PROGRESSIVE)
        .std_profile_idc(0x4d);
    let expected_info = vk::VideoProfileInfoKHR::default()
        .chroma_bit_depth(vk::VideoComponentBitDepthFlagsKHR::TYPE_8)
        .chroma_subsampling(vk::VideoChromaSubsamplingFlagsKHR::TYPE_420)
        .luma_bit_depth(vk::VideoComponentBitDepthFlagsKHR::TYPE_8)
        .video_codec_operation(vk::VideoCodecOperationFlagsKHR::DECODE_H264);

    let profile = video_profile();
    let (_, _, capabilities) = setup();
    let vq_capabilities = vp::video_queue::Capabilities::linked(capabilities.handle());

    let mut h264 = vk::VideoDecodeH264ProfileInfoKHR::default();
    let mut info = vk::VideoProfileInfoKHR::default().push_next(&mut h264);

    unsafe {
        vq_capabilities
            .get_profile_video_profile_info(&profile, None, 0, &mut info)
            .unwrap()
    };

    assert_eq!(expected_info.chroma_bit_depth, info.chroma_bit_depth);
    assert_eq!(expected_info.luma_bit_depth, info.luma_bit_depth);
    assert_eq!(expected_info.chroma_subsampling, info.chroma_subsampling);
    assert_eq!(
        expected_info.video_codec_operation,
        info.video_codec_operation
    );

    assert_eq!(expected_h264.picture_layout, h264.picture_layout);
    assert_eq!(expected_h264.std_profile_idc, h264.std_profile_idc);
}

#[test]
fn get_video_profile_info_structure_types() {
    let expected = [
        vk::StructureType::VIDEO_PROFILE_INFO_KHR,
        vk::StructureType::VIDEO_DECODE_H264_PROFILE_INFO_KHR,
    ];

    let profile = video_profile();
    let (_, _, capabilities) = setup();
    let vq_capabilities = vp::video_queue::Capabilities::linked(capabilities.handle());

    let s_types = unsafe {
        vq_capabilities
            .get_profile_video_profile_info_structure_types(&profile, None, 0)
            .unwrap()
    };

    assert_eq!(s_types.len(), expected.len(), "{:#?}", s_types);

    for expected in expected {
        assert!(s_types.contains(&expected), "{:#?}", s_types);
    }
}

#[test]
fn get_video_capabilities() {
    let expected_h264 = vk::VideoDecodeH264CapabilitiesKHR::default()
        .max_level_idc(vk::native::StdVideoH264LevelIdc_STD_VIDEO_H264_LEVEL_IDC_5_2);
    let expected_info = vk::VideoCapabilitiesKHR::default()
        .max_coded_extent(vk::Extent2D::default().width(1920).height(1080))
        .max_dpb_slots(17)
        .max_active_reference_pictures(16);

    let profile = video_profile();
    let (_, _, capabilities) = setup();
    let vq_capabilities = vp::video_queue::Capabilities::linked(capabilities.handle());

    let mut h264 = vk::VideoDecodeH264CapabilitiesKHR::default();
    let mut info = vk::VideoCapabilitiesKHR::default().push_next(&mut h264);

    unsafe {
        vq_capabilities
            .get_profile_video_capabilities(&profile, None, 0, &mut info)
            .unwrap()
    };

    assert_eq!(expected_info.max_coded_extent, info.max_coded_extent);
    assert_eq!(expected_info.max_dpb_slots, info.max_dpb_slots);
    assert_eq!(
        expected_info.max_active_reference_pictures,
        info.max_active_reference_pictures
    );

    assert_eq!(expected_h264.max_level_idc, h264.max_level_idc);
}

#[test]
fn get_video_capability_structure_types() {
    let expected = [
        vk::StructureType::VIDEO_CAPABILITIES_KHR,
        vk::StructureType::VIDEO_DECODE_H264_CAPABILITIES_KHR,
    ];

    let profile = video_profile();
    let (_, _, capabilities) = setup();
    let vq_capabilities = vp::video_queue::Capabilities::linked(capabilities.handle());

    let s_types = unsafe {
        vq_capabilities
            .get_profile_video_capability_structure_types(&profile, None, 0)
            .unwrap()
    };

    assert_eq!(s_types.len(), expected.len(), "{:#?}", s_types);

    for expected in expected {
        assert!(s_types.contains(&expected), "{:#?}", s_types);
    }
}

#[test]
fn get_video_format_structure_types() {
    let expected = [vk::StructureType::VIDEO_FORMAT_PROPERTIES_KHR];

    let profile = video_profile();
    let (_, _, capabilities) = setup();
    let vq_capabilities = vp::video_queue::Capabilities::linked(capabilities.handle());

    let s_types = unsafe {
        vq_capabilities
            .get_profile_video_format_structure_types(&profile, None, 0)
            .unwrap()
    };

    assert_eq!(s_types.len(), expected.len(), "{:#?}", s_types);

    for expected in expected {
        assert!(s_types.contains(&expected), "{:#?}", s_types);
    }
}

// ----

#[test]
fn get_video_format_count() {
    let expected = 1;

    let profile = video_profile();
    let (_, _, capabilities) = setup();
    let vq_capabilities = vp::video_queue::Capabilities::linked(capabilities.handle());

    let mut count = 0;
    let result = unsafe {
        vq_capabilities.get_profile_video_format_properties(&profile, None, 0, &mut count, None)
    };

    assert!(result.is_ok());
    assert_eq!(count, expected);
}

#[test]
fn get_video_formats() {
    let expected_0 = vk::VideoFormatPropertiesKHR::default()
        .format(vk::Format::G8_B8R8_2PLANE_420_UNORM)
        .image_type(vk::ImageType::TYPE_2D)
        .image_usage_flags(vk::ImageUsageFlags::VIDEO_DECODE_DPB_KHR);

    let profile = video_profile();
    let (_, _, capabilities) = setup();
    let vq_capabilities = vp::video_queue::Capabilities::linked(capabilities.handle());

    // Setup properties
    let mut count: u32 = 1;
    let mut extra_properties: Vec<_> = (0..count).map(|_| ()).collect();

    let mut properties: Vec<_> = extra_properties
        .iter_mut()
        .map(|_property| vk::VideoFormatPropertiesKHR::default())
        .collect();

    unsafe {
        vq_capabilities
            .get_profile_video_format_properties(
                &profile,
                None,
                0,
                &mut count,
                Some(&mut properties),
            )
            .unwrap()
    };

    assert_eq!(expected_0.format, properties[0].format);
    assert_eq!(expected_0.image_type, properties[0].image_type);
    assert_eq!(
        expected_0.image_usage_flags,
        properties[0].image_usage_flags
    );
}
