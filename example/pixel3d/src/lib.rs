#![allow(warnings)]

use log::warn;
use winit::event::WindowEvent;
use winit::dpi::PhysicalSize;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;
use wgpu::*;

use pixel::*;

#[wasm_bindgen]
pub fn main() {
    spawn_local(run());
}

pub async fn run() {

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn);

    let main_loop = winit::event_loop::EventLoop::new().unwrap();
    let mut window = winit::window::WindowBuilder::new().build(&main_loop).unwrap();
    window.request_inner_size(PhysicalSize::new(640, 640));

    let ctx = WebGPUContextBuilder::new(&window)
        .await
        .build()
        .await;

    let world = GameWorld::new(ctx, &window).await;

    let mut player = world.create_entity();
    player.add_shader_mesh_uniform();


    let angle_x = 30.0_f32.to_radians(); // Ровно 30 градусов в радианах
    let angle_y = 30.0_f32.to_radians(); // Ровно 30 градусов в радианах

// Матрица поворота по оси X
let rotation_x = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, angle_x.cos(), -angle_x.sin(), 0.0],
    [0.0, angle_x.sin(), angle_x.cos(), 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

// Матрица поворота по оси Y
let rotation_y = [
    [angle_y.cos(), 0.0, angle_y.sin(), 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [-angle_y.sin(), 0.0, angle_y.cos(), 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

let mut final_rotation = [[0.0; 4]; 4];

for i in 0..4 {
    for j in 0..4 {
        final_rotation[i][j] =
            rotation_y[i][0] * rotation_x[0][j] +
            rotation_y[i][1] * rotation_x[1][j] +
            rotation_y[i][2] * rotation_x[2][j] +
            rotation_y[i][3] * rotation_x[3][j];
    }
}

let c = [
    [0.2, 0.0, 0.0, 0.0],
    [0.0, 0.2, 0.0, 0.0],
    [0.0, 0.0, 0.2, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

let mut final_rotation2 = [[0.0; 4]; 4];

for i in 0..4 {
    for j in 0..4 {
        final_rotation2[i][j] =
            final_rotation[i][0] * c[0][j] +
            final_rotation[i][1] * c[1][j] +
            final_rotation[i][2] * c[2][j] +
            final_rotation[i][3] * c[3][j];
    }
}

// Создаём камеру с итоговой матрицей
let _max = Camera {
    matrix: final_rotation2,
};



player.add_mesh(vec![
    

], None);

    player.add_uniform(ShaderStages::VERTEX, _max);

    player.add_mesh_uniform_pipeline(PrimitiveTopology::LineStrip);

    main_loop.run(move |event, event_loop_window_target| {

        match event {

            winit::event::Event::AboutToWait => {

            }

            winit::event::Event::WindowEvent { window_id, event } => {

                match event {

                    WindowEvent::Destroyed => {

                    }

                    WindowEvent::RedrawRequested => {
                        world.draw_mesh_uniform(vec![&player]);
                    }

                    WindowEvent::Resized(size) => {
                        world.resize(size);
                    }

                    _ => ()
                }
            }
            _ => ()
        }
    });

}