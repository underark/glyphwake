use ratatui::{Frame, widgets::canvas::Circle};

use crate::{input::KeyEvent, render::draw_scene};

pub trait RenderMode {
    fn render(&self, frame: &mut Frame);
    fn handle_events(&mut self, e: Vec<KeyEvent>);
    fn update(&mut self);
}

pub struct PulseRenderer {
    objects: Vec<Circle>,
}

impl PulseRenderer {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    fn add(&mut self) {
        self.objects.push(Circle {
            x: 100.0,
            y: 100.0,
            radius: 5.0,
            color: ratatui::style::Color::Yellow,
        });
    }

    fn grow_radius(&mut self) {
        for o in self.objects.iter_mut() {
            o.radius += 1.0;
        }
    }

    fn prune(&mut self) {
        self.objects.retain(|o| o.radius <= 200.0);
    }
}

impl RenderMode for PulseRenderer {
    fn render(&self, frame: &mut Frame) {
        draw_scene(frame, &self.objects);
    }

    fn handle_events(&mut self, events: Vec<KeyEvent>) {
        for _ in events {
            self.add();
        }
    }

    fn update(&mut self) {
        self.grow_radius();
        self.prune();
    }
}

impl Default for PulseRenderer {
    fn default() -> Self {
        Self::new()
    }
}
