use ratatui::Frame;
use ratatui::symbols::Marker;
use ratatui::widgets::Block;
use ratatui::widgets::canvas::{Canvas, Shape};

pub fn draw_scene(frame: &mut Frame, objects: &Vec<impl Renderable>) {
    let canvas = Canvas::default()
        .block(Block::new())
        .x_bounds([0.0, 800.0])
        .y_bounds([0.0, 800.0])
        .marker(Marker::Octant)
        .paint(|ctx| {
            for o in objects {
                let s = o.to_shape(400.0, 400.0);
                ctx.draw(&s);
            }
        });
    frame.render_widget(canvas, frame.area());
}

pub trait Renderable {
    fn to_shape(&self, x: f64, y: f64) -> impl Shape;
    fn normalize(&self) -> f64;
}
