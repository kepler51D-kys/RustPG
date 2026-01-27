use std::sync::Arc;


use glam::Vec3;
use instant::Instant;
use winit::{
    application::ApplicationHandler, event::{DeviceEvent, KeyEvent, WindowEvent}, event_loop::ActiveEventLoop, keyboard::PhysicalKey, window::Window
};

use crate::{app_manager::state::State, dual_contouring::world::RenderManager};

pub struct App {
    pub render_manager: RenderManager,
    pub state: Option<State>,
    pub last_render_time: Instant,
}

impl App {
    pub fn new() -> Self {
        Self {
            render_manager: RenderManager::new(Vec3::ZERO, 10),
            state: None,
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
        self.state = Some(event);
    }
    fn device_event(
            &mut self,
            _event_loop: &ActiveEventLoop,
            _device_id: winit::event::DeviceId,
            event: DeviceEvent,
        ) {
        let current_state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };
        match event {
            DeviceEvent::MouseMotion { delta } => {
                current_state.camera_controller.handle_mouse(delta.0, delta.1);
            }
            _ => {}
        }
        // let _ = current_state.window.set_cursor_position(LogicalPosition::new(100.0f32,100.0f32));
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let current_state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };
        match event {
            WindowEvent::KeyboardInput {

                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                current_state.handle_key(event_loop, code, key_state.is_pressed());
                current_state.input(&event);
            },
            WindowEvent::RedrawRequested => {
                let now = instant::Instant::now();
                let dt = now - self.last_render_time;
                self.last_render_time = now;
                current_state.update(dt);
                // state.cam.eye.y += 0.0005;
                // state.cam.target.y += 0.001;
                // state.cam.build_view_projection_matrix();
                // state.queue.write_buffer(&state.cam.camera_buffer, 0, bytemuck::cast_slice(&[state.cam.camera_uniform]));
                match current_state.render_vertices() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = current_state.window.inner_size();
                        current_state.resize(size.width, size.height);
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