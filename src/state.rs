use std::cell::RefCell;

use cgmath::{Matrix4, Vector2};
use wgpu::*;
use winit::{dpi::PhysicalSize, event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowBuilder}};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub trait WinFunc where Self: Sized
{
    fn new(device: &Device, config: &SurfaceConfiguration) -> Self;
    fn update(&mut self, source: &State<Self>);
    fn render(&mut self, encoder: &mut CommandEncoder, view: &TextureView, source: &State<Self>);
    fn input(&mut self, event: &WindowEvent, source: &State<Self>) -> bool;
    fn on_size(&mut self, size: Vector2<u32>, source: &State<Self>);
}

pub struct State<'a, T: WinFunc>
{
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: &'a Window,
    imp: RefCell<T>
}

impl<'a, T: WinFunc> State<'a, T>
{
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &'a Window) -> Self
    {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::VULKAN,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();
        
        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                required_features: Features::empty(),
                required_limits: Limits::default(),
                label: None,
                memory_hints: Default::default(),
            },
            None, // Trace path
        ).await.unwrap();
        
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::AutoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        
        let imp = RefCell::new(T::new(&device, &config));
        
        return Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            imp
        };
    }

    fn window(&self) -> &Window
    {
        return &self.window;
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>)
    {
        if new_size.width > 0 && new_size.height > 0
        {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
        
        self.imp.borrow_mut().on_size(Vector2::<u32>::new(new_size.width, new_size.height), &self);
    }

    fn input(&mut self, event: &WindowEvent) -> bool
    {
        return self.imp.borrow_mut().input(event, &self);
    }

    fn render(&mut self) -> Result<(), SurfaceError>
    {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        self.imp.borrow_mut().render(&mut encoder, &view, &self);
        
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        return Ok(());
    }
}

pub async fn run<T: WinFunc>()
{
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    
    let mut state = State::<T>::new(&window).await;
    
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
                        
                        state.imp.borrow_mut().update(&state);
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
                            Err(SurfaceError::Other) => {
                                log::error!("ERROR");
                                control_flow.exit();
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