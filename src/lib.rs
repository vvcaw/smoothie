mod renderer;
mod smoothie;

use crate::renderer::Renderer;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub use smoothie::Smoothie;
pub use smoothie::DOM;

/// Render the given function live
pub fn shake(fun: fn(&mut Smoothie)) {
    // Create dom that is synced between threads
    let dom = Arc::new(Mutex::new(HashMap::new()));
    let renderer_dom = Arc::clone(&dom).clone();

    // Smoothie instance
    let mut smoothie = Smoothie::new(dom);

    // Create thread to execute user code
    thread::spawn(move || {
        fun(&mut smoothie);
    });

    // Renderer instance
    let mut renderer = Renderer::new(renderer_dom);

    // Run render loop
    pollster::block_on(renderer.run());
}
