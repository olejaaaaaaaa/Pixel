use super::{Entity, Vertex3DTexture};

pub struct ComponentTextureMesh {
    pub vertex: Vec<Vertex3DTexture>
}

pub trait SystemTextureMesh {
    fn add_texture_mesh(&mut self);
}

impl SystemTextureMesh for Entity<'_> {
    fn add_texture_mesh(&mut self) {

    }
}