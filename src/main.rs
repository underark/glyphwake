use std::panic::{set_hook, take_hook};

fn main() {
    let hook = take_hook();
    set_hook(Box::new(move |ph| {
        ratatui::restore();
        hook(ph);
    }));

    glyphwake::enter();
}
