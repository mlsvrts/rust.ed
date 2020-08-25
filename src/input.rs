// ======================================================================================================================== //
// ------------------------------------------------------ INPUT ----------------------------------------------------------- //
// ======================================================================================================================== //
extern crate termion;

use termion::event::{Key, Event};
use termion::input::TermRead;
// use termion::input::MouseTerminal;
// use termion::raw::IntoRawMode;

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
            _ => {
                disp.write();
            },
        }

        disp.flush();
    }
}
