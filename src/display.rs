// ======================================================================================================================== //
// ----------------------------------------------------- DISPLAY ---------------------------------------------------------- //
// ======================================================================================================================== //
extern crate termion;

use termion::screen::AlternateScreen;
use termion::{color, cursor};

use std::io::{Write, Stdout, stdout};

pub struct Display {
    screen: AlternateScreen<Stdout>,
}

impl Display {
    pub fn new() -> Display {
        // Builds a new alternate screen into a Display object
        return Display { screen: AlternateScreen::from(stdout()) };
    }

    pub fn flush(&mut self) {
        (*self).screen.flush().unwrap();
    }

    pub fn write(&mut self) {
        write!((*self).screen, "BEANS!").unwrap();
    }

    pub fn clear(&mut self) {
        // Clear the alternate screen
        let screen = &mut (*self).screen;

        write!(screen, "{}", termion::clear::All).unwrap();
        screen.flush().unwrap();
    }

    pub fn size() -> std::result::Result<(u16, u16), std::io::Error> {
        // Use termion to return the current terminal window size
        return termion::terminal_size();
    }

    fn midpoints() -> (u16, u16) {
        // Calculate the x and y midpoints based on current window size
        let window = Display::size().unwrap();

        return (window.0 / 2, window.1 / 2);
    }

    pub fn title(&mut self) {
        // Generates the 'Rust.Ed' logo, and prints it in the middle of the window

        // Get information about the current size of the screen
        let mids = Display::midpoints();

        // Set colors for display
        let bl = color::Fg(color::Blue);
        let rd = color::Fg(color::Red);
        let bk = color::Fg(color::Black);

        // Get the start position for line horizontals
        let xp = mids.0 - 25;
        let yp = mids.1 - 3;

        // Get the screen
        let screen = &mut (*self).screen;

        // Write the title to the screen
        write!(screen, "{}{}██████╗ ██╗   ██╗███████╗████████╗{}███████╗██████╗\n",
               cursor::Goto(xp, yp), rd, bl).unwrap();
        write!(screen, "{}{}██╔══██╗██║   ██║██╔════╝╚══██╔══╝{}██╔════╝██╔══██╗\n",
               cursor::Goto(xp, yp+1), rd, bl).unwrap();
        write!(screen, "{}{}██████╔╝██║   ██║███████╗   ██║   {}█████╗  ██║  ██║\n",
               cursor::Goto(xp, yp+2), rd, bl).unwrap();
        write!(screen, "{}{}██╔══██╗██║   ██║╚════██║   ██║   {}██╔══╝  ██║  ██║\n",
               cursor::Goto(xp, yp+3), rd, bl).unwrap();
        write!(screen, "{}{}██║  ██║╚██████╔╝███████║   ██║{}██╗{}███████╗██████╔╝\n",
               cursor::Goto(xp, yp+4), rd, bk, bl).unwrap();
        write!(screen, "{}{}╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝{}╚═╝{}╚══════╝╚═════╝\n",
               cursor::Goto(xp, yp+5), rd, bk, bl).unwrap();

        // Flush the output (display)
        screen.flush().unwrap();
    }

    pub fn reset_cursor(&mut self) {
        // Move cursor to 1,1 and reset the foreground color to default
        let screen = &mut (*self).screen;

        write!(screen, "{}{}", cursor::Goto(1, 1), color::Fg(color::Reset)).unwrap();
        screen.flush().unwrap();
    }


}
