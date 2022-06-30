use std::path::Path;

use image::{DynamicImage, GenericImageView};

use crate::{vec3::Vec3, vec2::Vec2};

use super::Texture;

pub struct ImageTexture {
    image: DynamicImage
}

impl ImageTexture {
    pub fn from_file(path: &Path) -> ImageTexture {
        match image::open(path) {
            Ok(image) => ImageTexture{ image },
            Err(_) => panic!("failed to open texture"),
        }
    }
}

impl Texture for ImageTexture {
    fn color(&self, mut uv: Vec2, _: Vec3) -> Vec3 {
        uv.apply(|n| n.clamp(0.0, 0.999));
        let x = (uv.x * self.image.width() as f32) as u32;
        let y = ((0.999 - uv.y) * self.image.height() as f32) as u32;
        let pixel = self.image.get_pixel(x, y).0;
        Vec3::new(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0)
    }
}