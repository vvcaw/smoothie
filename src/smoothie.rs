use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::animation::Scene;
use crate::element::Element;
use crate::renderer::Renderer;

pub struct Smoothie {
    /// List of **elements** in scene
    elements: Vec<Box<dyn Element + Send>>,
    /// HashMap mapping start times to keyframes
    keyframes: HashMap<f32, String>,
    /// Scene to update the data in
    scene: Scene,
}

impl Smoothie {
    /// Creates a new **Smoothie** instance
    pub(crate) fn new() -> Self {
        Smoothie {
            elements: Vec::new(),
            keyframes: HashMap::new(),
            scene: Scene::new(),
        }
    }

    /// Renders the current scene (either as live preview or as video file), the **Smoothie** object is lost after this function call, therefore, no other function calls are allowed after this one!
    pub fn serve(self) {
        // Create dom that is synced between threads
        let dom = Arc::new(Mutex::new(HashMap::new()));
        let renderer_dom = Arc::clone(&dom).clone();

        // Get mutable reference of scene to pass to second thread
        let mut scene = self.scene;

        // Create thread to execute user code
        thread::spawn(move || scene.animate_script(dom));

        // Renderer instance
        let mut renderer = Renderer::new(renderer_dom);

        // Run render loop
        pollster::block_on(renderer.run());
    }
}

// TODO: Current syntax rules allow for double semi-colon
#[macro_export]
macro_rules! animate {
    ($($obj:expr,$prop:ident => $val:expr)+ $(;with $(duration = $dur:expr)? $(;easing = $ease:expr)? $(;)?)?) => {
        $(
        println!(
            "The user wants to animate the property {:?} from {:?} to {:?} for element with id {:?}",
            (stringify! {$prop}),
            $obj.$prop,
            $val,
            $obj.id);
        )+
    };
    ($($obj:expr,$prop:ident => $val:expr)+ $(;with $(easing = $ease:expr)? $(;duration = $dur:expr)? $(;)?)?) => {
        $(
        println!(
            "The user wants to animate the property {:?} from {:?} to {:?} for element with id {:?}",
            (stringify! {$prop}),
            $obj.$prop,
            $val,
            $obj.id);
        )+
    };
}
