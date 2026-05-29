use std::panic::{set_hook, take_hook};

use glyphwake::backend::pulse::PulseRenderer;

fn main() {
    let hook = take_hook();
    set_hook(Box::new(move |ph| {
        ratatui::restore();
        hook(ph);
    }));

    let mode = PulseRenderer::new();
    glyphwake::enter(mode);
}
