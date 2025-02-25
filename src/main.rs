extern crate libc;

mod editor;
mod terminal;
mod util;

use std::env;

fn main() {
    terminal::enable_raw_mode();
    let mut editor = editor::Editor::new();

    let mut args = env::args();
    if args.len() >= 2 {
        let filename = args.nth(1).unwrap();
        editor.open_file(&filename);
    }

    editor.set_status_message("HELP: Ctrl-S = save | Ctrl-Q = quit | Ctrl-F = find");

    loop {
        editor.refresh_screen();
        editor.process_keypress();
    }
}
