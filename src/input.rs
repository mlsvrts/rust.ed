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
        _ => { },
    }

    screen.flush();

    return 0;
}
