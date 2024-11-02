use log::warn;
use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, PipelineLayoutDescriptor, PrimitiveTopology, RenderPipelineDescriptor, TextureFormat};
use super::{bind_group, id, ComponentMesh, ComponentShaderMesh, ComponentShaderMeshUniform, ComponentUniform, Entity, Id, WebGPUType};


pub trait SystemRenderPipelineMesh {
    fn add_mesh_pipeline(&mut self, topology: PrimitiveTopology);
}

impl SystemRenderPipelineMesh for Entity<'_> {
    fn add_mesh_pipeline(&mut self, topology: PrimitiveTopology) {
        self.add_component(ComponentRenderPipelineMesh::new(self, topology));
    }
}

pub struct ComponentRenderPipelineMesh {
    pub id: Id
}

impl ComponentRenderPipelineMesh {
    fn new(entity: &Entity, topology: PrimitiveTopology) -> Self {

        let mut res = entity.game_resource.borrow_mut();

        let mesh = entity.get_component::<ComponentMesh>().unwrap();
        let shader = entity.get_component::<ComponentShaderMeshUniform>().unwrap();
        let shader = &res.shader[&shader.id];

            let pipeline = res.ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: None,

                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    compilation_options: Default::default(),
                    buffers: &[mesh.vertex.layout()],
                },

                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: TextureFormat::Rgba8UnormSrgb,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                operation: wgpu::BlendOperation::Add,
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            },
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),

                primitive: wgpu::PrimitiveState {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    topology: PrimitiveTopology::TriangleList,
                    ..Default::default()
                },

                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

        let id = id();
        res.render_pipeline.insert(id, pipeline);

        Self {
            id
        }
    }
}



pub struct ComponentPipeLineLayout {
    pipeline_layout: Id
}


pub trait SystemRenderPipelineMeshUniform {
    fn add_mesh_uniform_pipeline(&mut self, topology: PrimitiveTopology);
}

impl SystemRenderPipelineMeshUniform for Entity<'_> {
    fn add_mesh_uniform_pipeline(&mut self, topology: PrimitiveTopology) {
        self.add_component(ComponentRenderPipelineMeshUniform::new(self, topology));
    }
}

pub struct ComponentRenderPipelineMeshUniform {
    pub id: Id,
    pub bind_group: Id,
}

impl ComponentRenderPipelineMeshUniform {
    fn new(entity: &Entity, topology: PrimitiveTopology) -> Self {

        let mut res = entity.game_resource.borrow_mut();

        let mesh = entity.get_component::<ComponentMesh>().unwrap();
        let shader = entity.get_component::<ComponentShaderMeshUniform>().unwrap();
        let uniform = entity.get_components::<ComponentUniform>().unwrap();

        let mut entry = vec![];
        let mut bind = 0;

        for i in &uniform {
            let s = res.uniform_buffer[&i.buffer].as_entire_binding();

            entry.push(BindGroupLayoutEntry {
                binding: bind,
                visibility: i.visible,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None },
                count: None,
            });

            bind += 1;
        }

        warn!("{:?}", entry);

        let bind_group_layout = res.ctx.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &entry
        });

        let mut bind = 0;
        let mut v = vec![];

        for i in &uniform {

            v.push(BindGroupEntry {
                binding: bind,
                resource: res.uniform_buffer[&i.buffer].as_entire_binding()
            });

            bind += 1;
        }

        warn!("{:?}", v);

        let bind_group = res.ctx.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &v
        });

        let bind_group_id = id();
        res.bind_group.insert(bind_group_id, bind_group);

        let pipeline_layout = res.ctx.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader = &res.shader[&shader.id];

        let pipeline = res.ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),

            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[mesh.vertex.layout()],
            },

                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: TextureFormat::Rgba8UnormSrgb,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                operation: wgpu::BlendOperation::Add,
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            },
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),

                primitive: wgpu::PrimitiveState {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    topology,
                    ..Default::default()
                },

                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

        let pipeline_id = id();
        res.render_pipeline.insert(pipeline_id, pipeline);

        Self {
            id: pipeline_id,
            bind_group: bind_group_id,
        }
    }
}





