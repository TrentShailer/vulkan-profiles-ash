#![warn(
    clippy::alloc_instead_of_core,
    clippy::use_self,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    deprecated_in_future,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications,
    clippy::unnecessary_self_imports
)]
#![allow(
    clippy::missing_safety_doc,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms
)]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub use capabilities::Capabilities;
pub use entry::Entry;

pub mod capabilities;
pub mod entry;
pub mod vp;

#[cfg(feature = "debug")]
mod enum_debug;
mod utils;
