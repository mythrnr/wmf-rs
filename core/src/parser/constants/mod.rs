//! Implementation of the definitions in Section 2.1 of the WMF specifications.

mod enums;
mod flags;

pub use self::{enums::*, flags::*};

#[rustfmt::skip]
macro_rules! impl_parser {
    ($t:ident,u8) => {
        $crate::parser::constants::impl_parser!(_, $t, u8, 1, 4);
    };
    ($t:ident,u16) => {
        $crate::parser::constants::impl_parser!(_, $t, u16, 2, 6);
    };
    ($t:ident,u32) => {
        $crate::parser::constants::impl_parser!(_, $t, u32, 4, 10);
    };
    ($t:ident,i32) => {
        $crate::parser::constants::impl_parser!(_, $t, i32, 4, 10);
    };
    (_, $t:ident, $raw:ty, $size:expr, $digits:expr) => {
        paste::paste! {
            impl $t {
                #[::tracing::instrument(
                    level = tracing::Level::TRACE,
                    skip_all,
                    err(level = tracing::Level::DEBUG, Display),
                )]
                pub fn parse<R: ::std::io::Read>(
                    buf: &mut R,
                ) -> Result<(Self, usize), $crate::parser::ParseError> {
                    let (value, consumed_bytes) = crate::parser::[<read_ $raw _from_le_bytes>](buf)?;
                    let Some(v)  = Self::from_repr(value) else {
                        return Err($crate::parser::ParseError::UnexpectedEnumValue {
                            cause: ::std::format!(
                                ::core::concat!(
                                    "unexpected value as ",
                                    ::core::stringify!($t),
                                    ": {:#0", $digits, "X}",
                                ),
                                value
                            ),
                        });
                    };

                    Ok((v, consumed_bytes))
                }
            }
        }
    };
}

use impl_parser;
