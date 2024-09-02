use wmf_core::*;

#[derive(Clone)]
pub struct Bitmap(Vec<u8>);

impl std::fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bitmap")
            .field(&format!("[u8; {}]", self.0.len()))
            .finish()
    }
}

impl From<DeviceIndependentBitmap> for Bitmap {
    fn from(dib: DeviceIndependentBitmap) -> Self {
        let mut file_header = vec![];
        let mut info_header = vec![];
        let mut file_size: u32 = 14;

        // write info header
        match dib.dib_header_info {
            BitmapInfoHeader::Core {
                header_size,
                width,
                height,
                planes,
                bit_count,
            } => {
                file_size += header_size;
                info_header.extend(header_size.to_le_bytes());
                info_header.extend(width.to_le_bytes());
                info_header.extend(height.to_le_bytes());
                info_header.extend(planes.to_le_bytes());
                info_header.extend((bit_count as u16).to_le_bytes());
            }
            BitmapInfoHeader::Info {
                header_size,
                width,
                height,
                planes,
                bit_count,
                compression,
                image_size,
                x_pels_per_meter,
                y_pels_per_meter,
                color_used,
                color_important,
            } => {
                file_size += header_size;
                info_header.extend(header_size.to_le_bytes());
                info_header.extend(width.to_le_bytes());
                info_header.extend(height.to_le_bytes());
                info_header.extend(planes.to_le_bytes());
                info_header.extend((bit_count as u16).to_le_bytes());
                info_header.extend((compression as u16).to_le_bytes());
                info_header.extend(image_size.to_le_bytes());
                info_header.extend(x_pels_per_meter.to_le_bytes());
                info_header.extend(y_pels_per_meter.to_le_bytes());
                info_header.extend(color_used.to_le_bytes());
                info_header.extend(color_important.to_le_bytes());
            }
            BitmapInfoHeader::V4 {
                header_size,
                width,
                height,
                planes,
                bit_count,
                compression,
                image_size,
                x_pels_per_meter,
                y_pels_per_meter,
                color_used,
                color_important,
                red_mask,
                green_mask,
                blue_mask,
                alpha_mask,
                color_space_type,
                endpoints,
                gamma_red,
                gamma_green,
                gamma_blue,
            } => {
                file_size += header_size;
                info_header.extend(header_size.to_le_bytes());
                info_header.extend(width.to_le_bytes());
                info_header.extend(height.to_le_bytes());
                info_header.extend(planes.to_le_bytes());
                info_header.extend((bit_count as u16).to_le_bytes());
                info_header.extend((compression as u16).to_le_bytes());
                info_header.extend(image_size.to_le_bytes());
                info_header.extend(x_pels_per_meter.to_le_bytes());
                info_header.extend(y_pels_per_meter.to_le_bytes());
                info_header.extend(color_used.to_le_bytes());
                info_header.extend(color_important.to_le_bytes());
                info_header.extend(red_mask.to_le_bytes());
                info_header.extend(green_mask.to_le_bytes());
                info_header.extend(blue_mask.to_le_bytes());
                info_header.extend(alpha_mask.to_le_bytes());
                info_header.extend((color_space_type as u32).to_le_bytes());
                info_header.extend(endpoints.red.x.to_le_bytes());
                info_header.extend(endpoints.red.y.to_le_bytes());
                info_header.extend(endpoints.red.z.to_le_bytes());
                info_header.extend(endpoints.green.x.to_le_bytes());
                info_header.extend(endpoints.green.y.to_le_bytes());
                info_header.extend(endpoints.green.z.to_le_bytes());
                info_header.extend(endpoints.blue.x.to_le_bytes());
                info_header.extend(endpoints.blue.y.to_le_bytes());
                info_header.extend(endpoints.blue.z.to_le_bytes());
                info_header.extend(gamma_red.to_le_bytes());
                info_header.extend(gamma_green.to_le_bytes());
                info_header.extend(gamma_blue.to_le_bytes());
            }
            BitmapInfoHeader::V5 {
                header_size,
                width,
                height,
                planes,
                bit_count,
                compression,
                image_size,
                x_pels_per_meter,
                y_pels_per_meter,
                color_used,
                color_important,
                red_mask,
                green_mask,
                blue_mask,
                alpha_mask,
                color_space_type,
                endpoints,
                gamma_red,
                gamma_green,
                gamma_blue,
                intent,
                profile_data,
                profile_size,
                reserved,
            } => {
                file_size += header_size;
                info_header.extend(header_size.to_le_bytes());
                info_header.extend(width.to_le_bytes());
                info_header.extend(height.to_le_bytes());
                info_header.extend(planes.to_le_bytes());
                info_header.extend((bit_count as u16).to_le_bytes());
                info_header.extend((compression as u16).to_le_bytes());
                info_header.extend(image_size.to_le_bytes());
                info_header.extend(x_pels_per_meter.to_le_bytes());
                info_header.extend(y_pels_per_meter.to_le_bytes());
                info_header.extend(color_used.to_le_bytes());
                info_header.extend(color_important.to_le_bytes());
                info_header.extend(red_mask.to_le_bytes());
                info_header.extend(green_mask.to_le_bytes());
                info_header.extend(blue_mask.to_le_bytes());
                info_header.extend(alpha_mask.to_le_bytes());
                info_header.extend((color_space_type as u32).to_le_bytes());
                info_header.extend(endpoints.red.x.to_le_bytes());
                info_header.extend(endpoints.red.y.to_le_bytes());
                info_header.extend(endpoints.red.z.to_le_bytes());
                info_header.extend(endpoints.green.x.to_le_bytes());
                info_header.extend(endpoints.green.y.to_le_bytes());
                info_header.extend(endpoints.green.z.to_le_bytes());
                info_header.extend(endpoints.blue.x.to_le_bytes());
                info_header.extend(endpoints.blue.y.to_le_bytes());
                info_header.extend(endpoints.blue.z.to_le_bytes());
                info_header.extend(gamma_red.to_le_bytes());
                info_header.extend(gamma_green.to_le_bytes());
                info_header.extend(gamma_blue.to_le_bytes());
                info_header.extend((intent as u32).to_le_bytes());
                info_header.extend(profile_data.to_le_bytes());
                info_header.extend(profile_size.to_le_bytes());
                info_header.extend(reserved.to_le_bytes());
            }
        };

        // write color palette
        match dib.colors {
            Colors::RGBColorMask(values) => {
                values.into_iter().for_each(|(r, g, b)| {
                    info_header.extend(r.to_le_bytes());
                    info_header.extend(g.to_le_bytes());
                    info_header.extend(b.to_le_bytes());
                });
            }
            Colors::RGBQuad(values) => {
                values.into_iter().for_each(|v| {
                    info_header
                        .extend(vec![v.red, v.green, v.blue, v.reserved]);
                });
            }
            _ => {}
        }

        // write pixel data
        let data = dib.bitmap_buffer.a_data;
        file_size += data.len() as u32;

        // write file headers
        file_header.extend(vec![0x42, 0x4d]);
        file_header.extend(file_size.to_le_bytes());
        file_header.extend(vec![0x00, 0x00]);
        file_header.extend((file_size - data.len() as u32).to_le_bytes());

        let data = {
            file_header.extend(info_header);
            file_header.extend(data);
            file_header
        };

        Self(data)
    }
}

impl From<Bitmap16> for Bitmap {
    fn from(bmp: Bitmap16) -> Self {
        let Bitmap16 { width, height, planes, bits_pixel, bits, .. } = bmp;

        let mut file_header = vec![];
        let mut info_header = vec![];
        let mut file_size: u32 = 12;

        info_header.extend(file_size.to_le_bytes());
        info_header.extend(width.to_le_bytes());
        info_header.extend(height.to_le_bytes());
        info_header.extend(u16::from(planes).to_le_bytes());
        info_header.extend(u16::from(bits_pixel).to_le_bytes());

        // write pixel data
        let data = bits;
        file_size += data.len() as u32;

        // write file headers
        file_header.extend(vec![0x42, 0x4d]);
        file_header.extend(file_size.to_le_bytes());
        file_header.extend(vec![0x00, 0x00]);
        file_header.extend((file_size - data.len() as u32).to_le_bytes());

        let data = {
            file_header.extend(info_header);
            file_header.extend(data);
            file_header
        };

        Self(data)
    }
}

impl Bitmap {
    pub fn as_data_url(&self) -> String {
        use base64::{engine::general_purpose::STANDARD, Engine};
        format!("data:image/bmp;base64,{}", STANDARD.encode(&self.0))
    }
}
