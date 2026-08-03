//! A parser for WMF (Windows Metafile) binaries and a converter to SVG,
//! conforming to the [MS-WMF] specification.
//!
//! # Usage
//!
//! ```no_run
//! let wmf_data = std::fs::read("input.wmf").expect("failed to read file");
//!
//! let svg = wmf_core::converter::convert_to_svg(wmf_data.as_slice())
//!     .expect("failed to convert");
//! ```
//!
//! Output formats other than SVG can be produced by implementing the
//! [`Player`](converter::Player) trait and passing the implementation
//! to [`convert`](converter::convert).
//!
//! # Attribution
//!
//! Portions of the API documentation in this crate are adapted from the
//! [MS-WMF] Open Specifications documentation, © Microsoft Corporation,
//! and are used under the Intellectual Property Rights Notice for Open
//! Specifications Documentation. The MS-WMF specification is covered by
//! the [Microsoft Open Specification Promise][OSP].
//!
//! [MS-WMF]: https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/4813e7fd-52d0-4f42-965f-228c8b7488d2
//! [OSP]: https://go.microsoft.com/fwlink/?LinkId=214445

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::enum_variant_names,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms,
    clippy::wildcard_imports,
    non_camel_case_types,
    non_snake_case
)]
#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(feature = "tracing")]
#[macro_use]
extern crate tracing;

#[cfg(not(feature = "tracing"))]
#[macro_use]
mod tracing {
    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)+) => {};
    }

    #[macro_export]
    macro_rules! info {
        ($($arg:tt)+) => {};
    }

    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)+) => {};
    }

    #[macro_export]
    macro_rules! error {
        ($($arg:tt)+) => {};
    }
}

pub mod converter;
pub mod parser;

mod imports {
    pub use alloc::{
        borrow::{Cow, ToOwned},
        boxed::Box,
        collections::{BTreeMap, BTreeSet, VecDeque},
        string::{String, ToString},
        vec::Vec,
    };
}

pub use embedded_io::Read;
