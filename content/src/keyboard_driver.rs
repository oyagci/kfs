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

    pub fn update(&mut self) {
        utils::outb(1, 0x64);
        let index = utils::inb(0x60);

        if self.update_state(index) {
            return;
        }
        if index < 0x5A && index != *LOCK.lock() {
            print!("{}", self.get_char(index));
        }
        *LOCK.lock() = index;
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

//    #[allow(dead_code)]
//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//#[repr(u8)]
//    pub enum KeyCode {
//        KeyEscape = 0x01,
//        Key1 = 0x02,
//        Key2 = 0x03,
//        Key3 = 0x04,
//        Key4 = 0x05,
//        Key5 = 0x06,
//        Key6 = 0x07,
//        Key7 = 0x08,
//        Key8 = 0x09,
//        Key9 = 0x0A,
//        Key0 = 0x0B,
//        KeyDash = 0x0C,
//        KeyEquals = 0x0D,
//        KeyBackspace = 0x0E,
//        KeyTab = 0x0,
//        KeyQ = 0x10,
//        KeyW = 0x11,
//        KeyE = 0x12,
//        KeyR = 0x13,
//        KeyT = 0x14,
//        KeyY = 0x15,
//        KeyU = 0x16,
//        KeyI = 0x17,
//        KeyO = 0x18,
//        KeyP = 0x19,
//        KeyOpenSquareBracket = 0x1A,
//        KeyCloseSquareBracket = 0x1B,
//        KeyEnter = 0x1C,
//        KeyLeftControl = 0x1D,
//        KeyA = 0x1E,
//        KeyS = 0x1F,
//        KeyD = 0x20,
//        KeyF = 0x21,
//        KeyG = 0x22,
//        KeyH = 0x23,
//        KeyJ = 0x24,
//        KeyK = 0x25,
//        KeyL = 0x26,
//        KeySemiCollumn = 0x27,
//        KeySingleQuote = 0x28,
//        KeyBackTick = 0x29,
//        KeyLeftShift = 0x2A,
//        KeyBackSlash = 0x2B,
//        KeyZ = 0x2C,
//        KeyX = 0x2D,
//        KeyC = 0x2E,
//        KeyV = 0x2F,
//        KeyB = 0x30,
//        KeyN = 0x31,
//        KeyM = 0x32,
//        KeyComma = 0x33,
//        KeyDot = 0x34,
//        KeySlash = 0x35,
//        KeyRightShift = 0x36,
//        KeyStartKeypad = 0x37,
//        KeyLeftAlt = 0x38,
//        Keyspace = 0x39,
//        KeyCapsLock = 0x3A,
//        KeyF1 = 0x3B,
//        KeyF2 = 0x3C,
//        KeyF3 = 0x3D,
//        KeyF4 = 0x3E,
//        KeyF5 = 0x3F,
//        KeyF6 = 0x40,
//        KeyF7 = 0x41,
//        KeyF8 = 0x42,
//        KeyF9 = 0x43,
//        KeyF10 = 0x44,
//        KeyNumberLock = 0x45,
//        KeyScrollLock = 0x46,
//        Keykeypad = 0x47,
//        KeyKeypad8 = 0x48,
//        KeyKeypad9 = 0x49,
//        KeyKeypadDash = 0x4A,
//        KeyKeypad4 = 0x4B,
//        KeyKeypad5 = 0x4C,
//        KeyKeypad6 = 0x4D,
//        KeyKeypadPlus = 0x4E,
//        KeyKeypad1 = 0x4F,
//        KeyKeypad2 = 0x50,
//        KeyKeypad3 = 0x51,
//        KeyKeypad0 = 0x52,
//        KeyKeypadDot = 0x53,
//
//        KeyF11 = 0x57,
//        KeyF12 = 0x58,
//    }
//}
