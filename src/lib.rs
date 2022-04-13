mod renderer;
mod smoothie;

pub use smoothie::Smoothie;
pub use smoothie::DOM;

/// Creates a new **Smoothie** instance for live rendering
pub fn shake() -> Smoothie {
    Smoothie::new()
}
