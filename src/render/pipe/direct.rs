//!
//! The mighty triangle example.
//! This examples shows colord triangle on white background.
//! Nothing fancy. Just prove that `rendy` works.
//!

use {
    rendy::{
        command::{Families, QueueId, RenderPassEncoder},
        factory::{Config, Factory},
        graph::{render::*, Graph, GraphBuilder, GraphContext, NodeBuffer, NodeImage},
        hal,
        memory::Dynamic,
        mesh::{
            PosColor,
            AsVertex,
        },
        resource::{Buffer, BufferInfo, DescriptorSetLayout, Escape, Handle},
        shader::{ShaderSet, ShaderKind, SourceLanguage, SourceShaderInfo, SpirvShader},
    },
};

lazy_static::lazy_static! {
    static ref VERTEX: SpirvShader = SourceShaderInfo::new(
        include_str!("../../../shaders/src/direct.vert.glsl"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/src/direct.vert.glsl").into(),
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

#[derive(Debug, Default)]
struct TriangleRenderPipelineDesc;

#[derive(Debug)]
struct TriangleRenderPipeline<B: hal::Backend> {
    vertex: Option<Escape<Buffer<B>>>,
}

impl<B, T> SimpleGraphicsPipelineDesc<B, T> for TriangleRenderPipelineDesc
    where
        B: hal::Backend,
        T: ?Sized,
{
    type Pipeline = TriangleRenderPipeline<B>;

    fn depth_stencil(&self) -> Option<hal::pso::DepthStencilDesc> {
        None
    }

    fn load_shader_set(&self, factory: &mut Factory<B>, _aux: &T) -> ShaderSet<B> {
        SHADERS.build(factory, Default::default()).unwrap()
    }

    fn vertices(
        &self,
    ) -> Vec<(
        Vec<hal::pso::Element<hal::format::Format>>,
        hal::pso::ElemStride,
        hal::pso::VertexInputRate,
    )> {
        return vec![PosColor::vertex().gfx_vertex_input_desc(hal::pso::VertexInputRate::Vertex)];
    }

    fn build<'a>(
        self,
        _ctx: &GraphContext<B>,
        _factory: &mut Factory<B>,
        _queue: QueueId,
        _aux: &T,
        buffers: Vec<NodeBuffer>,
        images: Vec<NodeImage>,
        set_layouts: &[Handle<DescriptorSetLayout<B>>],
    ) -> Result<TriangleRenderPipeline<B>, failure::Error> {
        assert!(buffers.is_empty());
        assert!(images.is_empty());
        assert!(set_layouts.is_empty());

        Ok(TriangleRenderPipeline { vertex: None })
    }
}

impl<B, T> SimpleGraphicsPipeline<B, T> for TriangleRenderPipeline<B>
    where
        B: hal::Backend,
        T: ?Sized,
{
    type Desc = TriangleRenderPipelineDesc;

    fn prepare(
        &mut self,
        factory: &Factory<B>,
        _queue: QueueId,
        _set_layouts: &[Handle<DescriptorSetLayout<B>>],
        _index: usize,
        _aux: &T,
    ) -> PrepareResult {
        if self.vertex.is_none() {
            #[cfg(feature = "spirv-reflection")]
                let vbuf_size = SHADER_REFLECTION.attributes_range(..).unwrap().stride as u64 * 3;

            #[cfg(not(feature = "spirv-reflection"))]
                let vbuf_size = PosColor::vertex().stride as u64 * 3;

            let mut vbuf = factory
                .create_buffer(
                    BufferInfo {
                        size: vbuf_size,
                        usage: hal::buffer::Usage::VERTEX,
                    },
                    Dynamic,
                )
                .unwrap();

            unsafe {
                // Fresh buffer.
                factory
                    .upload_visible_buffer(
                        &mut vbuf,
                        0,
                        &[
                            PosColor {
                                position: [0.0, -0.5, 0.0].into(),
                                color: [1.0, 0.0, 0.0, 1.0].into(),
                            },
                            PosColor {
                                position: [0.5, 0.5, 0.0].into(),
                                color: [0.0, 1.0, 0.0, 1.0].into(),
                            },
                            PosColor {
                                position: [-0.5, 0.5, 0.0].into(),
                                color: [0.0, 0.0, 1.0, 1.0].into(),
                            },
                        ],
                    )
                    .unwrap();
            }

            self.vertex = Some(vbuf);
        }

        PrepareResult::DrawReuse
    }

    fn draw(
        &mut self,
        _layout: &B::PipelineLayout,
        mut encoder: RenderPassEncoder<'_, B>,
        _index: usize,
        _aux: &T,
    ) {
        let vbuf = self.vertex.as_ref().unwrap();
        unsafe {
            encoder.bind_vertex_buffers(0, Some((vbuf.raw(), 0)));
            encoder.draw(0..3, 0..1);
        }
    }

    fn dispose(self, _factory: &mut Factory<B>, _aux: &T) {}
}