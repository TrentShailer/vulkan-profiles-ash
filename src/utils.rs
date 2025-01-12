//! Utility functions, most of these are copied exactly from [ash](https://github.com/ash-rs/ash)
//! as they are not exported.

use alloc::vec::Vec;

use ash::{
    prelude::VkResult,
    vk::{self, CStrTooLargeForStaticArray},
};

#[inline]
pub(crate) fn write_c_str_slice_with_nul(
    target: &mut [core::ffi::c_char],
    str: &core::ffi::CStr,
) -> Result<(), CStrTooLargeForStaticArray> {
    let bytes = str.to_bytes_with_nul();
    // SAFETY: The cast from c_char to u8 is ok because a c_char is always one byte.
    let bytes = unsafe { core::slice::from_raw_parts(bytes.as_ptr().cast(), bytes.len()) };
    let static_array_size = target.len();
    target
        .get_mut(..bytes.len())
        .ok_or(CStrTooLargeForStaticArray {
            static_array_size,
            c_str_size: bytes.len(),
        })?
        .copy_from_slice(bytes);
    Ok(())
}

#[inline]
pub(crate) fn wrap_c_str_slice_until_nul(
    str: &[core::ffi::c_char],
) -> Result<&core::ffi::CStr, core::ffi::FromBytesUntilNulError> {
    // SAFETY: The cast from c_char to u8 is ok because a c_char is always one byte.
    let bytes = unsafe { core::slice::from_raw_parts(str.as_ptr().cast(), str.len()) };
    core::ffi::CStr::from_bytes_until_nul(bytes)
}

/// Repeatedly calls `f` until it does not return [`vk::Result::INCOMPLETE`] anymore, ensuring all
/// available data has been read into the vector.
///
/// See for example [`vkEnumerateInstanceExtensionProperties`]: the number of available items may
/// change between calls; [`vk::Result::INCOMPLETE`] is returned when the count increased (and the
/// vector is not large enough after querying the initial size), requiring Ash to try again.
///
/// [`vkEnumerateInstanceExtensionProperties`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html
pub(crate) unsafe fn read_into_uninitialized_vector<N: Copy + Default + TryInto<usize>, T>(
    f: impl Fn(&mut N, *mut T) -> vk::Result,
) -> VkResult<Vec<T>>
where
    <N as TryInto<usize>>::Error: core::fmt::Debug,
{
    loop {
        let mut count = N::default();
        f(&mut count, core::ptr::null_mut()).result()?;
        let mut data =
            Vec::with_capacity(count.try_into().expect("`N` failed to convert to `usize`"));

        let err_code = f(&mut count, data.as_mut_ptr());
        if err_code != vk::Result::INCOMPLETE {
            break err_code.set_vec_len_on_success(
                data,
                count.try_into().expect("`N` failed to convert to `usize`"),
            );
        }
    }
}

/// Repeatedly calls `f` until it does not return [`vk::Result::INCOMPLETE`] anymore, ensuring all
/// available data has been read into the vector.
///
/// See for example [`vkEnumerateInstanceExtensionProperties`]: the number of available items may
/// change between calls; [`vk::Result::INCOMPLETE`] is returned when the count increased (and the
/// vector is not large enough after querying the initial size), requiring Ash to try again.
///
/// [`vkEnumerateInstanceExtensionProperties`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html
pub(crate) unsafe fn read_into_uninitialized_vector_mut<N: Copy + Default + TryInto<usize>, T>(
    mut f: impl FnMut(&mut N, *mut T) -> vk::Result,
) -> VkResult<Vec<T>>
where
    <N as TryInto<usize>>::Error: core::fmt::Debug,
{
    loop {
        let mut count = N::default();
        f(&mut count, core::ptr::null_mut()).result()?;
        let mut data =
            Vec::with_capacity(count.try_into().expect("`N` failed to convert to `usize`"));

        let err_code = f(&mut count, data.as_mut_ptr());
        if err_code != vk::Result::INCOMPLETE {
            break err_code.set_vec_len_on_success(
                data,
                count.try_into().expect("`N` failed to convert to `usize`"),
            );
        }
    }
}

#[cfg(feature = "debug")]
pub(crate) fn debug_flags<Value: Into<u64> + Copy>(
    f: &mut core::fmt::Formatter<'_>,
    known: &[(Value, &'static str)],
    value: Value,
) -> core::fmt::Result {
    let mut first = true;
    let mut accum = value.into();
    for &(bit, name) in known {
        let bit = bit.into();
        if bit != 0 && accum & bit == bit {
            if !first {
                f.write_str(" | ")?;
            }
            f.write_str(name)?;
            first = false;
            accum &= !bit;
        }
    }
    if accum != 0 {
        if !first {
            f.write_str(" | ")?;
        }
        write!(f, "{accum:b}")?;
    }
    Ok(())
}
