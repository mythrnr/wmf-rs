//! Implementation of the definitions in Section 2.2.2 of the WMF
//! specifications.

mod bitmap16;
mod bitmap_core_header;
mod bitmap_info_header;
mod bitmap_v4_header;
mod bitmap_v5_header;
mod ciexyz;
mod ciexyz_triple;
mod color_ref;
mod device_independent_bitmap;
mod log_brush;
mod log_color_space;
mod log_color_space_w;
mod palette_entry;
mod pitch_and_family;
mod point_l;
mod point_s;
mod poly_polygon;
mod rect;
mod rect_l;
mod rgb_quad;
mod scan;
mod size_l;

pub use self::{
    bitmap16::*, bitmap_core_header::*, bitmap_info_header::*,
    bitmap_v4_header::*, bitmap_v5_header::*, ciexyz::*, ciexyz_triple::*,
    color_ref::*, device_independent_bitmap::*, log_brush::*,
    log_color_space::*, log_color_space_w::*, palette_entry::*,
    pitch_and_family::*, point_l::*, point_s::*, poly_polygon::*, rect::*,
    rect_l::*, rgb_quad::*, scan::*, size_l::*,
};

/// Convert f32 from Q8.8 formatted bytes.
fn f32_from_fixed_point_q_8_8(bytes: [u8; 4]) -> f32 {
    let v: [u8; 2] = bytes[1..3].try_into().expect("should be converted");
    let v = i16::from_le_bytes(v);
    f32::from(v) / (1 << 8) as f32
}

/// Convert f32 from Q2.30 formatted bytes.
fn f32_from_fixed_point_q_2_30(bytes: [u8; 4]) -> f32 {
    let v = i32::from_le_bytes(bytes);
    (v as f32) / (1 << 30) as f32
}

/// Convert UTF16-LE bytes to String.
fn utf16le_bytes_to_string(
    bytes: Vec<u8>,
) -> Result<String, crate::ParseError> {
    if bytes.len() % 2 != 0 {
        return Err(crate::ParseError::UnexpectedPattern {
            cause: "Byte array length must be even".to_owned(),
        });
    }

    let u16_vec = bytes
        .chunks_exact(2)
        .map(|chunk| {
            u16::from_le_bytes(chunk.try_into().expect("should be converted"))
        })
        .collect::<Vec<_>>();

    String::from_utf16(&u16_vec).map_err(|err| {
        crate::ParseError::UnexpectedPattern { cause: err.to_string() }
    })
}
