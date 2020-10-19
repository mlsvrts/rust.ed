// ======================================================================================================================== //
// ------------------------------------------------------ INPUT ----------------------------------------------------------- //
// ======================================================================================================================== //
extern crate termion;

use termion::event::{Key, Event};
use termion::input::TermRead;

use crate::display::Display;

use std::io::stdin;

pub fn capture(disp: &mut Display) {
    let stdin = stdin();

    for op in stdin.events() {
        let evt = op.unwrap();

        // Key Handler
        match evt {
            Event::Key(Key::Char('\n')) => {
                // Handle enter/newlines
                disp.write("\x0D\x0A");  // \r\n
            },
            Event::Key(Key::Char(c)) => {
                // Just write the character
                disp.write_char(c);
            },
            Event::Key(Key::Backspace) => {
                // Move back one char, delete on char
                disp.write("\x08\x1B[1P");
            },
            Event::Key(Key::Up) => {
                disp.move_n('u', 1);
            },
            Event::Key(Key::Down) => {
                disp.move_n('d', 1);
            },
            Event::Key(Key::Left) => {
                disp.move_n('l', 1);
            },
            Event::Key(Key::Right) => {
                disp.move_n('r', 1);
            },
            Event::Key(Key::Ctrl('c')) => {
                break
            },
            _ => { },
        }

        disp.flush();
    }
}
