use std::sync::Arc;
use crate::{app_manager::{mesh::{Mesh, Vertex}}, voxels::{chunk_cache::IndicesSize, world::WorldManager}};

use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window}
};

use crate::app_manager::window::State;

const VERT_TEST: &[Vertex] = &[
    Vertex {pos:[-0.5,-0.5,0.0]},
    Vertex {pos:[0.5,-0.5,0.0]},
    Vertex {pos:[-0.5,0.5,0.0]},
    Vertex {pos:[0.5,0.5,0.0]},
];
const IND_TEST: &[IndicesSize] = &[
    3,2,0,1,3,0
];
pub struct App {
    pub state: Option<State>,
    pub world_manager: WorldManager,
    // pub camera: Camera
}

impl App {
    pub fn new() -> Self {
        Self {
            world_manager: WorldManager::new(0,0),
            state: None,
            // camera: Camera::new(90.0),
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
        // println!("hello");
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
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            WindowEvent::RedrawRequested => {
                state.update();
                // state.cam.eye.x += 0.001;
                // state.cam.target.x += 0.001;
                // state.camera_uniform = state.cam.build_view_projection_matrix();
                // state.queue.write_buffer(&state.camera_buffer, 0, bytemuck::cast_slice(&[state.camera_uniform]));
                // self.world_manager.render_world(state);
                match state.render_vertices(&Mesh {vertices:VERT_TEST.to_vec(),indices:IND_TEST.to_vec()}) {
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
            }
            _ => {}
        }
    }
    // ...
}