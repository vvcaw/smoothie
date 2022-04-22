mod primitive;
mod render_state;
mod vertex;
mod with_id;

use crate::DOM;
use render_state::RenderState;

use std::sync::{Arc, Mutex};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;

/// The **Renderer** struct
pub struct Renderer {
    /// The rendering **DOM**
    dom: Arc<Mutex<DOM>>,
}

impl Renderer {
    /// Creates a new **Renderer** instance to take care of rendering to the screen
    pub fn new(dom: Arc<Mutex<DOM>>) -> Self {
        Self { dom }
    }

    /// Runs the current **application** built
    pub async fn run(&mut self) {
        env_logger::init();

        // To make the event loop closure work, either use a mut event_loop and event_loop.run_return() or pass self by value
        let mut event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        // Initialize renderer
        let mut render_state = RenderState::new(&window).await;

        event_loop.run_return(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !render_state.input(event) {
                    // Updated event handle!
                    match event {
                        WindowEvent::Resized(physical_size) => {
                            render_state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so we have to dereference it twice
                            render_state.resize(**new_inner_size);
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
                match self.dom.lock() {
                    Ok(dom) => {
                        match render_state.render(dom) {
                            Ok(_) => {}
                            // Reconfigure the surface if lost
                            Err(wgpu::SurfaceError::Lost) => {
                                render_state.resize(render_state.size())
                            }
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                *control_flow = ControlFlow::Exit
                            }
                            // All other errors (Outdated, Timeout) should be resolved by the next frame
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
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
