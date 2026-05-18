pub mod app;
pub mod input;
pub mod render;
pub mod state;

use crate::app::App;
use crate::input::XInputListener;

pub fn enter() {
    let listener = XInputListener::start_input_listener();
    let mut app = App::new(listener);
    ratatui::run(|terminal| app.run(terminal));
}
