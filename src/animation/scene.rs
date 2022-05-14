use crate::element::Element;
use crate::Arrow;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Custom type to represent the rendering **DOM**
pub type DOM = HashMap<usize, Box<dyn Element + Send>>;

/// The **Smoothie** struct
pub struct Scene {
    /// List of **elements** in scene
    elements: Vec<Box<dyn Element + Send>>,
    /// HashMap mapping start times to keyframes
    keyframes: HashMap<f32, String>,
    /// The rendering **DOM**
    dom: Option<Arc<Mutex<DOM>>>,
    /// The copy of **dom** that is exposed to user
    dom_copy: DOM,
    /// Start time of animation
    start_time: Instant,
}

impl Scene {
    /// Create new **Smoothie** instance
    pub fn new() -> Self {
        Self {
            dom: None,
            elements: Vec::new(),
            keyframes: HashMap::new(),
            dom_copy: HashMap::new(),
            start_time: Instant::now(),
        }
    }

    /// Render the current **elements** and **keyframes**
    pub fn animate_script(&mut self, dom: Arc<Mutex<DOM>>) {
        self.dom = Some(dom);

        // Populate dom_copy with data from keyframes and elements

        // start rendering the scene
        self.dom_copy.insert(0, Box::new(Arrow::new(0.4)));
        self.commit();

        let mut duration = self.time_since_start().as_secs_f32();

        loop {
            self.dom_copy
                .get_mut(&0)
                .unwrap()
                .set_scale((duration / 3f32) % 1f32);
            self.dom_copy
                .get_mut(&0)
                .unwrap()
                .set_angle((duration * PI) % 2.0 * PI);

            self.commit();
            duration = self.time_since_start().as_secs_f32();
        }
    }

    /// Returns the time since the start of the animation
    fn time_since_start(&self) -> Duration {
        Instant::now() - self.start_time
    }

    /// Commits the changes to the **dom** to the **renderer**
    fn commit(&self) {
        match &self.dom {
            Some(dom) => match dom.lock() {
                Ok(mut dom) => {
                    *dom = self.dom_copy.clone();
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                }
            },
            None => {
                eprintln!("Provide DOM before calling this function!");
            }
        }
    }
}
