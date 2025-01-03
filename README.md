# Vulkan Profiles Ash
Bindings to the [Vulkan Profiles API](https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html) in Rust for [ash](https://github.com/ash-rs/ash).

> * Bindings are written against the [Vulkan Profiles Tools included in Vulkan SDK 1.3.296](https://github.com/KhronosGroup/Vulkan-Profiles/tree/v1.3.296).
> * Currently requires statically linking the Vulkan Profiles API and ash.
> * Build environment must support all profiles used in the application.
> * Does not support the Beta Capabilities API.

## Additional Dependencies
These are required to build the vulkan profiles library:

* A C++ compiler.
    * On Windows the build may fail without [long paths enabled](https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation?tabs=registry#enable-long-paths-in-windows-10-version-1607-and-later).
* CMake.
* Vulkan SDK. Only the vulkan headers are required for the build.
* A copy of `vulkan_profiles.cpp` and `vulkan_profiles.h`. See [Generate the Vulkan Profiles C++ library](#generate-the-vulkan-profiles-c-library).

## Getting Started
### Generate the Vulkan Profiles C++ library
Vulkan Profiles Ash requires a copy of `vulkan_profiles.cpp` and `vulkan_profiles.h`. The [Vulkan Profiles documentation](https://vulkan.lunarg.com/doc/view/1.3.296.0/windows/profiles_api_library.html#building) has instructions for generating these files.

* The output directory for `--output-library-inc` must match `--output-library-src` with `/vulkan` appended.

#### Windows Example:

```powershell
python $Env:VULKAN_SDK/share/vulkan/registry/gen_profiles_solution.py `
    --registry $Env:VULKAN_SDK/share/vulkan/registry/vk.xml `
    --input examples/custom_profile/vulkan_profiles/profiles `
    --output-library-src examples/custom_profile/vulkan_profiles `
    --output-library-inc examples/custom_profile/vulkan_profiles/vulkan
```

### Configuring Vulkan Profiles Ash
Vulkan Profiles Ash requires the `VULKAN_PROFILES_PATH` environment variable to be set to the `--output-library-src` directory when generating the C++ library.

This can be done per project by writing a `.cargo/config.toml` file as shown:

```toml
[env]
VULKAN_PROFILES_PATH = { value = "--output-library-src path relative to cargo.toml", force = true, relative = true }
```

See [examples/custom_profile/.cargo/config.toml](./examples/custom_profile/.cargo/config.toml).

## Thank You

* [ash](https://github.com/ash-rs/ash)
* [vk-profiles-rs](https://github.com/CodingRays/vk-profiles-rs)
