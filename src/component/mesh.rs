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

use nalgebra::{
    Vector3,
    Vector2,
};

use ecs::{
    world::World,
    system::System,
};

pub struct Mesh<F: EngineFloat, B: hal::Backend> {
    /// The (triangulated) vertices of the mesh.
    asset: MeshAsset<F>,

    /// The gpu buffer which stores mesh data in the vram.
    pub(crate) gpu_data: Option<GpuMesh<B>>,
}

impl<F: EngineFloat, B: hal::Backend> Mesh<F, B> {
    pub fn asset(&self) -> &MeshAsset<F> {
        &self.asset
    }

    pub fn gpu_data(&self) -> Option<&GpuMesh<B>> {
        self.gpu_data.as_ref()
    }
}

pub(crate) fn systems() -> Vec<System> {
    System::new();
}