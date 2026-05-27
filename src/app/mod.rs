use crate::backend::pulse::PulseRenderer;
use crate::input::{Key, KeyEvent};
use crate::mode::RenderMode;
use ratatui::{DefaultTerminal, Frame};
use std::{
    sync::mpsc::Receiver,
    time::{Duration, Instant},
};

pub struct App<R: RenderMode> {
    rx: Receiver<KeyEvent>,
    exit: bool,
    events: Vec<KeyEvent>,
    mode: R,
}

#[derive(Clone, Copy, Debug)]
pub struct AppState {
    pub wpm: u32,
}

// TODO: Move this implementation to the mode module under impl PulseRenderer
impl App<PulseRenderer> {
    pub fn new_pulse(rx: Receiver<KeyEvent>) -> Self {
        App {
            rx,
            exit: false,
            events: vec![],
            mode: PulseRenderer::default(),
        }
    }
}

impl<R: RenderMode> App<R> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        let mut last = Instant::now();
        while !self.exit {
            if last.elapsed() >= Duration::from_secs(1) {
                let events: Vec<KeyEvent> = self.rx.try_iter().collect();
                self.check_keys(&events);
                self.store_event(&events);
                let state = self.extract_state();
                self.mode.handle_events(&state);
                last = Instant::now()
            }
            self.mode.prune();
            self.prune();
            terminal.draw(|frame| self.draw(frame)).unwrap();
        }
    }

    fn draw(&self, frame: &mut Frame) {
        self.mode.render(frame);
    }

    fn check_keys(&mut self, events: &[KeyEvent]) {
        for e in events {
            if e.key == Key::Char('q') {
                self.exit = true;
            }
        }
    }

    fn store_event(&mut self, events: &[KeyEvent]) {
        self.events.extend_from_slice(events);
    }

    fn extract_state(&self) -> AppState {
        let len = self.events.len() as u32;
        let wpm = (len / 5) * (60 / 10);
        AppState { wpm }
    }

    fn prune(&mut self) {
        self.events
            .retain(|e| e.time.elapsed() <= Duration::from_secs(5));
    }
}
