use crate::input::{Key, KeyEvent};
use ratatui::{DefaultTerminal, Frame};
use std::{
    sync::mpsc::Receiver,
    time::{Duration, Instant},
};

pub struct App {
    rx: Receiver<KeyEvent>,
    exit: bool,
}

impl App {
    pub fn new(rx: Receiver<KeyEvent>) -> Self {
        App { rx, exit: false }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        let mut last = Instant::now();
        while !self.exit {
            if last.elapsed() >= Duration::from_micros(16) {
                for e in self.rx.try_iter() {
                    println!("{:?}", e);
                    if e.key == Key::Char('q') {
                        self.exit = true;
                    }
                }
                terminal.draw(|frame| self.draw(frame)).unwrap();
                last = Instant::now()
            }
        }
    }

    // TODO: Add logic for drawing canvas
    fn draw(&self, frame: &mut Frame) {}
}
