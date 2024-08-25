/// The CIEXYZ Object defines information about the CIEXYZ chromaticity object.
#[derive(Clone, Debug)]
pub struct CIEXYZ {
    /// ciexyzX (4 bytes): A 32-bit 2.30 fixed point type that defines the x
    /// chromaticity value.
    pub x: f32,
    /// ciexyzY (4 bytes): A 32-bit 2.30 fixed point type that defines the y
    /// chromaticity value.
    pub y: f32,
    /// ciexyzZ (4 bytes): A 32-bit 2.30 fixed point type that defines the z
    /// chromaticity value.
    pub z: f32,
}

impl CIEXYZ {
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::DEBUG, Display)
    )]
    pub fn parse<R: std::io::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::ParseError> {
        let ((x, x_bytes), (y, y_bytes), (z, z_bytes)) = (
            {
                let (v, c) = crate::read::<R, 4>(buf)?;
                (crate::objects::structure::f32_from_fixed_point_q_2_30(v), c)
            },
            {
                let (v, c) = crate::read::<R, 4>(buf)?;
                (crate::objects::structure::f32_from_fixed_point_q_2_30(v), c)
            },
            {
                let (v, c) = crate::read::<R, 4>(buf)?;
                (crate::objects::structure::f32_from_fixed_point_q_2_30(v), c)
            },
        );

        Ok((Self { x, y, z }, x_bytes + y_bytes + z_bytes))
    }
}
