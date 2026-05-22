use std::time::{Duration, Instant};

use crate::{
    input::KeyEvent,
    render::{Renderable, draw_scene},
};
use ratatui::{Frame, style::Color, widgets::canvas::Circle, widgets::canvas::Shape};

pub trait RenderMode {
    fn render(&self, frame: &mut Frame);
    fn handle_events(&mut self, e: &[KeyEvent]);
    fn prune(&mut self);
}

pub struct PulseRenderer {
    objects: Vec<Pulse>,
}

struct Pulse {
    birth_time: Instant,
    duration: u8,
}

impl PulseRenderer {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    fn add_pulse(&mut self, birth_time: Instant, duration: u8) {
        self.objects.push(Pulse {
            birth_time,
            duration,
        });
    }
}

impl RenderMode for PulseRenderer {
    fn render(&self, frame: &mut Frame) {
        draw_scene(frame, &self.objects);
    }

    fn handle_events(&mut self, events: &[KeyEvent]) {
        for _ in events {
            self.add_pulse(Instant::now(), 5);
        }
    }

    fn prune(&mut self) {
        self.objects.retain(|p| {
            let elapsed = p.birth_time.elapsed();
            let duration = Duration::from_secs(p.duration.into());
            elapsed.saturating_sub(duration) == Duration::ZERO
        });
    }
}

impl Renderable for Pulse {
    fn to_shape(&self, x: f64, y: f64) -> impl Shape {
        Circle {
            x,
            y,
            radius: self.ease_out_circ() * 150.0,
            color: Color::Red,
        }
    }
}

impl Pulse {
    fn ease_out_circ(&self) -> f64 {
        let elapsed = self.birth_time.elapsed();
        let duration = Duration::from_secs(self.duration.into());
        let normalized = elapsed.div_duration_f64(duration);
        (1.0 - (normalized - 1.0).powi(2)).sqrt()
    }
}

impl Default for PulseRenderer {
    fn default() -> Self {
        Self::new()
    }
}
