#[macro_use]
extern crate glium;
extern crate rand;
extern crate cgmath;
extern crate time;

use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder, Event, WindowEvent, KeyboardInput,
                    ElementState, VirtualKeyCode};
use glium::{Display, Surface};
use cgmath::Vector3;
use renderer::scene::{Scene, SceneParams};
use renderer::body::BodyRenderer;
use simulation::Body;
use rand::Rng;

mod renderer;
mod simulation;

fn main() {
    // Rand
    let mut rng = rand::thread_rng();

    // Configure scene.
    let mut scene = Scene::new(
        800.0,
        600.0,
        SceneParams {
            near: 1.0,
            far: 100.0,
            focus: 100.0,
            look_at: Vector3::new(0.0, 0.0, 0.0),
            camera: Vector3::new(0.0, 0.0, 3.0),
        });

    // Configure window with context
    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_dimensions(scene.get_width() as u32, scene.get_height() as u32);
    let context = ContextBuilder::new()
        .with_vsync(true)
        .with_depth_buffer(24);
    let display = Display::new(window, context, &events_loop)
        .expect("Couldn't create display");

    // Create initial bodies.
    let mut bodies: Vec<Body> = Vec::new();
    for _ in 0..50000 {
        let mut body = rng.gen::<Body>();
        body.position *= 100.0;
        bodies.push(body);
    }

    // Configure renderers.
    let mut body_renderer = BodyRenderer::new(&display, bodies.len());

    // Event loop
    let mut closed = false;
    while !closed {

        // Handle window events.
        events_loop.poll_events(|e| {
            match e {
                Event::WindowEvent { event, .. } =>
                    match event {
                        WindowEvent::Closed => closed = true,
                        WindowEvent::Resized(width, height) => {
                            scene.set_width_and_height(width as f32, height as f32);
                            display.gl_window().window().set_inner_size(width, height);
                        }
                        WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::A),
                                state: ElementState::Pressed, ..
                            }, ..
                        } => {
                            let far = scene.get_params().far;
                            scene.set_camera((rng.gen::<Vector3<f32>>() -
                                Vector3::new(0.5, 0.5, 0.5)) * far, true);
                        }
                        WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::S),
                                state: ElementState::Pressed, ..
                            }, ..
                        } => {
                            let far = scene.get_params().far;
                            scene.set_focus(rng.gen::<f32>() * far, true);
                        }
                        _ => {}
                    }
                _ => {}
            }
        });

        // Update camera coordinates and matrices.
        scene.update();

        // Get surface to draw on.
        let mut surface = display.draw();
        surface.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        // Draw bodies.
        body_renderer.draw(&mut surface, &scene, &bodies);

        // Swap buffers.
        surface.finish().expect("Couldn't finish drawing on surface");
    }
}