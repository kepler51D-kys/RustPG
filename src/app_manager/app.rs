use std::sync::Arc;
use crate::advanced_rendering::model::Mesh;//, voxels::{chunk_cache::IndicesSize, world::WorldManager}};
use crate::advanced_rendering::render_vertex::Vertex;

pub type IndicesSize = u16;
use glam::{Vec2, Vec3};
use instant::Instant;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window}
};

use crate::app_manager::state::State;

pub struct App {
    pub state: Option<State>,
    pub last_render_time: Instant,
    // pub world_manager: WorldManager,
    // pub camera: Camera
}

impl App {
    pub fn new() -> Self {
        Self {
            // world_manager: WorldManager::new(0,0),
            state: None,
            // camera: Camera::new(90.0),
            last_render_time: instant::Instant::now(), 
        }
    }
}
impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        // window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        self.state = Some(pollster::block_on(State::new(window)).unwrap());
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        // This is where proxy.send_event() ends up
        self.state = Some(event);
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };
        match event {
            // ...
            WindowEvent::KeyboardInput {

                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                state.handle_key(event_loop, code, key_state.is_pressed());
                state.input(&event);
            },
            WindowEvent::RedrawRequested => {
                let now = instant::Instant::now();
                let dt = now - self.last_render_time;
                self.last_render_time = now;
                state.update(dt);
                // state.cam.eye.y += 0.0005;
                // state.cam.target.y += 0.001;
                // state.cam.build_view_projection_matrix();
                // state.queue.write_buffer(&state.cam.camera_buffer, 0, bytemuck::cast_slice(&[state.cam.camera_uniform]));
                match state.render_vertices() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
                // self.world_manager.render_world(state);
            }
            _ => {}
        }
    }
    // ...
}