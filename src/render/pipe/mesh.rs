use rendy::{
    command::{DrawIndexedCommand, QueueId, RenderPassEncoder},
    factory::{Config, Factory, ImageState},
    graph::{present::PresentNode, render::*, GraphBuilder, GraphContext, NodeBuffer, NodeImage},
    hal::{self, pso::DescriptorPool, Device},
    memory::MemoryUsageValue,
    mesh::{AsVertex, PosNormTex},
    resource::{
        Buffer
    },
    shader::{Shader, ShaderSet, ShaderSetBuilder, ShaderKind, SourceLanguage, SourceShaderInfo, StaticShaderInfo, SpirvShader, SpecConstantSet},
    texture::{pixel::Rgba8Srgb, Texture, TextureBuilder},
};

use rendy::{
    command::Families,
    graph::{render::*, Graph},
    memory::Dynamic,
    mesh::PosColor,
    resource::{BufferInfo, DescriptorSetLayout, Escape, Handle},
};

use nalgebra::{RealField, Matrix4, Transform3};

use ecs::{
    query::{
        Read,
        Write,
        IntoQuery,
    },
    world::World,
};

use crate::{
    component::{
        Mesh,
        Material,
    },
    asset::{
        TextureAsset,
        MeshAsset,
        mesh::Vertex,
    },
    EngineFloat,
};

use crate::render::shader;

use std::slice;
use std::mem;
use std::marker::PhantomData;
use std::alloc::alloc;

const MAX_MATERIAL_DESC: usize = 100;
const MAX_TEXTURE_DESC: usize = 100;

