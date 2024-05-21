use winit::window::Window;
use wgpu::{Instance, Surface, Device, Queue};
use futures::executor::block_on;
use std::sync::Arc;

pub struct Renderer {
    surface: Surface<'static>,
    pub device: Device,
    queue: Queue,
    config: wgpu::SurfaceConfiguration,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Renderer{
        let size = window.inner_size();

        let instance = Instance::default();
        
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions{
            compatible_surface: Some(&surface),
            ..Default::default()
        })).unwrap();

        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Renderer {
            surface,
            device,
            queue,
            config,
        }
    }
}

fn main() {

}
