# Vulkan Profiles Ash (`vp-ash`)
Bindings to the [Vulkan Profiles API](https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html) in Rust for [ash](https://github.com/ash-rs/ash). Using the beta Capabilities API.

> ## Restrictions:
> * Bindings are written against the [Vulkan Profiles Tools included in Vulkan SDK 1.3.296](https://github.com/KhronosGroup/Vulkan-Profiles/tree/v1.3.296).
> * Currently requires statically linking the Vulkan Profiles API and Vulkan.
> * Only uses the Beta Capabilities API for Vulkan Profiles API.

## Additional Dependencies
These are required to build the Vulkan Profiles Library:

* A C++ compiler.
    * On Windows the build may fail without [long paths enabled](https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation?tabs=registry#enable-long-paths-in-windows-10-version-1607-and-later).
* The Vulkan SDK.

## Generate the Vulkan Profiles C++ library
`vp-ash` requires a copy of `vulkan_profiles.cpp` and `vulkan_profiles.h`. The [Vulkan Profiles documentation](https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#building) has instructions for generating these files.

> * The output directory for `--output-library-inc` must match `--output-library-src` with `/vulkan` appended.

### Powershell Example:

```powershell
python $Env:VULKAN_SDK/share/vulkan/registry/gen_profiles_solution.py `
    --registry $Env:VULKAN_SDK/share/vulkan/registry/vk.xml `
    --input examples/vulkan_profiles/profiles `
    --output-library-src examples/vulkan_profiles `
    --output-library-inc examples/vulkan_profiles/vulkan
```

## Configuring `vp-ash`
`vp-ash` requires the `VULKAN_PROFILES_PATH` environment variable to be set to the `--output-library-src` directory when generating the C++ library.

This can be done per project by writing a `.cargo/config.toml` file as shown:

```toml
[env]
VULKAN_PROFILES_PATH = { value = "path/to/output-library-src/from/cargo.toml", force = true, relative = true }
```

## Usage

See [the Compute example](./examples/compute.rs) for more detailed usage.

```rs
// ----- Setup objects.
// Load Vulkan functions.
let entry = ash::Entry::linked();

// Load Vulkan Profiles functions.
let vp_entry = vp_ash::Entry::linked();

// Create capabilities (similar to ash::Instance/ash::Device).
let capabilities = {
    let create_info = vp::CapabilitiesCreateInfo::default()
        .api_version(vk::make_api_version(0, 1, 2, 198))
        .flags(vp::CapabilitiesCreateFlags::STATIC);

    unsafe { vp_entry.create_capabilities(&create_info, None) }.unwrap()
};

// Profiles for this application.
// See examples/vulkan_profiles/profiles/VP_VPA_examples_compute.json
let core_profile = vp::ProfileProperties::default()
    .profile_name(c"VP_VPA_examples_compute")
    .unwrap()
    .spec_version(1);

// Sanity check that profiles are present, if the instance in the build environment is missing
// the required extensions, this will fail.
{
    let profiles = unsafe { capabilities.get_profiles() }.unwrap();

    assert!(
        profiles.contains(&core_profile),
        "The build environment does not support the core profile."
    );
};

// Check for instance support.
let supports_instance =
    unsafe { capabilities.get_instance_profile_support(None, &core_profile) }.unwrap();
if !supports_instance {
    panic!("Your Vulkan Instance does not meet the requirements to run this application. Try updating your drivers.")
}

// Create the list of profiles to use.
let enabled_profiles = [core_profile];

// Create instance.
let instance = {
    let api_version = unsafe { capabilities.get_profile_api_version(&core_profile) };

    let app_info = vk::ApplicationInfo::default()
        .api_version(api_version)
        .application_name(c"VPA Compute Example");

    let vk_create_info = vk::InstanceCreateInfo::default().application_info(&app_info);

    let vp_create_info = vp::InstanceCreateInfo::default()
        .create_info(&vk_create_info)
        .enabled_full_profiles(&enabled_profiles);

    unsafe { capabilities.create_instance(&entry, &vp_create_info, None) }.unwrap()
};

// Select a physical device.
let physical_device = {
    unsafe { instance.enumerate_physical_devices() }
        .unwrap()
        .into_iter()
        .filter(|&device| unsafe {
            let supported = capabilities
                .get_physical_device_profile_support(&instance, device, &core_profile)
                .unwrap();
            if ! supported{
                return  false;
            }

            let queue_properties =
                instance.get_physical_device_queue_family_properties(device);

            queue_properties
                .into_iter()
                .any(| properties| {
                    properties.queue_count >= 1
                            && properties.queue_flags.contains(vk::QueueFlags::COMPUTE)
                })
        }).min_by_key(|&device| {
            let properties = unsafe {instance.get_physical_device_properties(device)};

            match properties.device_type {
                vk::PhysicalDeviceType::DISCRETE_GPU => 0,
                vk::PhysicalDeviceType::INTEGRATED_GPU => 1,
                vk::PhysicalDeviceType::VIRTUAL_GPU => 2,
                vk::PhysicalDeviceType::CPU => 3,
                vk::PhysicalDeviceType::OTHER => 4,
                _ => 5,
            }
        }).expect("No GPU in your system meets the requirements to run this application. Try updating your drivers.")
};

// Get the queue family index.
let queue_family_index = {
    let queue_properties =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    queue_properties
        .into_iter()
        .position(|properties| {
            properties.queue_count >= 1
                && properties.queue_flags.contains(vk::QueueFlags::COMPUTE)
        })
        .unwrap() as u32
};

// Create logical device.
let device = {
    let queue_create_infos = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family_index)
        .queue_priorities(&[1.0; 1])];

    let vk_create_info =
        vk::DeviceCreateInfo::default().queue_create_infos(&queue_create_infos);

    let vp_create_info = vp::DeviceCreateInfo::default()
        .create_info(&vk_create_info)
        .enabled_full_profiles(&enabled_profiles);

    unsafe { capabilities.create_device(&instance, physical_device, &vp_create_info, None) }
        .unwrap()
};
```

## `vp-ash` Development

- Run Tests: `cargo test --features test`
- Run Examples: `cargo run --example <example> --features example`
- Compiling Test Profiles:
    ```powershell
    python $Env:VULKAN_SDK/share/vulkan/registry/gen_profiles_solution.py `
        --registry $Env:VULKAN_SDK/share/vulkan/registry/vk.xml `
        --input tests/vulkan_profiles/profiles `
        --output-library-src tests/vulkan_profiles `
        --output-library-inc tests/vulkan_profiles/vulkan
    ```
- Compiling Example Profiles:
    ```powershell
    python $Env:VULKAN_SDK/share/vulkan/registry/gen_profiles_solution.py `
        --registry $Env:VULKAN_SDK/share/vulkan/registry/vk.xml `
        --input examples/vulkan_profiles/profiles `
        --output-library-src examples/vulkan_profiles `
        --output-library-inc examples/vulkan_profiles/vulkan
    ```

## Thank You

* [ash](https://github.com/ash-rs/ash)
* [vk-profiles-rs](https://github.com/CodingRays/vk-profiles-rs)
* [Vulkan-Profiles](https://github.com/KhronosGroup/Vulkan-Profiles)
