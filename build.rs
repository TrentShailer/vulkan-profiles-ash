use std::env;

fn main() {
    #[cfg(feature = "linked")]
    {
        // Rerun if relevant env/files changed
        println!("cargo:rerun-if-changed=CMakeLists.txt");
        println!("cargo:rerun-if-env-changed=VULKAN_SDK");
        println!("cargo:rerun-if-env-changed=VULKAN_PROFILES_PATH");

        // Rerun if the user's vulkan_profiles file has changed.
        let profiles_path = env!("VULKAN_PROFILES_PATH");
        println!("cargo:rerun-if-changed={profiles_path}/vulkan_profiles.cpp");

        // Build the library
        let vk_profiles_library = cmake::Config::new(".").build();

        // Link the library
        println!(
            "cargo:rustc-link-search=native={}",
            vk_profiles_library.display()
        );
        println!("cargo:rustc-link-lib=static=vulkan_profiles_ash");
    }
}
