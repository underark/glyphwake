use crate::{app::App, mode::RenderMode};

pub mod animation;
pub mod app;
pub mod backend;
pub mod input;
pub mod math;
pub mod mode;
pub mod render;

pub fn enter(mode: impl RenderMode) {
    let mut app = App::new(mode);
    ratatui::run(|terminal| app.run(terminal));
}
