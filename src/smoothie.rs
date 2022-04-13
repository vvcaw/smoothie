use crate::renderer::Renderer;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

/// Custom type to represent the rendering **DOM**
pub type DOM = HashMap<String, String>;

/// The **Smoothie** struct
pub struct Smoothie {
    /// The rendering **DOM**
    dom: Arc<Mutex<DOM>>,
    /// The copy of **dom** that is exposed to user
    dom_copy: DOM,
}

impl Smoothie {
    /// Create new **Smoothie** instance
    pub fn new() -> Self {
        let dom = Arc::new(Mutex::new(HashMap::new()));

        // Create renderer to run asynchronously
        let renderer_dom = Arc::clone(&dom);
        thread::spawn(move || {
            let renderer = Renderer::new(renderer_dom);
            renderer.render();
        });

        Self {
            dom,
            dom_copy: HashMap::new(),
        }
    }

    /// Returns the reference to the **dom**
    pub fn dom(&mut self) -> &mut DOM {
        &mut self.dom_copy
    }

    /// Commits the changes to the **dom** to the **renderer**
    pub fn commit(&self) {
        println!("Commited");
        match self.dom.lock() {
            Ok(mut dom) => {
                *dom = self.dom_copy.clone();
            }
            Err(_) => {
                println!("Locked?");
            }
        }
    }
}
