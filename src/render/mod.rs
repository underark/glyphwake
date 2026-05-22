use ratatui::Frame;
use ratatui::symbols::Marker;
use ratatui::widgets::Block;
use ratatui::widgets::canvas::{Canvas, Shape};

pub fn draw_scene(frame: &mut Frame, objects: &Vec<impl Renderable>) {
    let (w, h) = get_w_h(frame);
    let canvas = Canvas::default()
        .block(Block::new())
        .x_bounds([0.0, 200.0])
        .y_bounds([0.0, 200.0])
        .marker(Marker::Octant)
        .paint(|ctx| {
            for o in objects {
                let s = o.to_shape(100.0, 100.0);
                ctx.draw(&s);
            }
        });
    frame.render_widget(canvas, frame.area());
}

fn get_w_h(frame: &Frame) -> (f64, f64) {
    let r = frame.area();
    (r.width.into(), r.height.into())
}

pub trait Renderable {
    fn to_shape(&self, x: f64, y: f64) -> impl Shape;
}
