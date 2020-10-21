// ======================================================================================================================== //
// ----------------------------------------------------- DISPLAY ---------------------------------------------------------- //
// ======================================================================================================================== //
extern crate termion;

use termion::screen::AlternateScreen;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, cursor, cursor::DetectCursorPos};

use std::io::{Write, Stdout, stdout};

pub struct Display {
    screen: RawTerminal<AlternateScreen<Stdout>>,
}

impl Display {
    pub fn new() -> Display {
        // Builds a new alternate screen into a Display object
        return Display { screen: AlternateScreen::from(stdout()).into_raw_mode().unwrap() };
    }

    pub fn flush(&mut self) {
        (*self).screen.flush().unwrap();
    }

    pub fn write(&mut self, string: &str) {
        write!((*self).screen, "{}", string).unwrap();
    }

    pub fn write_char(&mut self, c: char) {
        // Insert a blank character and overwrite it
        write!((*self).screen, "\x1B[1@{}", c).unwrap();

        // Fix EoL
        &self.redraw_eol();
    }

    pub fn redraw_eol(&mut self) {
        // Write end border
        let pos = &self.screen.cursor_pos().unwrap();
        let siz = Display::size().unwrap();
        write!((*self).screen, "{}{}│{}{}",
            cursor::Goto(siz.0, pos.1),
            color::Fg(color::Green),
            cursor::Goto(pos.0, pos.1),
            color::Fg(color::Reset)
        ).unwrap();
    }

    pub fn clear_eol(&mut self) {
        // Write end border
        let pos = &self.screen.cursor_pos().unwrap();
        let siz = Display::size().unwrap();
        write!((*self).screen, "{}{}\x1B[1@{}{}",
            cursor::Goto(siz.0, pos.1),
            color::Fg(color::Green),
            cursor::Goto(pos.0, pos.1),
            color::Fg(color::Reset)
        ).unwrap();
    }

    pub fn backspace(&mut self) {
        let pos = &self.screen.cursor_pos().unwrap();

        if pos.0 != 2 {
            &self.clear_eol();
            &self.write("\x08\x1B[1P");
            &self.redraw_eol();
        } else if pos.1 != 2 {
            // go to end of previous line
        }
    }

    pub fn newline(&mut self) {
        &self.write("\x0D\x0A");
        &self.move_n('r', 1);
    }

    pub fn move_n(&mut self, d: char, n: u16) {
        // Detect cursor positon to protect borders
        let pos = &self.screen.cursor_pos().unwrap();
        let siz = Display::size().unwrap();

        // Direction specific handlers
        if d == 'u' && pos.1 != 2 {
            write!((*self).screen, "{}", cursor::Up(n)).unwrap();
        } else if d == 'l' && pos.0 != 2 {
            write!((*self).screen, "{}", cursor::Left(n)).unwrap();
        } else if d == 'd' && pos.1 != (siz.1 - 1) {
            write!((*self).screen, "{}", cursor::Down(n)).unwrap();
        } else if d == 'r' && pos.0 != (siz.0 - 1) {
            write!((*self).screen, "{}", cursor::Right(n)).unwrap();
        }
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

    pub fn border(&mut self) {
        // Prints the rust.ed border around the screen
        
        // Get screen object and border color
        let gr = color::Fg(color::Green);
        let screen = &mut (*self).screen;
        let window = Display::size().unwrap();

        // Top
        write!(screen, "{}{}┌{}┐", cursor::Goto(1,1), gr, 
               "─".repeat((window.0 - 2) as usize)).unwrap();

        // Left, Right
        for i in 2..window.1 {
            write!(screen, "{}{}│", cursor::Goto(1,i), gr).unwrap();
            write!(screen, "{}{}│", cursor::Goto(window.0,i), gr).unwrap();
        }

        // Bottom
        write!(screen, "{}{}└{}┘", cursor::Goto(1,window.1), gr, 
               "─".repeat((window.0 - 2) as usize)).unwrap();
    }

    pub fn reset_cursor(&mut self) {
        // Move cursor to 1,1 and reset the foreground color to default
        let screen = &mut (*self).screen;

        write!(screen, "{}{}", cursor::Goto(2, 2), color::Fg(color::Reset)).unwrap();
        screen.flush().unwrap();
    }


}
