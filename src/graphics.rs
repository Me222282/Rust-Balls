use bytemuck::*;
use util::DeviceExt;
use wgpu::*;

pub trait IndexFormatType {
    fn get_format() -> IndexFormat;
}

impl IndexFormatType for u16 {
    fn get_format() -> IndexFormat
    {
        return IndexFormat::Uint16;
    }
}
impl IndexFormatType for u32 {
    fn get_format() -> IndexFormat
    {
        return IndexFormat::Uint32;
    }
}

pub struct DrawObject
{
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    length: u32,
    index_format: IndexFormat
}

impl DrawObject
{
    pub fn new<V: Pod + Zeroable, I: IndexFormatType + Pod + Zeroable>(device: &Device,
        vertex: &[V], index: &[I]) -> DrawObject
    {
        let vertex_buffer = device.create_buffer_init(
            &util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertex),
                usage: BufferUsages::VERTEX,
            }
        );
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(index),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        
        return DrawObject
        {
            vertex_buffer,
            index_buffer,
            length: index.len() as u32,
            index_format: I::get_format()
        };
    }
    
    pub fn draw(&self, render_pass: &mut RenderPass<'_>)
    {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), self.index_format);
        render_pass.draw_indexed(0..self.length, 0, 0..1);
    }
}