use crate::element::Element;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Custom type to represent the rendering **DOM**
pub type DOM = HashMap<usize, Box<dyn Element + Send>>;

/// The **Smoothie** struct
pub struct Smoothie {
    /// The rendering **DOM**
    dom: Arc<Mutex<DOM>>,
    /// The copy of **dom** that is exposed to user
    dom_copy: DOM,
    /// Start time of animation
    start_time: Instant,
}

impl Smoothie {
    /// Create new **Smoothie** instance
    pub fn new(dom: Arc<Mutex<DOM>>) -> Self {
        Self {
            dom,
            dom_copy: HashMap::new(),
            start_time: Instant::now(),
        }
    }

    /// Returns the time since the start of the animation
    pub fn time_since_start(&self) -> Duration {
        Instant::now() - self.start_time
    }

    /// Returns the reference to the **dom**
    pub fn dom(&mut self) -> &mut DOM {
        &mut self.dom_copy
    }

    /// Commits the changes to the **dom** to the **renderer**
    pub fn commit(&self) {
        match self.dom.lock() {
            Ok(mut dom) => {
                *dom = self.dom_copy.clone();
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}
