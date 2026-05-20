use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use x11rb::protocol::Event;
use x11rb::protocol::xinput::{Device, DeviceId, EventMask, XIEventMask, xi_select_events};
use x11rb::protocol::xproto::ConnectionExt;
use x11rb::{connection::Connection, rust_connection::RustConnection};
use xkeysym::{KeyCode, Keysym, key, keysym};

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

#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
    pub key: Key,
    pub time: Instant,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Key {
    Char(char),
    Shift,
    Ctrl,
    Alt,
    Unknown,
}

impl Key {
    fn from_keysym(value: Option<Keysym>) -> Key {
        let Some(k) = value else { return Key::Unknown };

        if let Some(c) = k.key_char() {
            Key::Char(c)
        } else {
            match k.raw() {
                key::Shift_L | key::Shift_R => Key::Shift,
                key::Control_L | key::Control_R => Key::Ctrl,
                key::Alt_L | key::Alt_R => Key::Alt,
                _ => Key::Unknown,
            }
        }
    }
}

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
                        e.mods.base.try_into().unwrap(),
                        u32::from(listener.kbd.first_key),
                        listener.kbd.keysyms_per,
                        &listener.kbd.keysyms,
                    );

                    let k = Key::from_keysym(keysym);
                    tx.send(KeyEvent {
                        key: k,
                        time: Instant::now(),
                    })
                    .unwrap();
                }
            }
        });
        rx
    }
}

pub fn keycode_to_key(detail: u32, col: u8, min: u32, per: u8, keysyms: &[u32]) -> Option<Keysym> {
    let keycode = KeyCode::new(detail);
    let min_key = KeyCode::new(min);
    keysym(keycode, col, min_key, per, keysyms)
}
