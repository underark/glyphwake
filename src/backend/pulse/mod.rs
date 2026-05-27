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
    objects: Vec<Pulse>,
    next: Instant,
}

#[derive(Debug)]
struct Pulse {
    birth_time: Instant,
    duration: f64,
    max_radius: f64,
}

impl Renderable for Pulse {
    fn to_shape(&self, x: f64, y: f64) -> impl Shape {
        Circle {
            x,
            y,
            radius: ease_out_circ(self.normalize()) * self.max_radius,
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
        let duration = reciprocal_decay(s.wpm.into(), 10.0, 5.0);
        let max_radius = quadratic(s.wpm.into()).clamp(100.0, 400.0);
        Pulse {
            birth_time: Instant::now(),
            duration,
            max_radius,
        }
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
        self.objects.push(p);
    }

    fn next_is_ready(&self) -> bool {
        self.next.elapsed() > Duration::from_secs_f64(0.0)
    }
}

impl RenderMode for PulseRenderer {
    fn render(&self, frame: &mut Frame) {
        draw_scene(frame, &self.objects);
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
