use crate::animation::Easing;
use crate::element::private::Element;

#[derive(Clone)]
pub struct Keyframe<T: Element + ?Sized> {
    pub setter_fn: fn(&mut T, f32),
    pub start_value: f32,
    pub end_value: f32,
    pub start_time: f32,
    pub duration: f32,
    pub easing: Easing,
}

impl<T: Element> Keyframe<T> {
    /// Updates the underlying value of the given **element**
    pub fn update_keyframe_data(&self, element: &mut T, time_since_start: f32) {
        let progress =
            1.0 - (((self.start_time + self.duration) - time_since_start) / self.duration);
        let eased_progress = self.get_progress_with_easing(progress);

        let diff = self.end_value - self.start_value;

        // Update value based on eased progress value
        (self.setter_fn)(element, self.start_value + eased_progress * diff);
    }

    /// Determines whether the **keyframe** is currently active
    pub fn is_active(&self, time_since_start: f32) -> bool {
        (time_since_start >= self.start_time)
            && (time_since_start <= self.start_time + self.duration)
    }

    /// Get the current progress as eased number between 0 and 1
    fn get_progress_with_easing(&self, progress: f32) -> f32 {
        match self.easing {
            Easing::Linear => progress,
            _ => progress,
        }
    }
}
