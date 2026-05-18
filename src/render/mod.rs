use ratatui::Frame;
use ratatui::symbols::Marker;
use ratatui::widgets::Block;
use ratatui::widgets::canvas::{Canvas, Shape};

pub struct Renderer {}

impl Renderer {
    fn new() -> Self {
        Renderer {}
    }

    pub fn draw_scene(&self, frame: &mut Frame, objects: &Vec<impl Shape>) {
        let canvas = Canvas::default()
            .block(Block::new())
            .x_bounds([0.0, 180.0])
            .y_bounds([0.0, 180.0])
            .marker(Marker::Braille)
            .paint(|ctx| {
                for o in objects {
                    ctx.draw(o);
                }
            });
        frame.render_widget(canvas, frame.area());
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Renderer::new()
    }
}
