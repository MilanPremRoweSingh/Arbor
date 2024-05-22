use winit::window::Window;
use wgpu::*;
use crate::gfx::formats::*;
use futures::executor::block_on;
use std::sync::Arc;

pub struct Renderer {
    surface: Surface<'static>,
    pub device: Device,
    queue: Queue,
    config: wgpu::SurfaceConfiguration,
    diffuse_bind_group_layout: wgpu::BindGroupLayout,
    render_pipeline: wgpu::RenderPipeline,
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
        
        // For now, we are going to use one shader for everything, so just define the render pipeline on the renderer itself
        let diffuse_bind_group_layout = 
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("diffuse_bind_group_layout")
            });
            
        let render_pipeline = Renderer::create_render_pipeline(&device, &diffuse_bind_group_layout, config.format);

        Renderer {
            surface,
            device,
            queue,
            config,
            diffuse_bind_group_layout,
            render_pipeline,
        }
    }

    fn create_render_pipeline(device: &Device, bind_group_layout: &BindGroupLayout, target_format: TextureFormat) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/shader.wgsl"));
        let render_pipeline_layout = 
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label : Some("Render Pipeline Layout"),
                bind_group_layouts: &[bind_group_layout],
                push_constant_ranges: &[],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module : &shader,
                entry_point: "vs_main",
                buffers: &[ 
                    Vertex::desc(), 
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.config.width  = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn draw(&self, vert_buffer: &Option<Buffer>, num_verts: u32, index_buffer: &Option<Buffer>, num_indices: u32) -> Result<(), SurfaceError> {
        
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label: Some("Render Encoder"),
        });

        let diffuse_bytes = include_bytes!("textures/tree.png"); 
        let diffuse_texture = crate::gfx::texture::Texture::from_bytes(&self.device, &self.queue, diffuse_bytes, "happy-tree.png");
        let diffuse_bind_group = self.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &self.diffuse_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label : Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                    view: &view, 
                    resolve_target: None, 
                    ops: wgpu::Operations { 
                        load: wgpu::LoadOp::Clear(wgpu::Color { 
                            r: 0.1, 
                            g: 0.2, 
                            b: 0.3, 
                            a: 1.0 }), 
                        store: wgpu::StoreOp::Store 
                    }, 
                }),],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &diffuse_bind_group, &[]);

            match vert_buffer {
                Some(buffer) => {
                   render_pass.set_vertex_buffer(0, buffer.slice(..));
                },
                _ => ()
            }
            
            match index_buffer {
                Some(buffer) => {
                    render_pass.set_index_buffer(buffer.slice(..), wgpu::IndexFormat::Uint16);
                },
                _ => ()
            }

            render_pass.draw_indexed(0..num_indices, 0, 0..1);
        }
        
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