lazy_static::lazy_static! {
    static ref VERTEX: SpirvShader = SourceShaderInfo::new(
        include_str!("../../../shaders/src/spatial_transform.vert.glsl"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/src/spatial_transform.vert.glsl").into(),
        ShaderKind::Vertex,
        SourceLanguage::GLSL,
        "main",
    ).precompile().unwrap();

    static ref FRAGMENT: SpirvShader = SourceShaderInfo::new(
        include_str!("../../../shaders/src/direct.frag.glsl"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/src/direct.frag.glsl").into(),
        ShaderKind::Fragment,
        SourceLanguage::GLSL,
        "main",
    ).precompile().unwrap();

    static ref SHADERS: rendy::shader::ShaderSetBuilder = rendy::shader::ShaderSetBuilder::default()
        .with_vertex(&*VERTEX).unwrap()
        .with_fragment(&*FRAGMENT).unwrap();
}

pub struct GpuMesh<B: hal::Backend> {
    vertex_buffer: Escape<Buffer<B>>,
    index_buffer: Escape<Buffer<B>>,

    vertex_count: usize,
    index_count: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct MeshRenderPipelineDesc<F: EngineFloat>(PhantomData<F>);

impl<F: EngineFloat> Default for MeshRenderPipelineDesc<F> {
    fn default() -> Self {
        MeshRenderPipelineDesc(PhantomData)
    }
}

impl<F: EngineFloat, B: hal::Backend> SimpleGraphicsPipelineDesc<B, World> for MeshRenderPipelineDesc<F> {

    type Pipeline = MeshRenderPipeline<F, B>;


    fn layout(&self) -> Layout {

        Layout {
            sets: vec![SetLayout {
                bindings: vec![
                    hal::pso::DescriptorSetLayoutBinding {
                        binding: 0,
                        ty: hal::pso::DescriptorType::CombinedImageSampler,
                        count: 1,
                        stage_flags: hal::pso::ShaderStageFlags::FRAGMENT,
                        immutable_samplers: false,
                    },
                ],
            }],
            push_constants: vec![(hal::pso::ShaderStageFlags::VERTEX, 0..(mem::size_of::<Matrix4<F>>() / mem::size_of::<u32>()) as u32) ],
        }
    }


    fn vertices(&self, ) -> Vec<(
        Vec<hal::pso::Element<hal::format::Format>>,
        hal::pso::ElemStride,
        hal::pso::VertexInputRate,
    )> {
//        vec![
//            Vertex::<F>::format()
//        ]
        vec![Vertex::<F>::format()]
    }

    fn load_shader_set(&self, factory: &mut Factory<B>, _: &World) -> ShaderSet<B> {
        SHADERS.build(factory, Default::default()).unwrap()
    }

    fn build<'a>(self, _ctx: &GraphContext<B>, factory: &mut Factory<B>, queue: QueueId, world: &World, buffers: Vec<NodeBuffer>, images: Vec<NodeImage>, set_layouts: &[Handle<DescriptorSetLayout<B>>]) -> Result<MeshRenderPipeline<F, B>, failure::Error> {
        // Allocate descriptor pool with max counts.

        let mut descriptor_pool = unsafe {
            factory.create_descriptor_pool(
                1,
                vec![
                    hal::pso::DescriptorRangeDesc {
                        ty: hal::pso::DescriptorType::CombinedImageSampler,
                        count: MAX_TEXTURE_DESC,
                    },
                ],
                hal::pso::DescriptorPoolCreateFlags::empty(),
            )
        }?;

        Ok(MeshRenderPipeline {
            descriptor_pool,
            _f: PhantomData
        })
    }
}

#[derive(Debug)]
pub struct MeshRenderPipeline<F: RealField, B: hal::Backend> {
    descriptor_pool: B::DescriptorPool,
    _f: PhantomData<F>,
}


impl<F: EngineFloat, B: hal::Backend> SimpleGraphicsPipeline<B, World> for MeshRenderPipeline<F, B> {

    type Desc = MeshRenderPipelineDesc<F>;

    fn prepare(&mut self, factory: &Factory<B>, queue: QueueId, set_layouts: &[Handle<DescriptorSetLayout<B>>], index: usize, world: &World) -> PrepareResult {
        // TODO: Optimize to use tags or something to reduce iter time for initialized objects.
        let mut query = <(Write<Mesh<F, B>>, Write<Material<B>>)>::query();
        for (mut mesh, mut material) in query.iter(&world) {

            // Allocate texture buffers.
            if let Some(tex) = material.texture().as_ref() {

                /*
                if material.gpu_texture.is_none() {
                    let tex_builder = TextureBuilder::new()
                        .with_kind(hal::image::Kind::D2(tex.dimensions().x, tex.dimensions().y, 1, 1))
                        .with_view_kind(hal::image::ViewKind::D2)
                        .with_data_width(tex.dimensions().x)
                        .with_data_height(tex.dimensions().y)
                        .with_data(tex.data());
                    let gpu_texture = tex_builder
                        .build(
                            ImageState {
                                queue,
                                stage: hal::pso::PipelineStage::FRAGMENT_SHADER,
                                access: hal::image::Access::SHADER_READ,
                                layout: hal::image::Layout::ShaderReadOnlyOptimal,
                            },
                            factory,
                        )
                        .expect("Failed to create gpu texture!");
                    material.gpu_texture = Some(gpu_texture);
                }
                */
                // Allocate material descriptor sets.
                if material.descriptor_set.is_none() {
                    if let Some(gpu_texture) = material.gpu_texture.as_ref() {
                        unsafe {
                            let set = self.descriptor_pool.allocate_set(&set_layouts[0].raw()).unwrap();
                            factory.write_descriptor_sets(vec![
                                hal::pso::DescriptorSetWrite {
                                    set: &set,
                                    binding: 0,
                                    array_offset: 0,
                                    descriptors: Some(hal::pso::Descriptor::CombinedImageSampler(
                                        gpu_texture.view().raw(),
                                        hal::image::Layout::ShaderReadOnlyOptimal,
                                        gpu_texture.sampler().raw(),
                                    )),
                                },
                            ]);
                            material.descriptor_set = Some(set);
                        }
                    }
                }
            }
        }
        PrepareResult::DrawRecord
    }

    fn draw(&mut self, layout: &<B as hal::Backend>::PipelineLayout, mut encoder: RenderPassEncoder<'_, B>, index: usize, world: &World) {
        let mut query = <(Read<Transform3<F>>, Write<Mesh<F, B>>, Write<Material<B>>)>::query();
        for (transform, mut mesh, mut material) in query.iter(&world) {
            if let Some(gpu_mesh) = mesh.gpu_data.as_ref() {

                if let Some(set) = material.descriptor_set.as_ref() {
                    // RENDERING CODE
                    unsafe {
                        encoder.bind_graphics_descriptor_sets(
                            layout,
                            0,
                            Some(set),
                            std::iter::empty(),
                        );
                        let push_constant_data: &[u32] = slice::from_raw_parts(transform.matrix() as *const Matrix4<F> as *const u32, mem::size_of::<Matrix4<F>>() / mem::size_of::<u32>());
                        encoder.push_constants(layout, hal::pso::ShaderStageFlags::VERTEX, 0, push_constant_data);
                        encoder.bind_vertex_buffers(
                            0,
                            std::iter::once((gpu_mesh.vertex_buffer.raw(), gpu_mesh.vertex_count as u64)),
                        );
                        encoder.draw_indexed_indirect(
                            gpu_mesh.index_buffer.raw(),
                            0,
                            gpu_mesh.index_count as u32,
                            mem::size_of::<u32>() as u32,
                        );
                    }
                }
            }
        }
    }

    fn dispose(self, factory: &mut Factory<B>, world: &World) {

    }
}