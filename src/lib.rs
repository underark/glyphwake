pub mod app;
pub mod backend;
pub mod input;
pub mod mode;
pub mod render;

use crate::app::App;
use crate::input::XInputListener;

pub fn enter() {
    let listener = XInputListener::start_input_listener();
    let mut app = App::new_pulse(listener);
    ratatui::run(|terminal| app.run(terminal));
}
