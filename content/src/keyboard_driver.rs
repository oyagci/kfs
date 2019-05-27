use crate::print;
use crate::utils;
use spin::Mutex;

pub static LOCK: Mutex<u8> = Mutex::new(0x00);
pub struct Keyboard {
    state: State,
    keymap: KeyMap,
}

pub struct State {
    pub lshift: bool,
    pub rshift: bool,
    pub lctrl: bool,
    pub rctrl: bool,
    pub lmeta: bool,
    pub rmeta: bool,
}

pub struct KeyMap {
    key_array: [char; 0x5A],
    shift_key_array: [char; 0x5A],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            state: State::new(),
            keymap: KeyMap::new(),
        }
    }

    pub fn get_char(&self, val: u8) -> char {
        if self.state.lshift || self.state.rshift {
            self.keymap.shift_key_array[val as usize]
        } else {
            self.keymap.key_array[val as usize]
        }
    }

    pub fn update_state(&mut self, val: u8) -> bool {
        if val == 0x2A {
            self.state.lshift = true;
            return true;
        }
        if val == 0xAA {
            self.state.lshift = false;
            return true;
        }
        return false;
    }

    pub fn update(&mut self) -> Option<u8> {
        utils::outb(1, 0x64);
        let index = utils::inb(0x60);

        if self.update_state(index) {
            return None;
        }
        if index < 0x5A && index != *LOCK.lock() {
            *LOCK.lock() = index;
            return Some(self.get_char(index) as u8);
        }
        *LOCK.lock() = index;
        return None;
    }
}

impl State {
    pub fn new() -> State {
        State {
            lshift: false,
            rshift: false,
            lctrl: false,
            rctrl: false,
            lmeta: false,
            rmeta: false,
        }
    }
}

impl KeyMap {
    pub fn new() -> KeyMap {
        KeyMap {
            key_array: [
                ' ', ' ', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', 127 as char, ' ',
                'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n', ' ', 'a', 's',
                'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`', ' ', '\\', 'z', 'x', 'c', 'v',
                'b', 'n', 'm', ',', '.', '/', ' ', '*', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', '7', '8', '9', '-', '4', '5', '6', '+', '1',
                '2', '3', '0', '.', ' ', ' ', ' ', ' ', ' ', ' ',
            ],
            shift_key_array: [
                ' ', ' ', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', 127 as char, ' ',
                'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', '\n', ' ', 'A', 'S',
                'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '"', '~', ' ', '|', 'Z', 'X', 'C', 'V',
                'B', 'N', 'M', '<', '>', '?', ' ', '*', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', '7', '8', '9', '-', '4', '5', '6', '+', '1',
                '2', '3', '0', '.', ' ', ' ', ' ', ' ', ' ', ' ',
            ],
        }
    }
}
