#![allow(dead_code)]
#![allow(unused_imports)]
mod advanced_rendering;
mod entities;
mod v3;
mod voxels;
mod app_manager;

use winit::event_loop::EventLoop;

use crate::app_manager::app::App;

pub fn run() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}
fn main() {
    let _ = run();
}