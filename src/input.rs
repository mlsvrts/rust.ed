// ======================================================================================================================== //
// ------------------------------------------------------ INPUT ----------------------------------------------------------- //
// ======================================================================================================================== //
extern crate termion;

use termion::event::{Key, Event};
use termion::input::TermRead;

use crate::window::display;

use std::io::stdin;

pub fn capture(screen: &mut display::Screen) {
    let stdin = stdin();

    for op in stdin.events() {
        let evt = op.unwrap();

        // Key Handler
        match evt {
            Event::Key(Key::Ctrl('c')) => {
                break
            },
            _ => { },
        }

        screen.flush();
    }
}
