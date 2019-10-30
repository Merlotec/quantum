use rendy::{
    shader::SpirvShader,
    hal::pso::ShaderStageFlags,
};

lazy_static::lazy_static! {
    pub static ref SPATIAL_TRANSFORM: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("../../shaders/spirv/spatial_transform.vert.spv"),
        ShaderStageFlags::VERTEX,
        "main",
    ).unwrap();

    pub static ref PBR: SpirvShader = SpirvShader::from_bytes(
        include_bytes!("../../shaders/spirv/pbr.frag.spv"),
        ShaderStageFlags::FRAGMENT,
        "main",
    ).unwrap();
}