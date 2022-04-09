mod renderer;

use renderer::Renderer;

use winit::platform::run_return::EventLoopExtRunReturn;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub type UpdateFn<T> = dyn FnMut(&mut T, &mut Vec<String>);

/// The root component of any smoothie application
pub struct App<T> {
    /// Custom user state that is provided in update
    user_state: T,
    /// Custom user update function
    update_fn: Box<UpdateFn<T>>,
    /// Represents the "dom" the user can interact with
    dom: Vec<String>,
}

impl<T> App<T> {
    /// Creates a new **App** instance with provided user state
    pub fn new(user_state: T, update_fn: Box<UpdateFn<T>>) -> Self {
        let dom = Vec::new();

        Self {
            user_state,
            update_fn,
            dom,
        }
    }

    /// Runs the current **application** built
    pub async fn run(&mut self) {
        env_logger::init();

        // To make the event loop closure work, either use a mut event_loop and event_loop.run_return() or pass self by value
        let mut event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        // Initialize renderer
        let mut renderer = Renderer::new(&window).await;

        event_loop.run_return(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !renderer.input(event) {
                    // Updated event handle!
                    match event {
                        WindowEvent::Resized(physical_size) => {
                            renderer.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so we have to dereference it twice
                            renderer.resize(**new_inner_size);
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
                // Update sketch by calling the user's update function
                (self.update_fn)(&mut self.user_state, &mut self.dom);

                match renderer.render(&self.dom) {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
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
