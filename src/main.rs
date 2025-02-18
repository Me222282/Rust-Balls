mod ball;
mod physics;
mod program;
mod maths;
mod graphics;

use crate::program::*;

use wgpu::*;
use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
    keyboard::{KeyCode, PhysicalKey}
};

async fn run()
{
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    
    let mut state = State::new(&window).await;
    
    let _ = event_loop.run(move |event, control_flow|
    {
        match event
        {
            Event::WindowEvent
            {
                ref event,
                window_id,
            } if window_id == state.window().id() => if !state.input(event)
            {
                match event
                {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput
                    {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => control_flow.exit(),
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    },
                    WindowEvent::RedrawRequested => {
                        // This tells winit that we want another frame after this one
                        state.window().request_redraw();
            
                        // if !surface_configured {
                        //     return;
                        // }
            
                        state.update();
                        match state.render() {
                            Ok(_) => {}
                            // Reconfigure the surface if it's lost or outdated
                            Err(
                                SurfaceError::Lost | SurfaceError::Outdated,
                            ) => state.resize(state.size),
                            // The system is out of memory, we should probably quit
                            Err(SurfaceError::OutOfMemory) => {
                                log::error!("OutOfMemory");
                                control_flow.exit();
                            }
            
                            // This happens when the a frame takes too long to present
                            Err(SurfaceError::Timeout) => {
                                log::warn!("Surface timeout")
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}

fn main() {
    pollster::block_on(run());
}
