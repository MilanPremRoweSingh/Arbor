use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BufferUsages};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride : std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}

// TODO: It actually doesn't really make sense for Renderables to own their buffers, since we want to be able to share vertex buffers for different renderables.
struct Renderable {
    vertex_buffer: Option<wgpu::Buffer>,
    num_verts: i32,
    index_buffer: Option<wgpu::Buffer>,
    num_indices: i32,
}

impl Renderable {
    fn new(
        renderer: &crate::gfx::Renderer, 
        vert_data: &Option<Vec<Vertex>>,
        index_data: &Option<Vec<i32>>) -> Renderable{

        let vertex_buffer = match vert_data {
            Some(data) => {
                Some(renderer.device.create_buffer_init(&BufferInitDescriptor{
                    contents: bytemuck::cast_slice(data.as_slice()),
                    label: None,
                    usage: BufferUsages::VERTEX,
                }))
            },
            _=> None
        };

        let num_verts = match vert_data{
            Some(data) => data.len() as i32,
            None => 0
        };

        let index_buffer = match index_data {
            Some(data) => {
                Some(renderer.device.create_buffer_init(&BufferInitDescriptor{
                    contents: bytemuck::cast_slice(data.as_slice()),
                    label: None,
                    usage: BufferUsages::INDEX,
                }))
            },
            _=> None
        };

        let num_indices = match index_data {
            Some(data) => data.len() as i32,
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