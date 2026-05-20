use crate::input::{Key, KeyEvent};
use crate::mode::{PulseRenderer, RenderMode};
use ratatui::{DefaultTerminal, Frame};
use std::{
    sync::mpsc::Receiver,
    time::{Duration, Instant},
};

pub struct App<R: RenderMode> {
    rx: Receiver<KeyEvent>,
    exit: bool,
    mode: R,
}

// TODO: Move this implementation to the mode module under impl PulseRenderer
impl App<PulseRenderer> {
    pub fn new_pulse(rx: Receiver<KeyEvent>) -> Self {
        App {
            rx,
            exit: false,
            mode: PulseRenderer::default(),
        }
    }
}

impl<R: RenderMode> App<R> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        let mut last = Instant::now();
        while !self.exit {
            if last.elapsed() >= Duration::from_millis(16) {
                let events: Vec<KeyEvent> = self.rx.try_iter().collect();
                self.check_keys(&events);
                self.mode.handle_events(events);
                self.mode.update();
                terminal.draw(|frame| self.draw(frame)).unwrap();
                last = Instant::now()
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        self.mode.render(frame);
    }

    fn check_keys(&mut self, events: &Vec<KeyEvent>) {
        for e in events {
            if e.key == Key::Char('q') {
                self.exit = true;
            }
        }
    }
}
