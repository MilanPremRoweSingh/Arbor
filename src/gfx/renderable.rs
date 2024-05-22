use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BufferUsages};
use crate::gfx::formats::Vertex;

// TODO: It actually doesn't really make sense for Renderables to own their buffers, since we want to be able to share vertex buffers for different renderables.
pub struct Renderable {
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub num_verts: u32,
    pub index_buffer: Option<wgpu::Buffer>,
    pub num_indices: u32,
}

impl Renderable {
    pub fn new(
        renderer: &crate::gfx::Renderer, 
        vert_data: Option<&[Vertex]>,
        index_data: Option<&[u16]>) -> Renderable{

        let vertex_buffer = match vert_data {
            Some(data) => {
                Some(renderer.device.create_buffer_init(&BufferInitDescriptor{
                    contents: bytemuck::cast_slice(data),
                    label: None,
                    usage: BufferUsages::VERTEX,
                }))
            },
            _=> None
        };

        let num_verts = match vert_data{
            Some(data) => data.len() as u32,
            None => 0
        };

        let index_buffer = match index_data {
            Some(data) => {
                Some(renderer.device.create_buffer_init(&BufferInitDescriptor{
                    contents: bytemuck::cast_slice(data),
                    label: None,
                    usage: BufferUsages::INDEX,
                }))
            },
            _=> None
        };

        let num_indices = match index_data {
            Some(data) => data.len() as u32,
            None => 0,
        };

        Renderable {
            vertex_buffer,
            num_verts,
            index_buffer,
            num_indices,
        }
    }
}