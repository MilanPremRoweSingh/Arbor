
use crate::gfx::Renderer;
use crate::gfx::renderable::{
    Renderable};
use crate::gfx::formats::Vertex;

use std::sync::Arc;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoopWindowTarget,
    keyboard::{self, KeyCode, PhysicalKey},
    window::Window,
};



pub struct Application {
    window: Arc<Window>,
    renderer: Renderer,
}

impl Application {
    pub fn run() {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
       
        let window = Arc::new(winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap());
    
        let mut renderer = Renderer::new(window.clone());

        let mut app = Application {
            window,
            renderer
        };

        event_loop.run(|event, handler| {
            app.process_window_events(event, handler);
        }).expect("Failed to start event loop");
    }

    fn update(&mut self) {
        const VERTICES: &[Vertex] = &[
            Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
            Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
            Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // C
            Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // D
            Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, 
        ];
        
        const INDICES: &[u16] = &[
            0, 1, 4,
            1, 2, 4,
            2, 3, 4,
        ];

        let test_renderable = Renderable::new(
            &self.renderer,
            Some(&VERTICES),
            Some(&INDICES),);

        self.renderer.draw(&test_renderable.vertex_buffer, test_renderable.num_verts, &test_renderable.index_buffer, test_renderable.num_indices); 
    }

    fn process_window_events<T, F>(&mut self, event: Event<T>, handler: &EventLoopWindowTarget<F>) {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => match event {
                WindowEvent::CloseRequested => handler.exit(),

                WindowEvent::KeyboardInput { .. } => {
                    println!("Input");
                }

                WindowEvent::Resized(physical_size) => {
                    self.renderer.resize(*physical_size);
                }

                WindowEvent::RedrawRequested => {
                    // This tells winit that we want another frame after this one
                    self.window.request_redraw();
                    self.update();

                    // RENDER
                }
                _ => {}
            },
            _ => {}
        }
    }    
}
