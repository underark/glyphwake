use crate::app::AppState;
use ratatui::Frame;

pub trait RenderMode {
    fn render(&self, frame: &mut Frame);
    fn handle_events(&mut self, s: &AppState);
    fn prune(&mut self);
}
