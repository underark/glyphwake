use crate::animation::Animation;
// TODO: look into using clamp to apply max values
use crate::app::AppState;
use crate::math::ease_out_circ;
use crate::math::quadratic;
use crate::math::reciprocal_decay;
use crate::mode::RenderMode;
use crate::render::Renderable;
use crate::render::draw_scene;
use ratatui::Frame;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Circle, Shape};
use std::time::Duration;
use std::time::Instant;

pub struct PulseRenderer {
    objects: Vec<Animation<Pulse>>,
    next: Instant,
}

#[derive(Debug)]
struct Pulse {
    max_radius: f64,
}

impl Renderable for Pulse {
    fn to_shape(&self, normalized: f64) -> impl Shape {
        Circle {
            // TODO: Move this to some kind of config
            x: 400.0,
            y: 400.0,
            radius: ease_out_circ(normalized) * self.max_radius,
            color: Color::Red,
        }
    }
}

impl Pulse {
    fn from(s: &AppState) -> Pulse {
        let max_radius = quadratic(s.wpm.into()).clamp(100.0, 400.0);
        Pulse { max_radius }
    }
}

impl PulseRenderer {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            next: Instant::now(),
        }
    }

    fn add_pulse(&mut self, s: &AppState) {
        let p = Pulse::from(s);
        let duration = reciprocal_decay(s.wpm.into(), 10.0, 5.0);
        let animation = Animation::new(p, duration);
        self.objects.push(animation);
    }

    fn next_is_ready(&self) -> bool {
        self.next.elapsed() > Duration::from_secs_f64(0.0)
    }

    fn to_shapes(&self) -> Vec<impl Shape> {
        self.objects.iter().map(|a| a.to_shape()).collect()
    }
}

impl RenderMode for PulseRenderer {
    fn render(&self, frame: &mut Frame) {
        let objects = self.to_shapes();
        draw_scene(frame, &objects);
    }

    fn handle_events(&mut self, state: &AppState) {
        if self.next_is_ready() {
            self.add_pulse(state);
            self.next = Instant::now()
                .checked_add(Duration::from_secs_f64(reciprocal_decay(
                    state.wpm.into(),
                    5.0,
                    2.5,
                )))
                .unwrap();
        }
    }

    fn prune(&mut self) {
        self.objects.retain(|p| p.is_ongoing());
    }
}

impl Default for PulseRenderer {
    fn default() -> Self {
        Self::new()
    }
}
