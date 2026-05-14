use std::sync::mpsc;
use std::thread;

use x11rb::protocol::Event;
use x11rb::protocol::xinput::{Device, DeviceId, EventMask, XIEventMask, xi_select_events};
use x11rb::{connection::Connection, rust_connection::RustConnection};

pub struct XInputListener {
    conn: RustConnection,
    root: u32,
}

impl XInputListener {
    fn new() -> Self {
        let (conn, num) = x11rb::connect(None).unwrap();
        let screen = &conn.setup().roots[num];
        let root = screen.root;
        let listener = Self { conn, root };
        listener.init();
        listener
    }

    fn init(&self) {
        let mask = EventMask {
            deviceid: DeviceId::from(Device::ALL),
            mask: vec![XIEventMask::KEY_PRESS],
        };
        xi_select_events(&self.conn, self.root, &[mask]).unwrap();
        self.conn.flush().unwrap();
    }

    pub fn start_input_listener() -> mpsc::Receiver<u32> {
        let listener = XInputListener::new();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            loop {
                let event = listener.conn.wait_for_event().unwrap();
                if let Event::XinputKeyPress(e) = event {
                    tx.send(e.detail).unwrap();
                }
            }
        });
        rx
    }
}
