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

pub mod converter;
pub mod parser;

mod imports {
    pub use alloc::{
        borrow::ToOwned,
        boxed::Box,
        collections::{BTreeMap, BTreeSet, VecDeque},
        string::{String, ToString},
        vec::Vec,
    };
}

pub use embedded_io::Read;
