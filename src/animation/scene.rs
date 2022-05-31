use crate::element::Element;
use crate::{Arrow, Keyframe};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Custom type to represent the rendering **DOM**
pub type DOM = HashMap<usize, Box<dyn Element + Send>>;

/// The **Smoothie** struct
pub struct Scene {
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
            dom_copy: DOM::new(),
            start_time: Instant::now(),
        }
    }

    /// Render the current **elements** and **keyframes**
    pub fn animate_script(
        &mut self,
        scene_dom: Arc<Mutex<DOM>>,
        mut elements: HashMap<usize, Box<dyn Element + Send>>,
    ) {
        // Create connection to render thread
        self.dom = Some(scene_dom);

        // TODO: Check whether this is necessary
        //self.start_time = Instant::now();

        // start rendering the scene
        loop {
            let time_since_start = self.time_since_start().as_secs_f32();
            println!("Time since start: {:?}", time_since_start);

            elements.iter_mut().for_each(|(element_id, element)| {
                // Update all keyframes that have to be updated
                element.update_data_with_keyframes(time_since_start);

                self.dom_copy.insert(*element_id, element.clone());
            });

            // Update render thread
            self.commit();
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
