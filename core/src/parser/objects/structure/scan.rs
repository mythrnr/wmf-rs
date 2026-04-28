use crate::imports::*;

/// The Scan Object specifies a collection of scanlines.
#[derive(Clone, Debug)]
#[allow(clippy::struct_field_names)]
pub struct Scan {
    /// Count (2 bytes): A 16-bit unsigned integer that specifies the number of
    /// horizontal (x-axis) coordinates in the ScanLines array. This value MUST
    /// be a multiple of 2, since left and right endpoints are required to
    /// specify each scanline.
    pub count: u16,
    /// Top (2 bytes): A 16-bit unsigned integer that defines the vertical
    /// (y-axis) coordinate, in logical units, of the top scanline.
    pub top: u16,
    /// Bottom (2 bytes): A 16-bit unsigned integer that defines the vertical
    /// (y-axis) coordinate, in logical units, of the bottom scanline.
    pub bottom: u16,
    /// ScanLines (variable): An array of scanlines, each specified by left and
    /// right horizontal (x-axis) coordinates of its endpoints.
    pub scan_lines: Vec<ScanLine>,
    /// Count2 (2 bytes): A 16-bit unsigned integer that MUST be the same as
    /// the value of the Count field; it is present to allow upward travel in
    /// the structure.
    pub count2: u16,
}

impl Scan {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let count = read_field(buf, &mut consumed_bytes)?;
        let top = read_field(buf, &mut consumed_bytes)?;
        let bottom = read_field(buf, &mut consumed_bytes)?;

        crate::parser::ParseError::expect_eq("count (parity)", count % 2, 0)?;

        let line_count = count as usize / 2;
        let mut scan_lines = Vec::with_capacity(line_count);

        for _ in 0..line_count {
            let v = read_with(buf, &mut consumed_bytes, ScanLine::parse)?;
            scan_lines.push(v);
        }

        let count2: u16 = read_field(buf, &mut consumed_bytes)?;

        crate::parser::ParseError::expect_eq("count2", count2, count)?;

        Ok((Self { count, top, bottom, scan_lines, count2 }, consumed_bytes))
    }
}

#[derive(Clone, Debug)]
pub struct ScanLine {
    /// Left (2 bytes): A 16-bit unsigned integer that defines the horizontal
    /// (x-axis) coordinate, in logical units, of the left endpoint of the
    /// scanline.
    pub left: u16,
    /// Right (2 bytes): A 16-bit unsigned integer that defines the horizontal
    /// (x-axis) coordinate, in logical units, of the right endpoint of the
    /// scanline.
    pub right: u16,
}

impl ScanLine {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::read_field;

        let mut consumed_bytes: usize = 0;
        let left = read_field(buf, &mut consumed_bytes)?;
        let right = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { left, right }, consumed_bytes))
    }
}
