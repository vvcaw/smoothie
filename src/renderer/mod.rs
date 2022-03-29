mod state;

pub use state::State;

use winit::platform::run_return::EventLoopExtRunReturn;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct Renderer {}

impl Renderer {
    /// Creates a new **Renderer** instance
    pub fn new() -> Renderer {
        return Renderer {};
    }

    /// Runs the current **sketch** build
    pub async fn run(&self) {
        env_logger::init();

        // To make the event loop closure work, either use a mut event_loop and event_loop.run_return() or pass self by value
        let mut event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        event_loop.run_return(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        });
    }
}
