use rendy::texture::Texture as GpuTexture;
use rendy::hal;

use crate::asset::texture::TextureAsset;

/// The material
#[derive(Debug, Default)]
pub struct Material<B: hal::Backend> {
    texture: Option<TextureAsset>,
    pub(crate) gpu_texture: Option<GpuTexture<B>>,
    pub(crate) descriptor_set: Option<B::DescriptorSet>,
}

impl<B: hal::Backend> Material<B> {
    pub fn texture(&self) -> Option<&TextureAsset> {
        self.texture.as_ref()
    }

    pub fn set_texture(&mut self, texture: Option<TextureAsset>) {
        self.texture = texture;
        // Invalidate existing gpu buffer.
        self.gpu_texture = None;
    }
}