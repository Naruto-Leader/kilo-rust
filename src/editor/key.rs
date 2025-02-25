pub enum Key {
    Character(char),
    Control(char),
    Arrow(ArrowKey),
    Escape,
    Backspace,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
}

pub enum ArrowKey {
    Left,
    Right,
    Up,
    Down,
}

impl Key {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0       => None,
            b'\x1b' => Some(Key::Escape),
            8 | 127 => Some(Key::Backspace),
            1...31  => Some(Key::Control((byte | 0x40) as char)),
            _       => Some(Key::Character(byte as char))
        }
    }

    pub fn from_escape_sequence(bytes: &[u8]) -> Self {
        match bytes {
            b"[A" => Key::Arrow(ArrowKey::Up),
            b"[B" => Key::Arrow(ArrowKey::Down),
            b"[C" => Key::Arrow(ArrowKey::Right),
            b"[D" => Key::Arrow(ArrowKey::Left),
            b"[3~"  => Key::Delete,
            b"[1~" | b"[7~" | b"[H" | b"OH" => Key::Home,
            b"[4~" | b"[8~" | b"[F" | b"OF" => Key::End,
            b"[5~" => Key::PageUp,
            b"[6~" => Key::PageDown,
            _      => Key::Escape,
        }
    }
}
