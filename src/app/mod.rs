use crate::input::{Key, KeyEvent};
use crate::mode::{PulseRenderer, RenderMode};
use ratatui::{DefaultTerminal, Frame};
use std::thread::sleep;
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

#[derive(Clone, Copy)]
pub struct AppState {
    pub wpm: u64,
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
            if last.elapsed() >= Duration::from_millis(16) {
                let events: Vec<KeyEvent> = self.rx.try_iter().collect();
                self.check_keys(&events);
                self.store_event(&events);
                self.mode.handle_events(&events);
                self.mode.prune();
                terminal.draw(|frame| self.draw(frame)).unwrap();
                last = Instant::now()
            }
            sleep(Duration::from_millis(2));
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
}
