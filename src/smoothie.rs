use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::animation::{Keyframe, Scene, DOM};
use crate::element::Element;
use crate::renderer::Renderer;
use crate::{Arrow, Easing};

pub struct Smoothie {
    /// List of elements in the scene
    elements: HashMap<usize, Box<dyn Element + Send>>,
    /// Current element id
    current_element_id: usize,
    /// Current time in animation
    current_time: f32,
    /// Scene to update the data in
    scene: Scene,
}

impl Smoothie {
    /// Creates a new **Smoothie** instance
    pub(crate) fn new() -> Self {
        Smoothie {
            elements: HashMap::new(),
            current_element_id: 0,
            current_time: 0f32,
            scene: Scene::new(),
        }
    }

    /// Creates a new **Arrow**
    pub fn arrow(&mut self) -> Arrow {
        let arrow = Arrow {
            x: 0.0,
            y: 0.0,
            angle: 0.0,
            scale: 1.0,
            stroke: false,
            fill: true,
            keyframes: vec![],
            id: self.current_element_id,
        };

        // Increment element id counter
        self.current_element_id += 1;

        arrow
    }

    /// Adds an **element** to tracking, this method is invoked by the `add!` macro!
    pub fn add_element(&mut self, element_reference: &(dyn Element)) {
        self.elements
            .insert(element_reference.get_id(), element_reference.box_clone());
    }

    /// Get the current animation time
    pub fn get_current_animation_time(&self) -> f32 {
        self.current_time
    }

    /// Increment the current animation time, this method is automatically invoked by the `animate!` macro
    pub fn increment_animation_time(&mut self, duration: f32) {
        self.current_time += duration;
    }

    /// Renders the current scene (either as live preview or as video file), the **Smoothie** object is lost after this function call, therefore, no other function calls are allowed after this one!
    pub fn serve(self) {
        // Sync DOM between threads
        let scene_dom = Arc::new(Mutex::new(DOM::new()));
        let renderer_dom = Arc::clone(&scene_dom).clone();

        // Get mutable reference of scene to pass to second thread
        let mut scene = self.scene;

        // User DOM
        let elements = self.elements;

        // Create thread to execute user code
        thread::spawn(move || scene.animate_script(scene_dom, elements));

        // Renderer instance
        let mut renderer = Renderer::new(renderer_dom);

        // Run render loop
        pollster::block_on(renderer.run());
    }
}

#[macro_export]
/// Animates certain values from the current value to the given value
///
/// # Examples
///
/// ```
/// use smoothie::{animate};
/// let mut smoothie = smoothie::shake();
/// let mut arrow = smoothie.arrow();
///
/// animate! {
///     smoothie;
///     arrow,x => 14.0;
///     arrow,y => 12.0;
/// };
///
/// smoothie.serve();
/// ```
macro_rules! animate {
    // Pattern without duration & easing
    ($smoothie:expr; $($object:expr,$property:ident => $value:expr);* $(;)?) => {{
        $(
            // Generate correct setter function based on element type
            let setter_fn = match $object {
                smoothie::Arrow { .. } => |object: &mut smoothie::Arrow, progress: f32| {
                    object.$property = progress;
                }
            };

            // Add keyframes to element
            $object.add_keyframe((setter_fn, $object.$property, $value, $smoothie.get_current_animation_time(), 1.0, smoothie::Easing::EaseInOut));

            // Add elements to track list
            $smoothie.add_element(&$object);

            // Update value in live element
            $object.$property = $value;
        )*

        $smoothie.increment_animation_time(1.0);
    }};
    // Pattern with duration & without easing
    ($smoothie:expr; duration = $duration:expr; $($object:expr,$property:ident => $value:expr);* $(;)?) => {{
        let mut keyframes = Vec::new();

        $(
            keyframes.push(($object.id, String::from(stringify!($property)), $object.$property, $value, $duration, smoothie::Easing::EaseInOut));
        )*

        $smoothie.add_keyframes(keyframes, $duration);
    }};
    // Pattern with easing & without duration
    ($smoothie:expr; easing = $easing:expr; $($object:expr,$property:ident => $value:expr);* $(;)?) => {{
        let mut keyframes = Vec::new();

        $(
            keyframes.push(($object.id, String::from(stringify!($property)), $object.$property, $value, 1.0, $easing));
        )*

        $smoothie.add_keyframes(keyframes, 1.0);
    }};
    // Pattern with easing & duration
    ($smoothie:expr; easing = $easing:expr; duration = $duration:expr; $($object:expr,$property:ident => $value:expr);* $(;)?) => {{
        let mut keyframes = Vec::new();

        $(
            keyframes.push(($object.id, String::from(stringify!($property)), $object.$property, $value, $duration, $easing));
        )*

        $smoothie.add_keyframes(keyframes, $duration);
    }};
    // Pattern with easing & duration in other direction
    ($smoothie:expr; duration = $duration:expr; easing = $easing:expr; $($object:expr,$property:ident => $value:expr);* $(;)?) => {{
        let mut keyframes = Vec::new();

        $(
            keyframes.push(($object.id, String::from(stringify!($property)), $object.$property, $value, $duration, $easing));
        )*

        $smoothie.add_keyframes(keyframes, $duration);
    }};
}
