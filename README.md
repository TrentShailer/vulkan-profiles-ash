# Vulkan Profiles Ash (`vp-ash`)
Bindings to the [Vulkan Profiles API](https://vulkan.lunarg.com/doc/view/1.4.304.0/windows/profiles_api_library.html) 
in Rust for [ash](https://github.com/ash-rs/ash). Using the beta Capabilities API.

> [!CAUTION] 
> ### This is a personal project
>
> Maintanance, bug fixes, new features, and support will only be provided when/if I feel like it.
>

## Restrictions:
* Bindings are written against the [Vulkan Profiles Tools included in Vulkan SDK 1.3.296](https://github.com/KhronosGroup/Vulkan-Profiles/tree/v1.3.296).
* Requires statically linking the Vulkan Profiles API and Vulkan.
* Uses the Beta Capabilities API for Vulkan Profiles API.
* The `--output-library-inc` must be the `--output-library-src + /vulkan`.

## Additional Dependencies
* A C++ compiler.
    * On Windows the build may fail without [long paths enabled](https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation?tabs=registry#enable-long-paths-in-windows-10-version-1607-and-later).
* The Vulkan SDK.

## Configuring `vp-ash`
Use the `VULKAN_PROFILES_PATH` environment variable set to the `--output-library-src` directory when generating the Vulkan Profiles library.

This can be done per project by writing a `.cargo/config.toml` file as shown:

```toml
[env]
VULKAN_PROFILES_PATH = { value = "path/to/output-library-src/from/cargo.toml", force = true, relative = true }
```

## Usage
See [the Compute example](./examples/compute.rs).

## Development

- Run Tests: `cargo test --features "test,VK_KHR_video_queue"`
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
