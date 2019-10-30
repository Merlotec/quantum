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
    wsi::winit::{
        Window,
        WindowBuilder,
        EventsLoop,
        Event,
    },
    wsi::Surface,
};
use crate::EngineFloat;
use ecs::world::World;
use std::marker::PhantomData;
use crate::render::pipe;

pub struct Renderer<F: EngineFloat, B: hal::Backend> {
    window: Window,
    events_loop: EventsLoop,
    factory: Factory<B>,
    families: Families<B>,
    graph: Graph<B, World>,
    _f: PhantomData<F>,
}

impl<F: EngineFloat, B: hal::Backend> Renderer<F, B> {
    pub fn init(window_builder: WindowBuilder, world: &World) -> Self {
        let config: Config = Default::default();

        let (mut factory, mut families): (Factory<B>, _) = rendy::factory::init(config).unwrap();

        let events_loop: EventsLoop = EventsLoop::new();
        let window: Window = window_builder.build(&events_loop).unwrap();

        let surface: Surface<B> = factory.create_surface(&window);
        let extent = unsafe { surface.extent(factory.physical()) }.expect("Failed to get surface extent from surface!");

        let mut graph_builder = GraphBuilder::<B, World>::new();


        let depth = graph_builder.create_image(
            hal::image::Kind::D2(extent.width, extent.height, 1, 1),
            1,
            hal::format::Format::D16Unorm,
            Some(hal::command::ClearValue::DepthStencil(
                hal::command::ClearDepthStencil(1.0, 0),
            )),
        );


        graph_builder.add_node(
            pipe::mesh::MeshRenderPipeline::<F, B>::builder()
                .into_subpass()
                .with_color_surface()
                .with_depth_stencil(depth)
                .into_pass()
                .with_surface(
                    surface,
                    Some(hal::command::ClearValue::Color([0.0, 0.0, 0.0, 1.0].into()))
                )
        );

        let mut graph = graph_builder
            .build(&mut factory, &mut families, &world)
            .unwrap();

        Self { window, events_loop, factory, families, graph, _f: PhantomData }
    }

    /// Renders the specified world.
    pub fn render(&mut self, world: &mut World) {
        self.factory.maintain(&mut self.families);
        self.graph.run(&mut self.factory, &mut self.families, world);
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn poll_events<C>(&mut self, callback: C)
        where C: FnMut(Event) {
        self.events_loop.poll_events(callback);
    }
}