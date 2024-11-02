#![allow(warnings)]

use wgpu::VertexAttribute;
use wgpu::VertexBufferLayout;
use wgpu::VertexStepMode;
use bytemuck::{Pod, Zeroable};


#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex3D {
    pub pos:   [f32; 3],
    pub color: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex3DTexture {
    pub pos:        [f32; 3],
    pub tex_pos:    [f32; 3],
}

pub trait WebGPUType {
    fn layout(&self) -> VertexBufferLayout<'static>;
    fn bytes(&self)  -> &[u8];
}

impl WebGPUType for Vec<Vertex3D> {

    fn bytes(&self)  -> &[u8] {
        bytemuck::cast_slice(self)
    }

    fn layout(&self) -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride:   24,
            step_mode:      VertexStepMode::Vertex,
            attributes:     &[

                VertexAttribute {
                    format:             wgpu::VertexFormat::Float32x3,
                    offset:             0 as u64,
                    shader_location:    0,
                },

                VertexAttribute {
                    format:             wgpu::VertexFormat::Float32x3,
                    offset:             12,
                    shader_location:    1,
                }

            ],
        }
    }
}

impl WebGPUType for Vec<Vertex3DTexture> {

    fn bytes(&self)  -> &[u8] {
        bytemuck::cast_slice(self)
    }

    fn layout(&self) -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride:   24,
            step_mode:      VertexStepMode::Vertex,
            attributes:     &[

                VertexAttribute {
                    format:             wgpu::VertexFormat::Float32x3,
                    offset:             0 as u64,
                    shader_location:    0,
                },

                VertexAttribute {
                    format:             wgpu::VertexFormat::Float32x3,
                    offset:             12,
                    shader_location:    1,
                }

            ],
        }
    }
}


#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Camera {
    pub matrix:   [[f32; 4]; 4],
}

impl WebGPUType for Camera {

    fn bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const _ as *const u8,
                std::mem::size_of::<Camera>(),
            )
        }
    }

    fn layout(&self) -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride:   24,
            step_mode:      VertexStepMode::Vertex,
            attributes:     &[

                VertexAttribute {
                    format:             wgpu::VertexFormat::Float32x3,
                    offset:             0 as u64,
                    shader_location:    0,
                },

                VertexAttribute {
                    format:             wgpu::VertexFormat::Float32x3,
                    offset:             12,
                    shader_location:    1,
                }

            ],
        }
    }
}