use std::sync::mpsc;
use std::thread;

use x11rb::protocol::Event;
use x11rb::protocol::xinput::{Device, DeviceId, EventMask, XIEventMask, xi_select_events};
use x11rb::protocol::xproto::ConnectionExt;
use x11rb::{connection::Connection, rust_connection::RustConnection};
use xkeysym::{KeyCode, Keysym, keysym};

pub struct XInputListener {
    conn: RustConnection,
    root: u32,
    kbd: KbdData,
}

struct KbdData {
    first_key: u8,
    max_key: u8,
    keysyms: Vec<u32>,
    keysyms_per: u8,
}

#[derive(Debug)]
pub struct KeyEvent {
    pub key_char: char,
}

// TODO:
// enum Key {}

impl XInputListener {
    fn new() -> Self {
        let (conn, num) = x11rb::connect(None).unwrap();
        let setup = conn.setup();
        let screen = &setup.roots[num];
        let root = screen.root;
        let kbd = KbdData {
            first_key: setup.min_keycode,
            max_key: setup.max_keycode,
            keysyms: vec![],
            keysyms_per: 0,
        };
        let mut listener = Self { conn, root, kbd };
        listener.init();
        listener
    }

    fn init(&mut self) {
        let mask = EventMask {
            deviceid: DeviceId::from(Device::ALL),
            mask: vec![XIEventMask::KEY_PRESS],
        };
        xi_select_events(&self.conn, self.root, &[mask]).unwrap();
        self.get_kbd();
        self.conn.flush().unwrap();
    }

    fn get_kbd(&mut self) {
        let count = self.kbd.max_key - self.kbd.first_key + 1;
        let reply = self
            .conn
            .get_keyboard_mapping(self.kbd.first_key, count)
            .unwrap()
            .reply_unchecked()
            .unwrap()
            .unwrap();
        self.kbd.keysyms = reply.keysyms;
        self.kbd.keysyms_per = reply.keysyms_per_keycode;
    }

    pub fn start_input_listener() -> mpsc::Receiver<KeyEvent> {
        let listener = XInputListener::new();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            loop {
                let event = listener.conn.wait_for_event().unwrap();
                if let Event::XinputKeyPress(e) = event {
                    let keysym = keycode_to_key(
                        e.detail,
                        u32::from(listener.kbd.first_key),
                        listener.kbd.keysyms_per,
                        &listener.kbd.keysyms,
                    )
                    .unwrap_or_else(|| Keysym::from(0));

                    // TODO: get mod keys and use that in keycode_to_key
                    let key_event = KeyEvent {
                        key_char: keysym.key_char().unwrap_or('a'),
                    };
                    tx.send(key_event).unwrap();
                }
            }
        });
        rx
    }
}

pub fn keycode_to_key(detail: u32, min: u32, per: u8, keysyms: &[u32]) -> Option<Keysym> {
    let keycode = KeyCode::new(detail);
    let min_key = KeyCode::new(min);
    keysym(keycode, 0, min_key, per, keysyms)
}
