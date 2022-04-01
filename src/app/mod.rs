mod renderer;

use renderer::Renderer;

use winit::platform::run_return::EventLoopExtRunReturn;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct App {}

impl App {
    /// Creates a new **Renderer** instance
    pub fn new() -> Self {
        return App {};
    }

    /// Runs the current **application** built
    pub async fn run(&self) {
        env_logger::init();

        // To make the event loop closure work, either use a mut event_loop and event_loop.run_return() or pass self by value
        let mut event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        // Initialize rendering state
        let mut state = renderer::State::new(&window).await;

        event_loop.run_return(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    // Updated event handle!
                    match event {
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so we have to dereference it twice
                            state.resize(**new_inner_size);
                        }
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
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update();

                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        });
    }
}
