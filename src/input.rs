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

        match evt {
            Event::Key(Key::Char('q')) => {
                break
            },
            Event::Key(Key::Char(c)) => {
                disp.write_char(c);
            },
            _ => { }
        }

        disp.flush();
    }
}
