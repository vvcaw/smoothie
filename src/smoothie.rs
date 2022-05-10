use crate::element::Element;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Custom type to represent the rendering **DOM**
pub type DOM = HashMap<usize, Box<dyn Element + Send>>;

/// The **Smoothie** struct
pub struct Smoothie {
    /// The rendering **DOM**
    dom: Arc<Mutex<DOM>>,
    /// The copy of **dom** that is exposed to user
    dom_copy: DOM,
}

impl Smoothie {
    /// Create new **Smoothie** instance
    pub fn new(dom: Arc<Mutex<DOM>>) -> Self {
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
