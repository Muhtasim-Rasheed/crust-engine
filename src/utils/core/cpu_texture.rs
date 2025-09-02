use glam::U8Vec4;
use image::GenericImageView;

use crate::utils::core::GPUTexture;

pub struct CPUTexture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<U8Vec4>,
}

impl CPUTexture {
    pub fn new(width: u32, height: u32) -> Self {
        let data = vec![U8Vec4::ZERO; (width * height) as usize];
        CPUTexture {
            width,
            height,
            data,
        }
    }

    pub fn new_filled<T: Into<U8Vec4>>(width: u32, height: u32, color: T) -> Self {
        let data = vec![color.into(); (width * height) as usize];
        CPUTexture {
            width,
            height,
            data,
        }
    }

    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let image = image::open(path).map_err(|e| format!("Failed to open image: {}", e))?;
        let (width, height) = image.dimensions();
        let data = image
            .to_rgba8()
            .pixels()
            .map(|p| U8Vec4::new(p[0], p[1], p[2], p[3]))
            .collect();
        Ok(CPUTexture {
            width,
            height,
            data,
        })
    }

    pub fn load_from_bytes(bytes: &[u8], width: u32, height: u32) -> Result<Self, String> {
        if bytes.len() != (width * height * 4) as usize {
            return Err("Byte length does not match width and height".to_string());
        }
        let data = bytes
            .chunks(4)
            .map(|c| U8Vec4::new(c[0], c[1], c[2], c[3]))
            .collect();
        Ok(CPUTexture {
            width,
            height,
            data,
        })
    }

    pub fn upload_to_gpu(self) -> GPUTexture {
        GPUTexture::new(
            self.width,
            self.height,
            self.data
                .iter()
                .flat_map(|c| [c.x, c.y, c.z, c.w])
                .collect::<Vec<_>>()
                .as_slice(),
        )
    }
}
