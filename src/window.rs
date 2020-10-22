// ======================================================================================================================== //
// ----------------------------------------------------- DISPLAY ---------------------------------------------------------- //
// ======================================================================================================================== //

/// Implements display controls
pub mod display {
    extern crate termion;

    use termion::screen::AlternateScreen;
    use termion::raw::{IntoRawMode, RawTerminal};
    use termion::cursor::{DetectCursorPos};
    use termion::{color, cursor};

    use std::io::{Write, Stdout, stdout};

    /// Screen structure for output pipe
    pub struct Screen {
        terminal: RawTerminal<AlternateScreen<Stdout>>,
    }

    /// Cursor structure that handles user movement
    pub struct Cursor {
        screen: Screen,
    }

    impl Screen {
        /// Create a new screen object
        pub fn new() -> Screen {
            return Screen { terminal: AlternateScreen::from(stdout()).into_raw_mode().unwrap() };
        }

        /// Flush output to screen
        pub fn flush(&mut self) {
            &self.terminal.flush().unwrap();
        }

        /// Completely wipe screen
        pub fn clear(&mut self) {
            write!((*self).terminal, "{}", termion::clear::All).unwrap();
            &self.flush();
        }

        /// Write a string to screen
        pub fn write(&mut self, string: &str) {
            write!((*self).terminal, "{}", string).unwrap();
        }

        /// Report screen size
        pub fn size(&mut self) -> (u16, u16) {
            return termion::terminal_size().unwrap();        
        }

        /// Return colors to default
        pub fn reset(&mut self) {
            write!((*self).terminal, "{}{}", 
                   color::Fg(color::Reset), color::Bg(color::Reset)).unwrap();
            &self.flush();
        }
    }

    /// Print the "Rust.Ed" title onto a given screen
    pub fn title(screen: &mut Screen) {
        // Generates the 'Rust.Ed' logo, and prints it in the middle of the window

        // Get information about the current size of the screen
        let size = screen.size();
        let mids = (size.0 / 2, size.1 / 2);

        // Set colors for display
        let bl = color::Fg(color::Blue);
        let rd = color::Fg(color::Red);
        let bk = color::Fg(color::Black);

        // Get the start position for line horizontals
        let xp = mids.0 - 25;
        let yp = mids.1 - 3;

        // Write the title to the screen
        let title = format!(
            "{}{}██████╗ ██╗   ██╗███████╗████████╗{}███████╗██████╗\n\
             {}{}██╔══██╗██║   ██║██╔════╝╚══██╔══╝{}██╔════╝██╔══██╗\n\
             {}{}██████╔╝██║   ██║███████╗   ██║   {}█████╗  ██║  ██║\n\
             {}{}██╔══██╗██║   ██║╚════██║   ██║   {}██╔══╝  ██║  ██║\n\
             {}{}██║  ██║╚██████╔╝███████║   ██║{}██╗{}███████╗██████╔╝\n\
             {}{}╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝{}╚═╝{}╚══════╝╚═════╝\n",
            cursor::Goto(xp, yp), rd, bl,
            cursor::Goto(xp, yp+1), rd, bl,
            cursor::Goto(xp, yp+2), rd, bl,
            cursor::Goto(xp, yp+3), rd, bl,
            cursor::Goto(xp, yp+4), rd, bk, bl,
            cursor::Goto(xp, yp+5), rd, bk, bl);

        screen.write(&title);

        // Flush the output (display)
        screen.flush();
    }
}
