use crate::smoothie::DOM;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// The **Renderer** struct
pub struct Renderer {
    /// The rendering **DOM**
    dom: Arc<Mutex<DOM>>,
}

impl Renderer {
    /// Creates new **Renderer** instance
    pub fn new(dom: Arc<Mutex<DOM>>) -> Self {
        Self { dom }
    }

    /// Rendering loop
    pub fn render(self) {
        loop {
            match self.dom.lock() {
                Ok(dom) => {
                    dom.iter().for_each(|(k, v)| {
                        println!("{}, {}", k, v);
                    });
                }
                Err(_) => {
                    println!("Locked?")
                }
            }

            thread::sleep(Duration::new(4, 0));
        }
    }
}
