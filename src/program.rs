use cgmath::Vector2;
use wgpu::*;
use winit::event::WindowEvent;

use crate::maths::*;
use crate::graphics::*;
use crate::state::WinFunc;

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
    
    fn desc() -> VertexBufferLayout<'static>
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


pub struct Program
{
    render_pipeline: RenderPipeline,
    draw_object: DrawObject
}

impl WinFunc for Program
{
    // Creating some of the wgpu types requires async code
    fn new(device: &Device, config: &SurfaceConfiguration) -> Self
    {
        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                // specify vertex buffer layout
                buffers: &[Vertex::desc()],
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
            draw_object
        };
    }

    fn on_size(&mut self, size: Vector2<u32>)
    {
        
    }

    fn input(&mut self, event: &WindowEvent) -> bool
    {
        return false;
    }

    fn update(&mut self)
    {
        // todo!()
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
        self.draw_object.draw(&mut render_pass);
        
        drop(render_pass);
    }
}