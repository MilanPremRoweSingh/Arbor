
use crate::gfx::Renderer;
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

    fn update(&mut self) {}

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

                WindowEvent::Resized(physical_size) => {}

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
