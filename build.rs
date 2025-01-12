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
