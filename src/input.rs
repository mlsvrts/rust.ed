// ======================================================================================================================== //
// ------------------------------------------------------ INPUT ----------------------------------------------------------- //
// ======================================================================================================================== //

use termion::event::{Key, Event};
use crate::window::display;

pub fn capture(screen: &mut display::Screen, event: termion::event::Event) -> u8 {
    // Key Handler
    match event {
        Event::Key(Key::Ctrl('c')) => {
            return 1;
        },
        Event::Key(Key::Char(c)) => {
            screen.write(&format!("{}", c));
        }
        _ => { },
    }

    screen.flush();

    return 0;
}
