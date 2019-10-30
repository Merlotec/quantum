use ecs::{
    world::{
        Universe,
        World,
    },
    system::StageExecutor
};

use rendy::{
    hal,
    wsi::winit::{
        Window,
        WindowBuilder,
        EventsLoop,
        Event,
        WindowEvent,
    },
};

use std::time::SystemTime;

use crate::{
    EngineFloat,
    render::Renderer,
};
use std::alloc::System;
use crate::ecs::system::Resources;
use std::collections::HashMap;

pub enum UpdateResult {
    Exit,
    Continue,
}

pub struct QuantumCore<F: EngineFloat, B: hal::Backend> {
    universe: Universe,
    world: World,
    resources: Resources,
    executor: StageExecutor,

    renderer: Renderer<F, B>,

    timestamp: Option<SystemTime>,
}

impl<F: EngineFloat, B: hal::Backend> QuantumCore<F, B> {
    pub fn new(window_builder: WindowBuilder) -> Self {
        let universe: Universe = Universe::new();
        let world: World = universe.create_world();
        let resources: Resources = Resources::new();
        let executor: StageExecutor = StageExecutor::new(None);
        let renderer: Renderer<F, B> = Renderer::init(window_builder, &world);
        Self { universe, world, resources, executor, renderer, timestamp: None }
    }

    pub fn update(&mut self) -> UpdateResult {
        let now: SystemTime = SystemTime::now();
        let delta: F = {
            match self.timestamp {
                Some(time) => {
                    if let Ok(dur) = now.duration_since(time) {
                        F::from(dur.as_secs_f32())
                    } else {
                        F::zero()
                    }
                },
                None => F::zero(),
            }
        };
        self.timestamp = Some(now);

        let mut should_close: bool = false;

        self.renderer.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => should_close = true,
            _ => (),
        });
        if should_close {
            return UpdateResult::Exit;
        }
        self.executor.execute(&self.resources, &mut self.world);
        self.renderer.render(&mut self.world);
        UpdateResult::Continue
    }
}