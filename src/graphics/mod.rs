use winit::dpi::PhysicalSize;
use winit::window::Window;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wgpu::*;

mod context;
pub use context::*;

mod types;
pub use types::*;

mod texture_mesh;
pub use texture_mesh::*;

mod mesh;
pub use mesh::*;

mod uniform;
pub use uniform::*;

mod shader;
pub use shader::*;

mod bind_group;
pub use bind_group::*;

mod pipeline;
pub use pipeline::*;

mod random;
pub use random::*;

mod render;
pub use render::*;

use log::warn;
type Id = i64;

pub struct Entity< 'p> {
    pub game_resource: Rc<RefCell<GameResource< 'p>>>,
    pub components: Vec<Box<dyn Any>>
}

impl Drop for Entity<'_> {
    fn drop(&mut self) {
        warn!("WARN! Enityt is drop, but resources not free! {}", self.components.len());
    }
}

impl< 'p> Entity< 'p> {

    pub fn add_component<T: 'static>(&mut self, component: T) {
        self.components.push(Box::new(component));
    }

    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        for i in &self.components {
            if let Some(x) = i.downcast_ref::<T>() {
                return Some(x);
            }
        }
        None
    }

    pub fn get_mut_component<T: 'static>(&mut self) -> Option<&mut T>{
        for i in &mut self.components {
            if let Some(x) = i.downcast_mut::<T>() {
                return Some(x);
            }
        }
        None
    }

    pub fn get_mut_components<T: 'static>(&mut self) -> Option<Vec<&mut T>>{

        let mut v: Option<Vec<&mut T>> = None;

        for i in &mut self.components {
            if let Some(x) = i.downcast_mut::<T>() {
                match &mut v {
                    None => { v = Some(vec![x]) }
                    Some(n) => { n.push(x) }
                }
            }
        }

        v
    }

    pub fn get_components<T: 'static>(&self) -> Option<Vec<&T>>{

        let mut v: Option<Vec<&T>> = None;

        for i in &self.components {
            if let Some(x) = i.downcast_ref::<T>() {
                match &mut v {
                    None => { v = Some(vec![x]) }
                    Some(n) => { n.push(x) }
                }
            }
        }

        v
    }

    pub fn remove_component<T: 'static>(&mut self) {
        for i in 0..self.components.len() {
            if let Some(x) = self.components[i].downcast_ref::<T>() {
                self.components.remove(i);
                return;
            }
        }
    }

    pub fn remove_components<T: 'static>(&mut self) {
        for i in 0..self.components.len() {
            if let Some(x) = self.components[i].downcast_ref::<T>() {
                self.components.remove(i);
            }
        }
    }

    pub fn new(game_resource: Rc<RefCell<GameResource< 'p>>>) -> Self {
        Self {
            game_resource,
            components: vec![]
        }
    }
}

pub struct GameResource<'s> {
    pub ctx:                        Rc<WebGPUContext<'s>>,
    pub render_pipeline:            HashMap<Id, RenderPipeline>,
    pub vertex_buffer:              HashMap<Id, Buffer>,
    pub index_buffer:               HashMap<Id, Buffer>,
    pub indeces:                    HashMap<Id, Vec<u16>>,
    pub bind_group:                 HashMap<Id, BindGroup>,
    pub bind_group_layout:          HashMap<Id, BindGroupLayout>,
    pub vertex_buffer_layout:       HashMap<Id, VertexBufferLayout<'static>>,
    pub uniform_buffer:             HashMap<Id, Buffer>,
    pub shader:                     HashMap<Id, ShaderModule>,
    pub texture_buffer:             HashMap<Id, Buffer>,
}

impl< 'p> GameResource< 'p> {
    async fn new(ctx: WebGPUContext<'p>, window: &'p Window) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            ctx:                        ctx.into(),
            vertex_buffer:              HashMap::new(),
            vertex_buffer_layout:       HashMap::new(),
            uniform_buffer:             HashMap::new(),
            indeces:                    HashMap::new(),
            index_buffer:               HashMap::new(),
            shader:                     HashMap::new(),
            render_pipeline:            HashMap::new(),
            bind_group:                 HashMap::new(),
            bind_group_layout:          HashMap::new(),
            texture_buffer:             HashMap::new(),
        }))
    }
}

pub struct GameWorld< 'p> {
    pub resource:   Rc<RefCell<GameResource< 'p>>>,
    pub entity:     Vec<Entity< 'p>>
}

impl< 'p> GameWorld< 'p> {
    pub async fn new(ctx: WebGPUContext<'p>, window: &'p Window) -> Self {
        Self {
            resource: GameResource::new(ctx, window).await,
            entity: vec![]
        }
    }

    pub fn create_entity(&self) -> Entity< 'p> {
        Entity::new(self.resource.clone())
    }

    pub fn resize(&self, size: PhysicalSize<u32>) {
        self.resource.borrow().ctx.resize(size);
    }
}





