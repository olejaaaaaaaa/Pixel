use wgpu::ShaderModuleDescriptor;

use crate::id;

use super::{Entity, Id};


pub trait SystemShaderMesh {
    fn add_shader_mesh(&mut self);
}

impl SystemShaderMesh for Entity<'_> {
    fn add_shader_mesh(&mut self) {
        self.add_component(ComponentShaderMesh::new(self));
    }
}

pub struct ComponentShaderMesh {
    pub id: Id
}

impl ComponentShaderMesh {
    fn new(entity: &Entity) -> Self {
        let mut res = entity.game_resource.borrow_mut();
        const mesh: &str = include_str!("../shaders/mesh.wgsl");

        let shader = res.ctx.device.create_shader_module(ShaderModuleDescriptor {
            label:  None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(mesh)),
        });

        let id = id();
        res.shader.insert(id, shader);

        Self {
            id
        }

    }
}

pub trait SystemShaderMeshUniform {
    fn add_shader_mesh_uniform(&mut self);
}

impl SystemShaderMeshUniform for Entity<'_> {
    fn add_shader_mesh_uniform(&mut self) {
        self.add_component(ComponentShaderMeshUniform::new(self));
    }
}

pub struct ComponentShaderMeshUniform {
    pub id: Id
}

impl ComponentShaderMeshUniform {
    fn new(entity: &Entity) -> Self {
        let mut res = entity.game_resource.borrow_mut();
        const mesh: &str = include_str!("../shaders/mesh_uniform.wgsl");

        let shader = res.ctx.device.create_shader_module(ShaderModuleDescriptor {
            label:  None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(mesh)),
        });

        let id = id();
        res.shader.insert(id, shader);

        Self {
            id
        }

    }
}

struct ComponentCustomShader {
    id: Id
}