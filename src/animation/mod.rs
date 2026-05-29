use std::time::{Duration, Instant};

use ratatui::widgets::canvas::Shape;

use crate::render::Renderable;

pub struct Animation<R: Renderable> {
    object: R,
    birth: Instant,
    duration: Duration,
}

impl<R: Renderable> Animation<R> {
    pub fn new(object: R, duration: f64) -> Self {
        Animation {
            object,
            birth: Instant::now(),
            duration: Duration::from_secs_f64(duration),
        }
    }

    pub fn is_ongoing(&self) -> bool {
        let elapsed = self.birth.elapsed();
        elapsed.saturating_sub(self.duration) == Duration::ZERO
    }

    pub fn to_shape(&self) -> impl Shape {
        let normalized = self.normalize();
        self.object.to_shape(normalized)
    }

    fn normalize(&self) -> f64 {
        let elapsed = self.birth.elapsed();
        elapsed.div_duration_f64(self.duration).clamp(0.0, 1.0)
    }
}
