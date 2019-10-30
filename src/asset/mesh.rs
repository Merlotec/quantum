use rendy::{
    hal,
    factory::Factory,
};

use crate::{
    asset::mesh::*,
    render::{
        pipe::mesh::GpuMesh,
        buffer,
    },
    EngineFloat,
};
use std::mem;
use nalgebra::{
    Vector3,
    Vector2,
};
use crate::EngineFloat;

#[derive(Debug, Copy, Clone)]
pub struct Vertex<F: EngineFloat> {
    pub pos: Vector3<F>,
    pub norm: Vector3<F>,
    pub tex: Vector2<F>,
}

impl<F: EngineFloat> Vertex<F> {
    pub fn format() -> (Vec<hal::pso::Element<hal::format::Format>>, hal::pso::ElemStride, hal::pso::VertexInputRate) {
        let mut elements: Vec<hal::pso::Element<hal::format::Format>> = Vec::with_capacity(3);

        let mut offset: u32 = 0;
        elements.push(hal::pso::Element { format: F::vector_format(3), offset });
        offset += mem::size_of::<Vector3<F>>() as u32;
        elements.push(hal::pso::Element { format: F::vector_format(3), offset });
        offset += mem::size_of::<Vector3<F>>() as u32;
        elements.push(hal::pso::Element { format: F::vector_format(2), offset });

        (
            elements,
            mem::size_of::<Self>() as u32,
            // TODO: Check
            hal::pso::VertexInputRate::Vertex,
        )
    }
}

#[derive(Debug, Default)]
pub struct MeshAsset<F: EngineFloat> {
    vertices: Vec<Vertex<F>>,
    indices: Vec<u32>,
}

impl<F: EngineFloat> MeshAsset<F> {
    pub fn vertices(&self) -> &[Vertex<F>] {
        self.vertices.as_slice()
    }

    pub fn indices(&self) -> &[u32] {
        self.indices.as_slice()
    }

    pub(crate) fn upload<B: hal::Backend>(&self, factory: &Factory<B>) -> Result<GpuMesh<B>, failure::Error> {
        let vertex_buffer = buffer::alloc_simple::<B, Vertex<F>>(
            &factory,
            hal::buffer::Usage::VERTEX,
            mesh.asset().vertices()
        )?;

        let index_buffer = buffer::alloc_simple::<B, u32>(
            &factory,
            hal::buffer::Usage::VERTEX,
            mesh.asset().indices()
        )?;

        Ok(GpuMesh { vertex_buffer, index_buffer, vertex_count: mesh.asset().vertices().len(), index_count: mesh.asset().indices().len() })
    }
}