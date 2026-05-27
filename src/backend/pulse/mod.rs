// TODO: look into using clamp to apply max values
use crate::app::AppState;
use crate::math::ease_out_circ;
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
    objects: Vec<Pulse>,
}

#[derive(Debug)]
struct Pulse {
    birth_time: Instant,
    duration: f64,
}

impl Renderable for Pulse {
    fn to_shape(&self, x: f64, y: f64) -> impl Shape {
        Circle {
            x,
            y,
            radius: ease_out_circ(self.normalize()) * 400.0,
            color: Color::Red,
        }
    }

    fn normalize(&self) -> f64 {
        let elapsed = self.birth_time.elapsed();
        let duration = Duration::from_secs_f64(self.duration);
        elapsed.div_duration_f64(duration)
    }
}

impl Pulse {
    fn from(s: &AppState) -> Pulse {
        let duration = reciprocal_decay(s.wpm.into());
        Pulse {
            birth_time: Instant::now(),
            duration,
        }
    }
}

impl PulseRenderer {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    fn add_pulse(&mut self, s: &AppState) {
        let p = Pulse::from(s);
        self.objects.push(p);
    }
}

impl RenderMode for PulseRenderer {
    fn render(&self, frame: &mut Frame) {
        draw_scene(frame, &self.objects);
    }

    fn handle_events(&mut self, state: &AppState) {
        self.add_pulse(state);
    }

    fn prune(&mut self) {
        self.objects.retain(|p| {
            let elapsed = p.birth_time.elapsed();
            let duration = Duration::from_secs_f64(p.duration);
            elapsed.saturating_sub(duration) == Duration::ZERO
        });
    }
}

impl Default for PulseRenderer {
    fn default() -> Self {
        Self::new()
    }
}
