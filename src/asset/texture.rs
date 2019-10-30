use nalgebra::Vector2;

use rendy::texture::pixel::Rgba8Srgb;

#[derive(Debug)]
pub struct TextureAsset {
    data: Vec<Rgba8Srgb>,
    dimensions: Vector2<u32>,
}

impl TextureAsset {
    pub fn from_file(path: &str) -> Result<Self, &'static str> {
        if let Ok(image) = image::open(path) {
            let img = image.to_rgba();
            let (width, height) = img.dimensions();
            let data: Vec<Rgba8Srgb> = img
                .pixels()
                .map(|p| Rgba8Srgb { repr: p.0 })
                .collect::<_>();
            return Ok(Self { data, dimensions: Vector2::new(width, height) });
        }
        return Err("Could not find a valid image file at the path specified.");

    }

    pub fn from_image_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if let Ok(image) = image::load_from_memory(bytes) {
            let img = image.to_rgba();
            let (width, height) = img.dimensions();
            let data: Vec<Rgba8Srgb> = img
                .pixels()
                .map(|p| Rgba8Srgb { repr: p.0 })
                .collect::<_>();
            return Ok(Self { data, dimensions: Vector2::new(width, height) });
        }
        return Err("Failed to load texture from bytes. Perhaps the bytes were of invalid format?");
    }

    pub fn data(&self) -> &[Rgba8Srgb] {
        self.data.as_slice()
    }

    pub fn dimensions(&self) -> &Vector2<u32> {
        &self.dimensions
    }
}