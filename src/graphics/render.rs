use std::iter;

use wgpu::Color;
use wgpu::CommandEncoderDescriptor;

use super::ComponentMesh;
use super::ComponentRenderPipelineMesh;
use super::ComponentRenderPipelineMeshUniform;
use super::GameWorld;
use super::Entity;

pub trait SystemRenderMesh {
    fn draw_mesh(&self, v: Vec<&Entity>);
}

impl SystemRenderMesh for GameWorld<'_> {
    fn draw_mesh(&self, v: Vec<&Entity>) {

        let res = self.resource.borrow();

        if !res.ctx.resized.get() { return; }

        let mut encoder = res.ctx.device.create_command_encoder(&CommandEncoderDescriptor { label: Some("Default Command Encoder") });
        let mut output =  res.ctx.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Default Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color { r: 0.0, g: 0.2, b: 0.1, a: 1.0 }),
                        store: wgpu::StoreOp::Discard,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            for i in v {
                let pipeline = i.get_component::<ComponentRenderPipelineMesh>().unwrap();
                rpass.set_pipeline(&res.render_pipeline[&pipeline.id]);

                let mesh = i.get_components::<ComponentMesh>().unwrap();
                for i in mesh {
                    rpass.set_vertex_buffer(0, res.vertex_buffer[&i.vertex_buffer].slice(..));
                    rpass.draw(0..i.vertex.len() as u32, 0..1);
                }

            }

        }

        res.ctx.queue.submit(iter::once(encoder.finish()));
        output.present();
    }
}


pub trait SystemRenderMeshUniform {
    fn draw_mesh_uniform(&self, v: Vec<&Entity>);
}

impl SystemRenderMeshUniform for GameWorld<'_> {
    fn draw_mesh_uniform(&self, v: Vec<&Entity>) {

        let res = self.resource.borrow();

        if !res.ctx.resized.get() { return; }

        let mut encoder = res.ctx.device.create_command_encoder(&CommandEncoderDescriptor { label: Some("Default Command Encoder") });
        let mut output =  res.ctx.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Default Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
                        store: wgpu::StoreOp::Discard,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            for i in v {
                let pipeline = i.get_component::<ComponentRenderPipelineMeshUniform>().unwrap();
                rpass.set_pipeline(&res.render_pipeline[&pipeline.id]);

                let mesh = i.get_components::<ComponentMesh>().unwrap();
                let bind_group = &res.bind_group[&pipeline.bind_group];

                for i in mesh {
                    rpass.set_bind_group(0, bind_group, &[]);
                    rpass.set_vertex_buffer(0, res.vertex_buffer[&i.vertex_buffer].slice(..));
                    rpass.draw(0..i.vertex.len() as u32, 0..1);
                }

            }

        }

        res.ctx.queue.submit(iter::once(encoder.finish()));
        output.present();
    }
}


