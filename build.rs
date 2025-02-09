use std::{env, path::Path};

fn main() {
    // Rerun on env change.
    println!("cargo:rerun-if-env-changed=VULKAN_SDK");
    println!("cargo:rerun-if-env-changed=VULKAN_PROFILES_PATH");

    // Load the paths from env.
    let vulkan_sdk = Path::new(env!("VULKAN_SDK"));
    let profiles_dir = Path::new(env!("VULKAN_PROFILES_PATH"));

    // If test/example flags are enabled, overwrite env paths.
    #[cfg(feature = "test")]
    let profiles_dir = Path::new("tests/vulkan_profiles");
    #[cfg(feature = "example")]
    let profiles_dir = Path::new("examples/vulkan_profiles");

    // Extend paths to correct file/directory.
    let profiles_source = profiles_dir.join("vulkan_profiles.cpp");
    let vulkan_include_dir = vulkan_sdk.join("Include");

    // Rerun on source change.
    println!(
        "cargo:rerun-if-changed={}",
        profiles_source.to_string_lossy()
    );

    // Setup cc build for the Vulkan Profiles Library.
    let mut build = cc::Build::new();

    build
        .file(profiles_source)
        .cpp(true)
        .std("c++17")
        .include(profiles_dir)
        .include(&vulkan_include_dir)
        .define("VP_USE_OBJECT", "1");

    #[cfg(feature = "VK_KHR_video_queue")]
    build.define("VK_KHR_video_queue", "1");

    // Platform defines
    #[cfg(feature = "VK_USE_PLATFORM_ANDROID_KHR")]
    build.define("VK_USE_PLATFORM_ANDROID_KHR", "1");
    #[cfg(feature = "VK_USE_PLATFORM_FUCHSIA")]
    build.define("VK_USE_PLATFORM_FUCHSIA", "1");
    #[cfg(feature = "VK_USE_PLATFORM_IOS_MVK")]
    build.define("VK_USE_PLATFORM_IOS_MVK", "1");
    #[cfg(feature = "VK_USE_PLATFORM_MACOS_MVK")]
    build.define("VK_USE_PLATFORM_MACOS_MVK", "1");
    #[cfg(feature = "VK_USE_PLATFORM_METAL_EXT")]
    build.define("VK_USE_PLATFORM_METAL_EXT", "1");
    #[cfg(feature = "VK_USE_PLATFORM_VI_NN")]
    build.define("VK_USE_PLATFORM_VI_NN", "1");
    #[cfg(feature = "VK_USE_PLATFORM_WAYLAND_KHR")]
    build.define("VK_USE_PLATFORM_WAYLAND_KHR", "1");
    #[cfg(feature = "VK_USE_PLATFORM_WIN32_KHR")]
    build.define("VK_USE_PLATFORM_WIN32_KHR", "1");
    #[cfg(feature = "VK_USE_PLATFORM_XCB_KHR")]
    build.define("VK_USE_PLATFORM_XCB_KHR", "1");
    #[cfg(feature = "VK_USE_PLATFORM_XLIB_KHR")]
    build.define("VK_USE_PLATFORM_XLIB_KHR", "1");
    #[cfg(feature = "VK_USE_PLATFORM_DIRECTFB_EXT")]
    build.define("VK_USE_PLATFORM_DIRECTFB_EXT", "1");
    #[cfg(feature = "VK_USE_PLATFORM_XLIB_XRANDR_EXT")]
    build.define("VK_USE_PLATFORM_XLIB_XRANDR_EXT", "1");
    #[cfg(feature = "VK_USE_PLATFORM_GGP")]
    build.define("VK_USE_PLATFORM_GGP", "1");
    #[cfg(feature = "VK_USE_PLATFORM_SCREEN_QNX")]
    build.define("VK_USE_PLATFORM_SCREEN_QNX", "1");
    #[cfg(feature = "VK_USE_PLATFORM_SCI")]
    build.define("VK_USE_PLATFORM_SCI", "1");

    // Beta define
    #[cfg(feature = "VK_ENABLE_BETA_EXTENSIONS")]
    build.define("VK_ENABLE_BETA_EXTENSIONS", "1");

    // Compile the library.
    build.compile("vulkan_profiles_ash");

    // Add 'fake' Vulkan Functions for testing.
    #[cfg(feature = "test")]
    cc::Build::new()
        .file("tests/common/mock_vulkan_api.cpp")
        .cpp(true)
        .std("c++17")
        .include(&vulkan_include_dir)
        .compile("mock_vulkan_api");
}
