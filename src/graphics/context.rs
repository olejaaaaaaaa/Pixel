#![allow(warnings)]
use std::{cell::Cell, iter, rc::Rc};
use winit::{dpi::PhysicalSize, window::{self, Window}};
use wgpu::*;

pub struct WebGPUContext<'s> {
    pub resized:        Cell<bool>,
    pub window:         &'s Window,
    pub surface:        Surface<'s>,
    pub adapter:        wgpu::Adapter,
    pub device:         wgpu::Device,
    pub instance:       wgpu::Instance,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface_caps:   wgpu::SurfaceCapabilities,
    pub surface_format: wgpu::TextureFormat,
    pub queue:          wgpu::Queue,
}

impl<'s> WebGPUContext<'s> {

    pub fn resize(&self, size: PhysicalSize<u32>) {

        let mut width = size.width;
        let mut height = size.height;

        let max_texture_size = self.device.limits().max_texture_dimension_1d;

        if width > max_texture_size || height > max_texture_size {
            width = max_texture_size;
            height = max_texture_size;
        }

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            width: 640,
            height: 640,
            present_mode: self.surface_caps.present_modes[0],
            alpha_mode: self.surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        self.surface.configure(&self.device, &surface_config);
        self.resized.set(true);
    }

}

pub struct WebGPUContextBuilder<'s> {
    pub window:         &'s Window,
    pub surface:        Option<Surface<'s>>,
    pub adapter:        Option<wgpu::Adapter>,
    pub device:         Option<wgpu::Device>,
    pub instance:       Option<wgpu::Instance>,
    pub surface_config: Option<wgpu::SurfaceConfiguration>,
    pub surface_caps:   Option<wgpu::SurfaceCapabilities>,
    pub surface_format: Option<wgpu::TextureFormat>,
    pub queue:          Option<wgpu::Queue>,
}


impl<'s> WebGPUContextBuilder<'s> {

    fn create_canvas(window: &Window) {
        #[cfg(target_arch = "wasm32")] {
            use winit::platform::web::WindowExtWebSys;
            let win = web_sys::window().unwrap();
            let doc = win.document().unwrap();
            let body = doc.get_element_by_id("main-body").unwrap();
            let canvas = web_sys::Element::from(window.canvas().unwrap());
            body.append_child(&canvas);
        }
    }

    pub async fn new(window: &'s Window) -> Self {

        Self::create_canvas(window);

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window).expect("Error create surface");
        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface)
        }).await.expect("Error create adapter");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Self {
            window,
            surface:        Some(surface),
            adapter:        Some(adapter),
            device:         None,
            queue:          None,
            instance:       Some(instance),
            surface_config: Some(surface_config),
            surface_caps:   Some(surface_caps),
            surface_format: Some(surface_format),
        }

    }

    pub async fn with_webgl2_limits(mut self) {
        let (device, queue) = self.adapter.unwrap().request_device(&DeviceDescriptor {
            label:              Some("Main Adapter"),
            required_features:  Features::empty(),
            required_limits:    Limits::downlevel_webgl2_defaults(),
            memory_hints:       MemoryHints::Performance
        }, None).await.expect("Error create device or queue");

        self.device = Some(device);
        self.queue = Some(queue);
    }

    pub async fn with_webgl_limits(mut self) -> Self {

        let limits = wgpu::Limits {
            max_compute_workgroups_per_dimension: 0,
            max_compute_workgroup_size_z: 0,
            max_compute_workgroup_size_y: 0,
            max_compute_workgroup_size_x: 0,
            max_compute_invocations_per_workgroup: 0,
            max_compute_workgroup_storage_size: 0,
            max_storage_buffer_binding_size: 0,
            max_storage_textures_per_shader_stage: 0,
            max_storage_buffers_per_shader_stage: 0,
            max_dynamic_storage_buffers_per_pipeline_layout: 0,
            ..Default::default()
        };

        let (device, queue) = self.adapter.as_ref().unwrap().request_device(&DeviceDescriptor {
            label:                  Some("Main Adapter"),
            required_features:      Features::empty(),
            required_limits:        limits,
            memory_hints:           MemoryHints::Performance
        }, None).await.expect("Error create device or queue");

        self.device = Some(device);
        self.queue = Some(queue);

        self
    }

    pub async fn memory_hints(mut self, memory: MemoryHints) -> Self {

        let limits = self.device.unwrap().limits();
        let (device, queue) = self.adapter.as_ref().unwrap().request_device(&DeviceDescriptor {
            label: Some("Main Adapter"),
            required_features: Features::empty(),
            required_limits:   limits,
            memory_hints:      memory
        }, None).await.expect("Error create device or queue");

        self.device = Some(device);
        self.queue = Some(queue);
        self
    }

    pub async fn build(mut self) -> WebGPUContext<'s> {

        if self.queue.is_none() || self.device.is_none() {
            self = Self::with_webgl_limits(self).await;
        }

        unsafe {
            WebGPUContext {
                window:         &self.window,
                resized:        false.into(),
                surface:        self.surface.unwrap_unchecked(),
                adapter:        self.adapter.unwrap_unchecked(),
                device:         self.device.unwrap_unchecked(),
                instance:       self.instance.unwrap_unchecked(),
                surface_config: self.surface_config.unwrap_unchecked(),
                surface_caps:   self.surface_caps.unwrap_unchecked(),
                surface_format: self.surface_format.unwrap_unchecked(),
                queue:          self.queue.unwrap_unchecked()
            }
        }
    }


}
