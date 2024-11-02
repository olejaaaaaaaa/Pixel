use wgpu::util::BufferInitDescriptor;
use wgpu::util::DeviceExt;
use wgpu::BufferUsages;
use wgpu::ShaderStages;

use super::id;
use super::Id;
use super::{Entity, WebGPUType};

#[derive(Debug)]
pub struct ComponentUniform {
    pub buffer:     Id,
    pub visible:    ShaderStages,
}

impl ComponentUniform{
    fn new<T: WebGPUType>(entity: &Entity, vis: ShaderStages, uniform: T) -> Self {

        let mut res = entity.game_resource.borrow_mut();

        let uniform = res.ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: uniform.bytes(),
            usage: BufferUsages::UNIFORM | BufferUsages::UNIFORM
        });

        let id = id();
        res.uniform_buffer.insert(id, uniform);

        Self{
            buffer: id,
            visible: vis
        }
    }
}

pub trait SystemUniform {
    fn add_uniform<T: WebGPUType>(&mut self, vis: ShaderStages, uniform: T);
}

impl SystemUniform for Entity<'_> {
    fn add_uniform<T: WebGPUType>(&mut self, vis: ShaderStages, uniform: T) {
        self.add_component(ComponentUniform::new(self, vis, uniform));
    }
}