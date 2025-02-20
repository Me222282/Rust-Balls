use cgmath::Matrix4;
use cgmath::Vector2;
use util::BufferInitDescriptor;
use util::DeviceExt;
use wgpu::*;
use winit::event::WindowEvent;
use std::vec::Vec;
use rand::prelude::*;


use crate::ball::*;
use crate::physics::*;
use crate::maths::*;
use crate::graphics::*;
use crate::state::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex
{
    position: Vec2,
    uv: Vec2,
}
impl Vertex {
    const fn new(pos: Vec2, uv: Vec2) -> Vertex
    {
        return Vertex
        {
            position: pos,
            uv
        };
    }
}
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}
impl Vertex
{
    const ATTRIBS: [VertexAttribute; 2] =
        vertex_attr_array![0 => Float32x2, 1 => Float32x2];
    
        const fn desc() -> VertexBufferLayout<'static>
    {
        return VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS
        }
    }
}
const VERTICES: &[Vertex] = &[
    Vertex::new(vec2(0.5, 0.5), vec2(1.0, 1.0)),
    Vertex::new(vec2(-0.5, 0.5), vec2(0.0, 1.0)),
    Vertex::new(vec2(-0.5, -0.5), vec2(0.0, 0.0)),
    Vertex::new(vec2(0.5, -0.5), vec2(1.0, 0.0))
];
const INDICES: &[u16] = &[
    0, 1, 2,
    2, 3, 0
];

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Uniform
{
    matrix: Matrix4<f32>
}
unsafe impl bytemuck::Pod for Uniform {}
unsafe impl bytemuck::Zeroable for Uniform {}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
struct Instance
{
    colour: Vec3,
    location: Vec2,
    radius: f32
}
impl Instance
{
    const ATTRIBS: [VertexAttribute; 3] =
        vertex_attr_array![3 => Float32x3, 4 => Float32x2, 5 => Float32];
    
    const fn desc() -> VertexBufferLayout<'static>
    {
        return VertexBufferLayout {
            array_stride: std::mem::size_of::<Instance>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &Self::ATTRIBS
        }
    }
    
    const fn from_ball(b: Ball) -> Instance
    {
        return Instance {
            colour: c_to_v(b.colour),
            location: b.location,
            radius: b.radius
        };
    }
}
unsafe impl bytemuck::Pod for Instance {}
unsafe impl bytemuck::Zeroable for Instance {}

pub struct Program
{
    render_pipeline: RenderPipeline,
    draw_object: DrawObject,
    uniform_buffer: Buffer,
    uniform_data: Uniform,
    bind_group: BindGroup,
    instances: Vec<Instance>,
    instance_buffer: Buffer,
    
    physics: Physics
}

impl WinFunc for Program
{
    // Creating some of the wgpu types requires async code
    fn new(device: &Device, config: &SurfaceConfiguration) -> Self
    {
        let mut instances = Vec::with_capacity(100);
        let mut physics = Physics::new(size_bounds(config.width as f32, config.height as f32));
        let mut rand = rand::rng();
        
        let bounds = physics.get_bounds();
        let range = vec2(bounds.x, bounds.w)..vec2(bounds.y, bounds.z);
        for _ in 0..100
        {
            let b = Ball::random(&mut rand, &range, 1.0..5.0);
            physics.add(b);
            instances.push(Instance::from_ball(b));
        }
        
        let uniform_data = Uniform {
            matrix: Matrix4::from_scale(1.0)
        };
        
        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform_data]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        
        let uniform_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("uniform_bind_group_layout"),
        });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });
        
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instances[..]),
                usage: wgpu::BufferUsages::VERTEX | BufferUsages::COPY_DST,
            }
        );
        
        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });
        
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                // specify vertex buffer layout
                buffers: &[Vertex::desc(), Instance::desc()],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None
        });
        
        let draw_object = DrawObject::new(&device, VERTICES, INDICES);
        
        return Self {
            render_pipeline,
            draw_object,
            uniform_buffer,
            uniform_data,
            bind_group: uniform_bind_group,
            instances,
            instance_buffer,
            physics
        };
    }

    fn on_size(&mut self, size: Vector2<u32>)
    {
        // let m: Matrix4<f32> = ortho(size.x as f32, size.y as f32).into();
        let m = Matrix4::from_nonuniform_scale(2.0 / size.x as f32, 2.0 / size.y as f32, 1.0);
        self.uniform_data = Uniform {
            matrix: OPENGL_TO_WGPU_MATRIX * m
        };
        
        self.physics.set_bounds(size_bounds(size.x as f32, size.y as f32));
    }

    fn input(&mut self, event: &WindowEvent) -> bool
    {
        return false;
    }

    fn update(&mut self, queue: &Queue)
    {
        self.physics.apply_phsyics(1.0 / 60.0);
        fill_buffer(&self.physics, &mut self.instances);
        
        queue.write_buffer(&self.instance_buffer,
            0, bytemuck::cast_slice(&self.instances[..]));
        
        queue.write_buffer(&self.uniform_buffer,
            0, bytemuck::cast_slice(&[self.uniform_data]));
    }

    fn render(&mut self, encoder: &mut CommandEncoder, view: &TextureView)
    {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        self.draw_object.draw(&mut render_pass, self.instances.len() as u32);
    }
}

fn fill_buffer(balls: &Physics, inst: &mut Vec<Instance>)
{
    if inst.len() != balls.count() { return; }
    
    let mut i = 0;
    for b in balls
    {
        inst[i] = Instance::from_ball(*b);
        i += 1;
    }
}