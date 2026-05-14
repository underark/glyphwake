use crate::input::KeyEvent;
use ratatui::{DefaultTerminal, Frame};
use std::sync::mpsc::Receiver;

pub struct App {
    rx: Receiver<KeyEvent>,
    exit: bool,
}

impl App {
    pub fn new(rx: Receiver<KeyEvent>) -> Self {
        App { rx, exit: false }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        while !self.exit {
            for e in self.rx.try_iter() {
                println!("{:?}", e);
                if e.key_char == 'q' {
                    self.exit = true;
                }
            }
            terminal.draw(|frame| self.draw(frame)).unwrap();
        }
    }

    fn draw(&self, frame: &mut Frame) {}
}
