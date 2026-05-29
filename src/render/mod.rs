use ratatui::Frame;
use ratatui::symbols::Marker;
use ratatui::widgets::Block;
use ratatui::widgets::canvas::{Canvas, Shape};

// TODO: Remove this module entirely and move rendering logic locally to the render mode
pub fn draw_scene(frame: &mut Frame, objects: &[impl Shape]) {
    let canvas = Canvas::default()
        .block(Block::new())
        .x_bounds([0.0, 800.0])
        .y_bounds([0.0, 800.0])
        .marker(Marker::Octant)
        .paint(|ctx| {
            for o in objects {
                ctx.draw(o);
            }
        });
    frame.render_widget(canvas, frame.area());
}

pub trait Renderable {
    fn to_shape(&self, x: f64) -> impl Shape;
}
