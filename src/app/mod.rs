use crate::{
    input::{Key, KeyEvent},
    render::Renderer,
    state::AppState,
};
use ratatui::{DefaultTerminal, Frame};
use std::{
    sync::mpsc::Receiver,
    time::{Duration, Instant},
};

pub struct App {
    rx: Receiver<KeyEvent>,
    exit: bool,
    state: AppState,
    renderer: Renderer,
}

impl App {
    pub fn new(rx: Receiver<KeyEvent>) -> Self {
        App {
            rx,
            exit: false,
            state: AppState::default(),
            renderer: Renderer::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        let mut last = Instant::now();
        while !self.exit {
            if last.elapsed() >= Duration::from_millis(16) {
                let events: Vec<KeyEvent> = self.rx.try_iter().collect();
                self.handle_events(events);
                self.state.update();
                terminal.draw(|frame| self.draw(frame)).unwrap();
                last = Instant::now()
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        self.renderer.draw_scene(frame);
    }

    fn handle_events(&mut self, events: Vec<KeyEvent>) {
        for e in events {
            if e.key == Key::Char('q') {
                self.exit = true;
            }
            self.new_object();
        }
    }

    fn new_object(&mut self) {
        self.state.add();
    }
}
