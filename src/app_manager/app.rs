use std::sync::Arc;
use crate::{app_manager::{mesh::{Mesh, Vertex}}, voxels::{chunk_cache::IndicesSize, world::WorldManager}};

use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window}
};

use crate::app_manager::state::State;

const VERT_TEST: &[Vertex] = &[
    Vertex {pos:[-0.8,-0.4,0.0], texture_coord: [1.0,1.0]},
    Vertex {pos:[-0.8,0.4,0.0], texture_coord: [1.0,0.0]},
    Vertex {pos:[0.8,-0.4,0.0], texture_coord: [0.0,1.0]},
    Vertex {pos:[0.8,0.4,0.0], texture_coord: [0.0,0.0]},
];
const IND_TEST: &[IndicesSize] = &[
    3,1,0,
    0,2,3,
];

// const VERT_TEST: &[Vertex] = &[
//     Vertex {pos:[0.588,-0.809,0.0], texture_coord: [1.0,1.0]},
//     Vertex {pos:[-0.951,0.309,0.0], texture_coord: [0.3,1.0]},
//     Vertex {pos:[0.951,0.309,0.0], texture_coord: [1.0,0.5]},
//     Vertex {pos:[-0.588,-0.809,0.0], texture_coord: [1.0,0.5]},
//     Vertex {pos:[0.0,1.0,0.0], texture_coord: [0.5,1.0]},
// ];
// const IND_TEST: &[IndicesSize] = &[
//     0,2,4,
//     4,1,3,
//     3,0,4,
// ];
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
        // println!("frame");
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
                state.cam.eye.y += 0.0005;
                // state.cam.target.y += 0.001;
                state.cam.build_view_projection_matrix();
                state.queue.write_buffer(&state.cam.camera_buffer, 0, bytemuck::cast_slice(&[state.cam.camera_uniform]));
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