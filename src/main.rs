use std::{panic::set_hook, process::exit};

fn main() {
    set_hook(Box::new(|ph| {
        ratatui::restore();
        println!(
            "Panic occured at {:?}: {:?}",
            ph.location().unwrap(),
            // There is a compatibility issue with older (pre-2018 Rust) code here -- consider
            // fixing
            ph.payload().downcast_ref::<&str>()
        );
        exit(1);
    }));

    glyphwake::enter();
}
