use wgpu::util::BufferInitDescriptor;
use wgpu::util::DeviceExt;
use wgpu::BufferUsages;

use super::id;
use super::Id;
use super::Vertex3D;
use super::Entity;
use super::WebGPUType;

#[derive(Debug)]
pub struct ComponentMesh {
    pub vertex: Vec<Vertex3D>,
    pub vertex_buffer: Id,
    pub index_buffer:  Option<Id>,
    pub indeces:       Option<Vec<u16>>
}

impl ComponentMesh {
    fn new(entity: &Entity, vertex: Vec<Vertex3D>, indeces: Option<Vec<u16>>) -> Self {

        let mut res = entity.game_resource.borrow_mut();

        let vertex_buffer = res.ctx.device.create_buffer_init(&BufferInitDescriptor {
            label:      None,
            contents:   vertex.bytes(),
            usage:      BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        let v_id = id();
        res.vertex_buffer.insert(v_id, vertex_buffer);

        match indeces {
            Some(index) => {

                let index_buffer = res.ctx.device.create_buffer_init(&BufferInitDescriptor {
                    label:      None,
                    contents:   vertex.bytes(),
                    usage:      BufferUsages::VERTEX | BufferUsages::COPY_DST,
                });

                let indeces_id = id();
                res.index_buffer.insert(indeces_id, index_buffer);

                Self {
                    vertex,
                    vertex_buffer:  v_id,
                    index_buffer:   Some(indeces_id),
                    indeces:        Some(index),
                }
            }

            None => {
                Self {
                    vertex,
                    vertex_buffer: v_id,
                    index_buffer: None,
                    indeces: None,
                }
            }
        }
    }
}

pub trait SystemMesh {
    fn add_mesh(&mut self, vertex: Vec<Vertex3D>, indeces: Option<Vec<u16>>);
}

impl SystemMesh for Entity<'_> {
    fn add_mesh(&mut self, vertex: Vec<Vertex3D>, indexes: Option<Vec<u16>>) {
        self.add_component(ComponentMesh::new(self, vertex, indexes));
    }
}