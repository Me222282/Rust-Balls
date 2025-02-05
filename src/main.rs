mod ball;
mod physics;

use crate::ball::*;
use crate::physics::*;

use cgmath::Zero;
use console::Term;
use std::io::Write;

use std::thread;
use std::time::*;

// use wgpu::{Backends, Dx12Compiler, Instance, InstanceDescriptor, InstanceFlags, SurfaceTarget};
// use winit::{
//     event::*,
//     event_loop::{ControlFlow, EventLoop},
//     window::{Window, WindowBuilder, WindowId}
// };

// fn main() {
//     env_logger::init(); // Necessary for logging within WGPU
//     let event_loop = EventLoop::new().unwrap(); // Loop provided by winit for handling window events
//     let window = WindowBuilder::new().build(&event_loop).unwrap();
    
//     event_loop.set_control_flow(ControlFlow::Wait);
    
//     let instance = Instance::new(InstanceDescriptor {
//         backends: Backends::VULKAN,
//         flags: InstanceFlags::DEBUG,
//         dx12_shader_compiler: Dx12Compiler::Fxc,
//         gles_minor_version: wgpu::Gles3MinorVersion::Automatic
//     });
//     let surface = instance.create_surface(&window).unwrap();
//     let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
//         power_preference: wgpu::PowerPreference::default(),
//         compatible_surface: Some(&surface),
//         force_fallback_adapter: false,
//     }))
//     .unwrap();

//     let (device, queue) = pollster::block_on(adapter.request_device(
//         &wgpu::DeviceDescriptor {
//             label: None,
//             features: wgpu::Features::empty(),
//             limits: wgpu::Limits::default(),
//         },
//         None, // Trace path
//     ))
//     .unwrap();

//     let size = window.inner_size();
//     surface.configure(&device, &wgpu::SurfaceConfiguration {
//         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//         format: surface.get_preferred_format(&adapter).unwrap(),
//         width: size.width,
//         height: size.height,
//         present_mode: wgpu::PresentMode::Fifo,
//     });
    
//     let _ = event_loop.run(move |event, elwt| {
//         match event {
//             Event::WindowEvent {
//                 event: WindowEvent::CloseRequested,
//                 ..
//             } => {
//                 println!("The close button was pressed; stopping");
//                 elwt.exit();
//             },
//             Event::AboutToWait => {
//                 // Application update code.
    
//                 // Queue a RedrawRequested event.
//                 //
//                 // You only need to call this if you've determined that you need to redraw in
//                 // applications which do not always need to. Applications that redraw continuously
//                 // can render here instead.
//                 window.request_redraw();
//             },
//             Event::WindowEvent {
//                 event: WindowEvent::RedrawRequested,
//                 ..
//             } => {
//                 // Redraw the application.
//                 //
//                 // It's preferable for applications that do not render continuously to render in
//                 // this event rather than in AboutToWait, since rendering in here allows
//                 // the program to gracefully handle redraws requested by the OS.
//             },
//             _ => ()
//         }
//     });
// }

fn render_balls<'a, T: IntoIterator<Item = &'a Ball>>(l: T, out: &mut Term)
{
    let _ = out.clear_screen();
    let s = out.size();
    let v = ['#' as u8];
    
    for b in l
    {
        let (x, y) = (b.location.x as usize,
            s.1 as usize - b.location.y as usize - 1);
        let _ = out.move_cursor_to(x, y);
        let _ = out.write(&v);
    }
    
    let _ = out.flush();
}

fn main()
{   
    let mut term = Term::stdout();
    let mut physics = Physics::new(Vec4::new(0.0, 0.0, 20.0, 10.0));
    
    let dt = Duration::from_millis(16);
    let mut then = Instant::now();
    
    physics.add(Ball::new(Vec2::new(10.0, 5.0), 0.5, Colour::zero()));
    physics.add(Ball::new(Vec2::new(10.0, 7.0), 0.5, Colour::zero()));
    
    loop
    {
        // physics.apply_phsyics(1.0 / 60.0);
        
        render_balls(&physics, &mut term);
        
        let now = Instant::now();
        let diff = now - then;
        then = now;
        if diff > dt { continue; }
        thread::sleep(dt - diff);
    }
}