// TODO: look into using clamp to apply max values
use crate::app::AppState;
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
    duration: u64,
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
            let duration = Duration::from_secs(p.duration);
            elapsed.saturating_sub(duration) == Duration::ZERO
        });
    }
}

impl Renderable for Pulse {
    fn to_shape(&self, x: f64, y: f64) -> impl Shape {
        Circle {
            x,
            y,
            radius: self.ease_out_circ() * 400.0,
            color: Color::Red,
        }
    }
}

impl Pulse {
    fn ease_out_circ(&self) -> f64 {
        let elapsed = self.birth_time.elapsed();
        let duration = Duration::from_secs(self.duration);
        let normalized = elapsed.div_duration_f64(duration);
        (1.0 - (normalized - 1.0).powi(2)).sqrt()
    }

    fn from(s: &AppState) -> Pulse {
        let duration = 10 / (1 + s.wpm / 50);
        Pulse {
            birth_time: Instant::now(),
            duration: duration.into(),
        }
    }
}

impl Default for PulseRenderer {
    fn default() -> Self {
        Self::new()
    }
}
