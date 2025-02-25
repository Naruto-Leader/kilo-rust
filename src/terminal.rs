use libc;

use std::io::{self, Read, Write};
use std::mem;
use std::str;

static mut ORIG_TERMIOS: Option<libc::termios> = None;

extern "C" fn disable_raw_mode() {
    unsafe {
        let mut termios = ORIG_TERMIOS.unwrap();
        let errno = libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &mut termios);
        if errno == -1 { panic!("tcsetattr") }
    }
}

pub fn enable_raw_mode() {
    unsafe {
        let mut termios: libc::termios = mem::zeroed();

        let errno = libc::tcgetattr(libc::STDIN_FILENO, &mut termios as *mut libc::termios);
        if errno == -1 { panic!("tcgetattr") }

        ORIG_TERMIOS = Some(termios);
        libc::atexit(disable_raw_mode);

        termios.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
        termios.c_oflag &= !libc::OPOST;
        termios.c_cflag |= libc::CS8;
        termios.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);
        termios.c_cc[libc::VMIN] = 0;
        termios.c_cc[libc::VTIME] = 1;

        let errno = libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &mut termios);
        if errno == -1 { panic!("tcsetattr") }
    }
}

fn get_cursor_position() -> Option<(u16, u16)> {
    let _ = io::stdout().write(b"\x1b[6n");
    let _ = io::stdout().flush();
    let mut buffer = [0;32];
    let _ = io::stdin().read(&mut buffer);
    let mut iter = str::from_utf8(&buffer[2..]).unwrap().split(|c| c == ';');
    let rows: u16 = iter.next().unwrap()
        .parse().unwrap();
    let cols = iter.next().unwrap()
        .split(|c| c == 'R')
        .next().unwrap()
        .parse().unwrap();
    Some((rows, cols))
}

pub fn get_window_size() -> Option<(u16, u16)> {
    unsafe {
        let mut ws: libc::winsize = mem::zeroed();
        let errno = libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws);
        if errno == -1 || ws.ws_col == 0 {
            let bytes_written = io::stdout().write(b"\x1b[999C\x1b[999B");
            if let Ok(12) = bytes_written {
                get_cursor_position()
            } else {
                None
            }
        } else {
            Some((ws.ws_row, ws.ws_col))
        }
    }
}
