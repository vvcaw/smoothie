extern crate core;

mod animation;
mod element;
mod renderer;
mod smoothie;

pub use animation::Easing;
pub use animation::Keyframe;
pub use animation::DOM;
pub use element::Arrow;
pub use smoothie::Smoothie;

/// Returns a **Smothie** instance for rendering a script
pub fn shake() -> Smoothie {
    Smoothie::new()
}
