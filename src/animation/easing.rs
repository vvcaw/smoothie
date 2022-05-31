#[derive(Debug, Clone)]
pub enum Easing {
    Linear,
    EaseInOut,
}

/// Evaluates a value between 0 and 1 based on a given **easing** and an absolute value (progress) between 0 and 1
pub fn evaluate_easing_progress(easing: Easing, progress: f32) -> f32 {
    match easing {
        Easing::Linear => progress,
        _ => progress,
    }
}
