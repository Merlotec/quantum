use rendy::{
    factory::{
        Config,
        Factory,
    },
    command::Families,
    graph::{
        GraphBuilder,
        Graph,
        present::PresentNode,
        render::{
            SimpleGraphicsPipeline,
            RenderGroupBuilder,
        },
    },
    hal,
    wsi::winit::Window,
    wsi::Surface,
};
use crate::EngineFloat;
use ecs::world::World;
use std::marker::PhantomData;
use crate::render::pipe;

pub struct Renderer<F: EngineFloat, B: hal::Backend> {
    factory: Factory<B>,
    families: Families<B>,
    frames: usize,
    graph: Graph<B, World>,
    _f: PhantomData<F>,
}

impl<F: EngineFloat, B: hal::Backend> Renderer<F, B> {
    pub fn init(window: &Window, world: &World) -> Self {
        let config: Config = Default::default();

        let (mut factory, mut families): (Factory<B>, _) = rendy::factory::init(config).unwrap();

        let surface: Surface<B> = factory.create_surface(window.into());
        let extent = unsafe { surface.extent(factory.physical()) }.expect("Failed to get surface extent from surface!");

        let mut graph_builder = GraphBuilder::<B, World>::new();

        let color = graph_builder.create_image(
            hal::image::Kind::D2(extent.width, extent.height, 1, 1),
            1,
            factory.get_surface_format(&surface),
            Some(hal::command::ClearValue::Color(
                [1.0, 1.0, 1.0, 1.0].into(),
            )),
        );

        let depth = graph_builder.create_image(
            hal::image::Kind::D2(extent.width, extent.height, 1, 1),
            1,
            hal::format::Format::D16Unorm,
            Some(hal::command::ClearValue::DepthStencil(
                hal::command::ClearDepthStencil(1.0, 0),
            )),
        );

        let pass = graph_builder.add_node(
            pipe::mesh::MeshRenderPipeline::<F, B>::builder()
                .into_subpass()
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        let present_builder = PresentNode::builder(&factory, surface, color).with_dependency(pass);

        let frames = present_builder.image_count() as usize;

        graph_builder.add_node(present_builder);

        let mut graph = graph_builder
            .build(&mut factory, &mut families, &world)
            .unwrap();

        Self { factory, families, frames, graph, _f: PhantomData }
    }
}